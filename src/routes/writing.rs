use std::borrow::Borrow;
use std::fs::{self, DirEntry};
use std::path::{Path, PathBuf};

use afire::{
    middleware::{MiddleRequest, Middleware},
    Content, Method, Request, Response, Server,
};
use chrono::prelude::*;
use lazy_static::LazyStatic;
use rusqlite;
use simple_config_parser::Config;
use unindent::unindent;

use crate::assets::WRITING;
use crate::color::{color, Color};
use crate::config::{EXTERNAL_URI, WRITING_PATH};
use crate::Template;

lazy_static! {
    static ref DOCS: Vec<Document> = Document::load_documents(PathBuf::from(&*WRITING_PATH));
    static ref DOCS_API: String = gen_api_data();
    static ref DOCS_RSS: String = gen_rss_data();
}

#[derive(Debug, Clone)]
struct Document {
    file_path: PathBuf,
    path: String,

    title: String,
    date: String,
    author: String,
    description: String,

    tags: Vec<String>,
    hidden: bool,
    words: usize,
    icon: String,
}

struct Markdown {
    connection: rusqlite::Connection,
}

pub fn attach(server: &mut Server) {
    LazyStatic::initialize(&DOCS);

    Markdown::new(&DOCS).attach(server);
    server.route(Method::GET, "/api/writing", |_req| {
        Response::new().text(&*DOCS_API).content(Content::JSON)
    });

    server.route(Method::GET, "/writing.rss", |_req| {
        Response::new().text(&*DOCS_RSS).content(Content::XML)
    });
}

macro_rules! safe_config {
    ($e: expr, $d: expr, $dn: expr) => {{
        if $e.ok().is_none() {
            let text = format!(
                "[-] No `{}` defined on document `{}`",
                $d,
                $dn.as_os_str().to_str().unwrap_or_default()
            );
            println!("{}", color(text, Color::Red));
        }

        $e.ok()
    }};
}

impl Document {
    fn load_documents(path: PathBuf) -> Vec<Self> {
        let mut out = Vec::new();

        let files = fs::read_dir(path).expect("Error Reading Writing Dir");
        for i in files {
            let i = i.unwrap();

            if i.path().is_dir() {
                out.append(&mut Document::load_documents(i.path()));
            }

            if i.file_name().to_string_lossy() == "README.md" {
                continue;
            }

            if let Some(doc) = Document::load_document(i) {
                out.push(doc);
            };
        }

        out.sort_unstable_by(|x, y| {
            let x_parts = x.date.split('-').collect::<Vec<_>>();
            let y_parts = y.date.split('-').collect::<Vec<_>>();

            let x = Utc.ymd(
                x_parts[2].parse().unwrap(),
                x_parts[0].parse().unwrap(),
                x_parts[1].parse().unwrap(),
            );
            let y = Utc.ymd(
                y_parts[2].parse().unwrap(),
                y_parts[0].parse().unwrap(),
                y_parts[1].parse().unwrap(),
            );

            y.cmp(&x)
        });

        out
    }

    fn load_document(i: DirEntry) -> Option<Document> {
        if i.path().extension()?.to_str()?.to_lowercase().as_str() != "md" {
            return None;
        }

        let data = fs::read_to_string(i.path()).expect("Error Reading a Writing");
        let mut parts = data.splitn(2, "---");

        let cfg = Config::new()
            .text(parts.next().unwrap())
            .expect("Error Parseing a Writing Config");

        let words = parts.next().unwrap().split_whitespace().count();

        let tags = cfg
            .get_str("@Tags")
            .unwrap_or_default()
            .split(',')
            .map(|x| x.trim().to_owned())
            .collect();

        let icon = cfg
            .get_str("@Icon")
            .unwrap_or_else(|_| "file-text-o".to_owned());

        let file_path = i.path();
        let path = safe_config!(cfg.get_str("@Path"), "Path", i.path());
        let title = safe_config!(cfg.get_str("@Title"), "Title", i.path());
        let date = safe_config!(cfg.get_str("@Date"), "Date", i.path());
        let description = safe_config!(cfg.get_str("@Description"), "Description", i.path());
        let author = cfg
            .get_str("@Author")
            .unwrap_or_else(|_| "Connor Slade".to_owned());
        let hidden = cfg.get("@Hidden").unwrap_or(false);

        Some(Document {
            file_path,
            path: path?,
            title: title?,
            date: date?,
            description: description?,
            words,
            author,
            hidden,
            tags,
            icon,
        })
    }

    fn jsonify(&self) -> String {
        format!(
            r#"{{"name": "{}", "disc": "{}", "date": "{}", "icon": "{}", "link": "/writing/{}", "reading": "{}"}}"#,
            self.title,
            self.description,
            self.date,
            self.icon,
            self.path,
            (self.words as f64 / 3.5).round()
        )
    }

