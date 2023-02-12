use afire::{Content, Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/rawhttp", |req| {
        let mut out = format!("{} {} {}", req.method, req.path, req.version);

        for i in req.headers.iter() {
            out.push_str(i.to_string().as_str());
            out.push('\n');
        }

        Response::new().text(out).content(Content::TXT)
    });
}
