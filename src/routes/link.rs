use std::fs;

use afire::{Header, Method, Response, Server};

lazy_static! {
    static ref LINKS: Vec<[String; 2]> =
        load_links("data/config/link.cfg").expect("Error Loading Links");
}

pub fn attach(server: &mut Server) {
    lazy_static::initialize(&LINKS);

    server.route(Method::GET, "/r/{code}", |req| {
        let code = req.path_param("code").unwrap();

        for i in (*LINKS).clone() {
            if i[0].to_lowercase() == code {
                let link = &i[1];
                return Response::new()
                    .status(308)
                    .reason("Permanent Redirect")
                    .text(format!(r#"<a href={link}>{link}</a>"#, link = link))
                    .header(Header::new("Content-Type", "text/html"))
                    .header(Header::new("Location", link));
            }
        }

        Response::new()
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