    fn rssify(&self) -> String {
        let parts = self.date.split('-').collect::<Vec<_>>();
        let date = Utc
            .ymd(
                parts[2].parse().unwrap(),
                parts[0].parse().unwrap(),
                parts[1].parse().unwrap(),
            )
            .and_time(NaiveTime::from_hms(0, 0, 0))
            .unwrap();

        unindent(
            format!(
                r#"<item>
                     <title>{}</title>
                     <description>{}</description>
                     <pubDate>{}</pubDate>
                     <link>{}/writing/{}</link>
                 </item>"#,
                self.title,
                self.description,
                date.to_rfc2822(),
                *EXTERNAL_URI,
                self.path
            )
            .as_str(),
        )
    }
}

impl Middleware for Markdown {
    fn pre(&mut self, req: Request) -> MiddleRequest {
        if req.method != Method::GET || !req.path.starts_with("/writing/") {
            return MiddleRequest::Continue;
        }

        if req.path.starts_with("/writing/assets/") {
            let file = req
                .path
                .strip_prefix("/writing/assets/")
                .unwrap_or_default()
                .replace("..", "");

            let path = Path::new(&*WRITING_PATH).join("assets").join(&file);
            let ext = path.extension().unwrap_or_default();
            let ext = ext.borrow().to_str().unwrap_or_default();

            let mime = match ext.to_lowercase().as_str() {
                "png" => "image/png",
                "jpg" => "image/jpeg",
                "jpeg" => "image/jpeg",
                "svg" => "image/svg+xml",
                _ => "",
            };

            if let Ok(data) = fs::read(path) {
                return MiddleRequest::Send(
                    Response::new().bytes(data).content(Content::Custom(mime)),
                );
            }
        }

        let code = req.path.strip_prefix("/writing/").unwrap_or_default();
        let doc = match (*DOCS).iter().find(|x| x.path == code) {
            Some(i) => i,
            None => return MiddleRequest::Continue,
        };

        let data = match fs::read_to_string(&doc.file_path) {
            Ok(i) => i,
            Err(i) => {
                return MiddleRequest::Send(
                    Response::new()
                        .status(500)
                        .text(
                            Template::new(crate::assets::ERROR_PAGE)
                                .template("ERROR", i)
                                .template("VERSION", crate::VERSION)
                                .build(),
                        )
                        .content(Content::HTML),
                )
            }
        };
        let data = data.split_once("---").unwrap().1;

        let trans = self.connection.transaction().unwrap();
        let views: u32 = trans
            .query_row(
                "SELECT views from article where name = ?1 LIMIT 1",
                [doc.path.clone()],
                |row| row.get(0),
            )
            .unwrap();
        let views = views + 1;

        trans
            .execute(
                "UPDATE article SET views = ?2 WHERE name = ?1",
                rusqlite::params![doc.path.clone(), views],
            )
            .unwrap();

        trans.commit().unwrap();

        let mut opt = comrak::ComrakOptions::default();
        opt.extension.table = true;
        opt.extension.strikethrough = true;
        opt.extension.autolink = true;
        opt.extension.header_ids = Some("markdown-".to_owned());
        opt.extension.footnotes = true;
        opt.parse.smart = true;
        opt.render.unsafe_ = true;

        let doc_render = comrak::markdown_to_html(data, &opt);
        let html = Template::new(WRITING)
            .template("VERSION", crate::VERSION)
            .template("DOCUMENT", doc_render)
            .template("AUTHOR", &doc.author)
            .template("PATH", &doc.path)
            .template("DATE", &doc.date)
            .template("VIEWS", views)
            .template("TIME", (doc.words as f64 / 3.5).round())
            .template("DISC", &doc.description)
            .template("TAGS", &doc.tags.join(", "))
            .build();

        MiddleRequest::Send(Response::new().text(html).content(Content::HTML))
    }
}

impl Markdown {
    fn new(docs: &[Document]) -> Self {
        let mut conn = rusqlite::Connection::open("data/data.db").unwrap();

        let trans = conn.transaction().unwrap();
        trans
            .execute(
                "CREATE TABLE IF NOT EXISTS article (
              name TEXT NOT NULL PRIMARY KEY,
              views INTEGER NOT NULL,
              UNIQUE(name)
              )",
                [],
            )
            .unwrap();

        for i in docs {
            trans
                .execute(
                    r#"INSERT OR IGNORE INTO article (name, views) VALUES (?1, ?2)"#,
                    [i.path.clone(), 0.to_string()],
                )
                .unwrap();
        }

        trans
            .execute(
                "CREATE UNIQUE INDEX IF NOT EXISTS articles_name_index ON article(name);",
                [],
            )
            .unwrap();

        trans.commit().unwrap();

        conn.pragma_update(None, "journal_mode", "WAL").unwrap();
        conn.pragma_update(None, "synchronous", "off").unwrap();

        Self { connection: conn }
    }
}

fn gen_api_data() -> String {
    let mut out = String::new();

    for i in &*DOCS {
        if i.hidden {
            continue;
        }

        out.push_str(i.jsonify().as_str());
        out.push_str(", ");
    }
    out.pop();
    out.pop();

    format!("[{}]", out)
}

fn gen_rss_data() -> String {
    let mut out = String::new();

    for i in &*DOCS {
        if i.hidden {
            continue;
        }

        out.push_str(i.rssify().as_str());
        out.push_str("\n\n");
    }
    out.pop();
    out.pop();

    unindent(
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
            *EXTERNAL_URI, out
        )
        .as_str(),
    )
}
