use afire::{Content, Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::ANY, "/api/rawhttp", |req| {
        let mut out = format!("{} {} {}\n", req.method, req.path, req.version);

        for i in req.headers.iter() {
            out.push_str(i.to_string().as_str());
            out.push('\n');
        }

        let mut out = out.as_bytes().to_vec();
        out.push(b'\n');
        out.extend_from_slice(&req.body);

        Response::new().bytes(&out).content(Content::TXT)
    });
}
