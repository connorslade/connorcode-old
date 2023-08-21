use afire::{Method, Server};
use serde::Deserialize;

use crate::{app::App, common::RealIp};

#[derive(Deserialize)]
struct RequestData {
    doc: String,
    like: bool,
}

pub fn attach(server: &mut Server<App>) {
    server.route(Method::POST, "/api/writing/like", |ctx| {
        let app = ctx.app();

        let body = String::from_utf8_lossy(&ctx.req.body);
        let json = serde_json::from_str::<RequestData>(&body)?;
        let ip = ctx.req.real_ip().to_string();

        // Verify Document
        let articles = app.articles.articles.read();
        let document = match articles.get(&json.doc) {
            Some(i) => i,
            None => panic!("Article Not Found"),
        };

        let connection = app.database.lock();
        if json.like {
            connection.execute(
                "INSERT OR IGNORE INTO article_likes (name, ip, date) VALUES (?1, ?2, \
                 strftime('%s','now'))",
                rusqlite::params![document.path, ip],
            )?;
            ctx.send()?;
            return Ok(());
        }

        connection.execute(
            "DELETE FROM article_likes where name = ?1 AND ip = ?2",
            rusqlite::params![document.path, ip],
        )?;

        ctx.send()?;
        return Ok(());
    });
}
