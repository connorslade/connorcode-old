use std::fs;
use std::path::PathBuf;

use afire::{
    middleware::{MiddleRequest, Middleware},
    Content, Method, Request, Response, Server,
};
use chrono::prelude::*;
use lazy_static::LazyStatic;
use simple_config_parser::Config;

use crate::Template;

lazy_static! {
    static ref DOCS: Vec<Document> = Document::load_documents();
    static ref DOCS_API: String = gen_api_data();
}

#[derive(Debug, Clone)]
struct Document {
    path: String,
    file_path: PathBuf,
    title: String,
    date: String,
    description: String,
    hidden: bool,
    assets: PathBuf,
}

struct Markdown;

pub fn attach(server: &mut Server) {
    LazyStatic::initialize(&DOCS);

    Markdown.attach(server);
    server.route(Method::GET, "/api/writing", |_req| {
        Response::new().text(&*DOCS_API).content(Content::JSON)
    });
}

impl Document {
    fn load_documents() -> Vec<Self> {
        let mut out = Vec::new();

        let files = fs::read_dir("./data/writing").expect("Error Reading Writing Dir");
        for i in files {
            let i = i.unwrap();

            if i.path()
                .extension()
                .unwrap()
                .to_str()
                .unwrap()
                .to_lowercase()
                .as_str()
                != "md"
            {
                continue;
            }

            let data = fs::read_to_string(i.path()).expect("Error Reading a Writing");
            let mut parts = data.splitn(2, "---");

            let cfg = Config::new()
                .text(parts.next().unwrap())
                .expect("Error Parseing a Writing Config");

            out.push(Document {
                path: cfg.get_str("@Path").expect("Error geting Writing Path"),
                file_path: i.path(),
                title: cfg.get_str("@Title").expect("Error geting Writing Title"),
                date: cfg.get_str("@Date").expect("Error geting Writing Date"),
                description: cfg
                    .get_str("@Description")
                    .expect("Error geting Writing Description"),
                hidden: cfg.get("@Hidden").unwrap_or(false),
                assets: PathBuf::from(cfg.get_str("@Assets").expect("Error geting Writing Assets")),
            })
        }

        out.sort_unstable_by(|x, y| {
            let x_parts = x.date.split("-").collect::<Vec<_>>();
            let y_parts = y.date.split("-").collect::<Vec<_>>();

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

    fn jsonify(&self) -> String {
        format!(
            r#"{{"name": "{}", "disc": "{}", "date": "{}", "link": "/writing/{}"}}"#,
            self.title, self.description, self.date, self.path
        )
    }
}

impl Middleware for Markdown {
    fn pre(&mut self, req: Request) -> MiddleRequest {
        if req.method != Method::GET || !req.path.starts_with("/writing/") {
            return MiddleRequest::Continue;
        }

        let code = req.path.strip_prefix("/writing/").unwrap();
        let doc = match (*DOCS).iter().find(|x| x.path == code) {
            Some(i) => i,
            None => return MiddleRequest::Continue,
        };

        let data = fs::read_to_string(&doc.file_path).unwrap();
        let data = data.split_once("---").unwrap().1;

        let mut opt = comrak::ComrakOptions::default();
        opt.extension.table = true;
        opt.extension.strikethrough = true;
        opt.extension.autolink = true;

        let doc_render = comrak::markdown_to_html(&data, &opt);
        let html = Template::new(include_str!("../../data/template/writing.html"))
            .template("VERSION", crate::VERSION)
            .template("DOCUMENT", doc_render)
            .template("PATH", &doc.path)
            .template("DATE", &doc.date)
            .build();

        MiddleRequest::Send(Response::new().text(html).content(Content::HTML))
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
