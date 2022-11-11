use std::fs;

use afire::{Method, Response, Server};

use crate::{app::App, serve_static::not_found};

lazy_static! {
    // TODO: Use config path from env vars
    static ref LINKS: Vec<[String; 2]> =
        load_links("data/config/link.cfg").expect("Error Loading Links");
}

pub fn attach(server: &mut Server<App>) {
    lazy_static::initialize(&LINKS);

    server.route(Method::GET, "/r/{code}", |req| {
        let code = req.path_param("code").unwrap();

        let link = match LINKS.iter().find(|x| x[0].to_lowercase() == code) {
            Some(i) => i,
            None => return not_found(&req.path),
        };

        Response::new()
            .status(308)
            .reason("Permanent Redirect")
            .text(format!(r#"<a href={link}>{link}</a>"#, link = &link[1]))
            .header("Content-Type", "text/html")
            .header("Location", &link[1])
    });
}

fn load_links(file: &str) -> Option<Vec<[String; 2]>> {
    let mut out = Vec::new();
    let data = fs::read_to_string(file).ok()?;

    for i in data.lines() {
        if i.is_empty() || i.starts_with(';') || i.starts_with('#') {
            continue;
        }
        let mut parts = i.splitn(2, '=');
        let code = parts.next()?.trim().to_owned();
        let link = parts.next()?.trim().to_owned();
        out.push([code, link]);
    }

    Some(out)
}
