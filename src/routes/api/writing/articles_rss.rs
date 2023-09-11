use afire::{Content, Method, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/writing.rss", |ctx| {
        ctx.text(ctx.app().articles.rss_cache.read())
            .content(Content::XML)
            .send()?;
        Ok(())
    });
}
