use std::{
    fs::{self, File},
    path::Path,
    sync::Arc,
};

use afire::{
    extension::{serve_static, RealIp}, prelude::MiddleResult, Content, Method, Middleware, Request, Response,
    Server,
};

use crate::{
    app::App,
    assets::{self, WRITING},
};

struct Article(pub Arc<App>);

// TODO: Convert this into a route and do good error handling
impl Middleware for Article {
    fn pre(&self, req: &mut Request) -> MiddleResult {
        // Continue on non GET requests
        if req.method != Method::GET {
            return MiddleResult::Continue;
        }

        // Handel writing asset reuqests
        if req.path.starts_with("/writing/assets/") {
            let file = req
                .path
                .strip_prefix("/writing/assets/")
                .unwrap_or_default()
                .replace("..", "");

            let path = Path::new(&self.0.config.writing_path)
                .join("assets")
                .join(file);

            let ext = path.extension().unwrap_or_default().to_string_lossy();
            let mime = serve_static::get_type(ext.to_lowercase().as_str(), &serve_static::TYPES)
                .unwrap_or("application/octet-stream");

            if let Ok(data) = File::open(path) {
                return MiddleResult::Send(
                    Response::new().stream(data).content(Content::Custom(mime)),
                );
            }
        }

        // Handel requests for the base articles
        if req.path.starts_with("/writing/") {
            let code = req.path.strip_prefix("/writing/").unwrap_or_default();
            let articles = self.0.articles.articles.read();
            let doc = match articles.get(code) {
                Some(i) => i,
                None => return MiddleResult::Continue,
            };

            let data = match fs::read_to_string(&doc.file_path) {
                Ok(i) => i,
                Err(i) => {
                    return MiddleResult::Send(
                        Response::new()
                            .status(500)
                            .text(
                                assets::ERROR_PAGE
                                    .replace("{{ERROR}}", i.to_string().as_str())
                                    .replace("{{VERSION}}", crate::VERSION),
                            )
                            .content(Content::HTML),
                    )
                }
            };
            let data = data.split_once("---").unwrap().1;

            // Get real Client IP
            let ip = req.real_ip().to_string();
            let mut conn = self.0.database.lock();
            let trans = conn.transaction().unwrap();
            // Add a view to the article if it hasn't been viewed before
            trans
                .execute(
                    "INSERT OR IGNORE INTO article_views (name, ip, date) VALUES (?1, ?2, \
                     strftime('%s','now'))",
                    rusqlite::params![doc.path, ip],
                )
                .unwrap();

            // Get View Count
            let views = trans
                .query_row(
                    "SELECT COUNT(*) FROM article_views WHERE name = ?1",
                    rusqlite::params![doc.path],
                    |row| row.get::<_, usize>(0),
                )
                .unwrap();

            // Get Total Liked
            let likes: usize = trans
                .query_row(
                    "SELECT COUNT(*) FROM article_likes WHERE name = ?1",
                    rusqlite::params![doc.path],
                    |row| row.get(0),
                )
                .unwrap();

            // Get if this ip has like the post
            let liked = trans
                .query_row(
                    "SELECT COUNT(*) FROM article_likes WHERE name = ?1 AND ip = ?2",
                    rusqlite::params![doc.path, ip],
                    |row| row.get::<_, usize>(0),
                )
                .unwrap();

            trans.commit().unwrap();

            let mut opt = comrak::ComrakOptions::default();
            opt.extension.table = true;
            opt.extension.strikethrough = true;
            opt.extension.autolink = true;
            opt.extension.header_ids = Some("".to_owned());
            // opt.extension.header_no_aria_hidden = false;
            opt.extension.footnotes = true;
            opt.parse.smart = true;
            opt.render.unsafe_ = true;

            let doc_render = comrak::markdown_to_html(data, &opt);
            let html = WRITING
                .replace("{{VERSION}}", crate::VERSION)
                .replace("{{DOCUMENT}}", &doc_render)
                .replace("{{AUTHOR}}", &doc.author)
                .replace("{{PATH}}", &doc.path)
                .replace("{{DATE}}", &doc.date)
                .replace("{{VIEWS}}", &views.to_string())
                .replace("{{LIKES}}", &likes.to_string())
                .replace("{{LIKED}}", &(liked >= 1).to_string())
                .replace("{{TIME}}", &(doc.words as f64 / 3.0).round().to_string())
                .replace("{{DISC}}", &doc.description)
                .replace("{{TAGS}}", &doc.tags.join(", "));
            return MiddleResult::Send(Response::new().text(html).content(Content::HTML));
        }

        // If no writing related path found, continue
        MiddleResult::Continue
    }
}

pub fn attach(server: &mut Server<App>) {
    let app = server.state.as_ref().unwrap().to_owned();
    Article(app).attach(server);
}
