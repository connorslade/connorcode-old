use afire::{Content, Method, Response, Server};
use unindent::unindent;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/writing.rss", |app, _req| {
        let articles = app.articles.read();
        let mut article_vec = articles.iter().map(|x| x.1).collect::<Vec<_>>();
        article_vec.sort_unstable_by(|x, y| y.epoch.cmp(&x.epoch));

        let mut out = String::new();
        for i in article_vec.iter().filter(|x| !x.hidden) {
            out.push_str(i.rssify(&app.config.external_uri).as_str());
            out.push_str("\n\n");
        }

        out.pop();
        out.pop();

        let out = unindent(
            format!(
                r#"<?xml version="1.0" encoding="UTF-8" ?>
            <rss version="2.0">
            <channel>
             <title>ConnorCode</title>
             <description>ConnorCode Articles</description>
             <link>{}</link>
             <copyright>Connor Slade</copyright>
             <language>en</language>
             <ttl>1800</ttl>

             {}

            </channel>
            </rss>"#,
                app.config.external_uri, out
            )
            .as_str(),
        );

        Response::new().text(out).content(Content::XML)
    });
}
