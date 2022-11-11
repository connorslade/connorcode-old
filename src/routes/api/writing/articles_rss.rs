use afire::{Content, Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/writing.rss", |app, _req| {
        Response::new()
            .text(app.articles.rss_cache.read())
            .content(Content::XML)
    });
}
