use afire::Content;
use afire::Method;
use afire::Response;
use afire::Server;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/headers", |req| {
        let mut resp = String::new();

        for i in req.headers.iter() {
            resp.push_str(i.to_string().as_str());
            resp.push('\n');
        }

        Response::new().text(resp).content(Content::TXT)
    });
}
