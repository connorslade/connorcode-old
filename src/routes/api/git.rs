use afire::{Content, Method, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/git", |ctx| {
        ctx.text(env!("GIT_INFO")).content(Content::TXT).send()?;
        Ok(())
    });
}
