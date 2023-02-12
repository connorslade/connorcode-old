use afire::{Content, Method, Response, Server};

use crate::{app::App, common::RealIp};

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/ip", |req| {
        Response::new().text(req.real_ip()).content(Content::TXT)
    });
}
