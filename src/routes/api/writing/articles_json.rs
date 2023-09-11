use afire::{Content, Method, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/writing", |ctx| {
        ctx.text(ctx.app().articles.api_cache.read())
            .content(Content::JSON)
            .send()?;
        Ok(())
    });
}
