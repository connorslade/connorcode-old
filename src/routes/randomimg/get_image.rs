use std::io::Read;

use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

use ureq;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/randomimg/image.png", |_req| {
        for _ in 1..5 {
            match get_random_image() {
                Some(i) => {
                    return Response::new()
                        .bytes(i)
                        .header(Header::new("Content-Type", "image/png"))
                }
                None => {}
            }
        }

        Response::new().status(404).text("Error Getting Image")
    });
}

fn get_random_image() -> Option<Vec<u8>> {
    let chars: String = (1..6)
        .map(|_| thread_rng().sample(Alphanumeric) as char)
        .collect();

    let agent = ureq::Agent::new();
    let i = agent
        .get(&format!("https://prnt.sc/{}", chars.to_lowercase()))
        .call()
        .ok()?
        .into_string()
        .ok()?;

    let j = extract_image_url(i)?;

    // Check if is a screenshot was removed message
    if j == "//st.prntscr.com/2021/10/22/2139/img/0_173a7b_211be8ff.png" {
        return None;
    }

    let mut buf = Vec::new();
    agent
        .get(&j)
        .call()
        .unwrap()
        .into_reader()
        .read_to_end(&mut buf)
        .unwrap();

    Some(buf)
}

fn extract_image_url(body: String) -> Option<String> {
    Some(
        body.split(r#"<img class="no-click screenshot-image" src=""#)
            .nth(1)?
            .split(r#"" crossorigin="anonymous""#)
            .next()?
            .to_string(),
    )
}
