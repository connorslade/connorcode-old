use afire::{Content, Method, Response, Server};
use serde_json::json;

use crate::{app::App, writing::Article};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/api/writing", |app, _req| {
        let articles = app.articles.read();
        let mut article_vec = articles.iter().map(|x| x.1).collect::<Vec<_>>();
        article_vec.sort_unstable_by(|x, y| y.epoch.cmp(&x.epoch));

        Response::new()
            .text(json!(article_vec
                .into_iter()
                .map(Article::jsonify)
                .collect::<Vec<_>>()))
            .content(Content::JSON)
    });
}
