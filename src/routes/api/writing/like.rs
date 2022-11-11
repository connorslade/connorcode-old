use afire::{Method, Response, Server};
use serde::Deserialize;

use crate::{app::App, common::get_ip};

#[derive(Deserialize)]
struct RequestData {
    doc: String,
    like: bool,
}

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::POST, "/api/writing/like", |app, req| {
        let body = String::from_utf8_lossy(&req.body);
        let json = serde_json::from_str::<RequestData>(&body).unwrap();
        let ip = get_ip(&req);

        // Verify Document
        let articles = app.articles.read();
        let document = match articles.get(&json.doc) {
            Some(i) => i,
            None => panic!("Article Not Found"),
        };

        let connection = app.database.lock();
        if json.like {
            connection
                .execute(
                    "INSERT OR IGNORE INTO article_likes (name, ip, date) VALUES (?1, ?2, strftime('%s','now'))",
                    rusqlite::params![document.path, ip],
                )
                .unwrap();
            return Response::new();
        }

        connection
            .execute(
                "DELETE FROM article_likes where name = ?1 AND ip = ?2",
                rusqlite::params![document.path, ip],
            )
            .unwrap();

        Response::new()
    });
}
