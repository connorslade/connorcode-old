use afire::{Content, Method, Server};

use crate::{app::App, common::RealIp};

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/ip", |ctx| {
        ctx.text(ctx.req.real_ip()).content(Content::TXT).send()?;
        Ok(())
    });
}
