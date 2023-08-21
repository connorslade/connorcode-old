use std::io::Read;
use std::time::Duration;

use afire::Method;
use afire::Server;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use ureq::AgentBuilder;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/randomimg/image.png", |ctx| {
        // Try to find a random image 5 times
        for _ in 1..5 {
            if let Some((i, j)) = get_random_image() {
                ctx.stream(i)
                    .header("Content-Type", "image/png")
                    .header("X-Image-Id", j)
                    .send()?;
                return Ok(());
            }
        }

        ctx.status(404).text("Error Getting Image").send()?;
        Ok(())
    });
}

/// Try to get a random Lightshot Image
fn get_random_image() -> Option<(Box<dyn Read + Send + Sync + 'static>, String)> {
    // Gen Posable Lightshot Image ID
    let chars: String = (0..6)
        .map(|_| thread_rng().sample(Alphanumeric) as char)
        .collect::<String>()
        .to_lowercase();

    let agent = AgentBuilder::new().timeout(Duration::from_secs(1)).build();

    // Get Image page
    let i = agent
        .get(&format!("https://prnt.sc/{}", chars))
        .call()
        .ok()?
        .into_string()
        .ok()?;

    // Try to extract image url
    let j = extract_image_url(i)?;

    // Check if is a screenshot that was removed
    if j == "//st.prntscr.com/2021/10/22/2139/img/0_173a7b_211be8ff.png" {
        return None;
    }

    // Load Screenshot
    let reader = agent.get(&j).call().ok()?.into_reader();

    // Respond with Screenshot Bytes
    Some((reader, chars))
}

/// Extract Image Url from Lightshot Page
fn extract_image_url(body: String) -> Option<String> {
    Some(
        body.split(r#"<img class="no-click screenshot-image" src=""#)
            .nth(1)?
            .split(r#"" crossorigin="anonymous""#)
            .next()?
            .to_string(),
    )
}
