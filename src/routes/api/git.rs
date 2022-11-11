use afire::{Content, Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/git", |_req| {
        Response::new().text(env!("GIT_INFO")).content(Content::TXT)
    });
}
