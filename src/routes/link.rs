use afire::{Method, Response, Server};

use crate::{app::App, serve_static::not_found};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/r/{code}", |app, req| {
        let code = req.param("code").unwrap();
        let links = app.redirects.read();

        let link = match links.get(&code) {
            Some(i) => i,
            None => return not_found(&req.path),
        };

        Response::new()
            .status(308)
            .reason("Permanent Redirect")
            .text(format!(r#"<a href={link}>{link}</a>"#, link = link))
            .header("Content-Type", "text/html")
            .header("Location", link)
    });
}
