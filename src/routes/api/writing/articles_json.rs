use afire::{Content, Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/api/writing", |app, _req| {
        Response::new()
            .text(app.articles.api_cache.read())
            .content(Content::JSON)
    });
}
