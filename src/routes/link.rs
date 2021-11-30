use std::fs;

use afire::{Header, Method, Request, Response, Server};

pub fn attach(server: &mut Server) {
    server.middleware(Box::new(|req| link(req)));
}

fn link(req: &Request) -> Option<Response> {
    if req.method != Method::GET || !req.path.starts_with("/r/") {
        return None;
    }

    let code = req.path.split("/r/").last()?.to_lowercase();
    let links = load_links("data/config/link.cfg")?;

    for i in links {
        if i[0].to_lowercase() == code {
            let link = &i[1];
            return Some(
                Response::new()
                    .status(308)
                    .reason("Permanent Redirect")
                    .text(format!(r#"<a href={link}>{link}</a>"#, link = link))
                    .header(Header::new("Content-Type", "text/html"))
                    .header(Header::new("Location", link)),
            );
        }
    }

    None
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
