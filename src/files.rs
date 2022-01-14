use std::cell::RefCell;
use std::fs;
use std::path::PathBuf;

use afire::{
    middleware::{MiddleRequest, Middleware},
    Header, Method, Request, Response, Server,
};

use crate::common::{best_size, best_time};
use crate::config::{FILE_SERVE, FILE_SERVE_PATH};
use crate::Template;
use crate::VERSION;

pub struct Files;

impl Middleware for Files {
    fn attach(self, server: &mut Server)
    where
        Self: Sized + 'static,
    {
        if !*FILE_SERVE {
            return;
        }

        server.middleware.push(Box::new(RefCell::new(self)));
    }

    fn pre(&mut self, req: Request) -> MiddleRequest {
        if !req.path.starts_with("/files") || req.method != Method::GET {
            return MiddleRequest::Continue;
        }

        MiddleRequest::Send(self.run(req).unwrap_or_else(|| {
            Response::new()
                .status(500)
                .text(
                    Template::new(crate::assets::ERROR_PAGE)
                        .template("VERSION", VERSION)
                        .template("ERROR", "smthng")
                        .build(),
                )
                .header(Header::new("Content-Type", "text/html"))
        }))
    }
}

impl Files {
    pub fn new() -> Self {
        Files
    }

    fn run(&self, req: Request) -> Option<Response> {
        let file_path = req.path.replace("/..", "");
        let mut file_path = file_path.strip_prefix("/files")?.to_owned();

        while file_path.starts_with('/') {
            file_path = file_path.replacen("/", "", 1);
        }

        let path = PathBuf::from(FILE_SERVE_PATH.clone()).join(&file_path);

        if path.is_dir() {
            let mut dir = match fs::read_dir(&path) {
                Ok(i) => i,
                Err(_) => {
                    return Some(
                        Response::new()
                            .status(404)
                            .text(format!("Folder `{}` not found", file_path)),
                    )
                }
            }
            .map(|x| x.unwrap().path())
            .collect::<Vec<PathBuf>>();

            dir.sort();

            let mut out = String::new();

            if path != PathBuf::from(FILE_SERVE_PATH.clone()) {
                out.push_str(&format!(
                    r#"<div class="file"><i class="fa fa-folder"></i><a href="/files{}">..</a><p class="size"></p></div>"#,
                    path.parent()?.to_string_lossy().replacen(
                        FILE_SERVE_PATH.as_str(),
                        "",
                        1
                    )
                ));
            }

            for i in dir {
                let j = i.to_string_lossy();
                let url = j.split(&*FILE_SERVE_PATH).nth(1)?.to_owned();
                let name = j.split(path.to_str()?).nth(1)?.to_owned();
                let mut size = i.metadata().ok()?.len();

                if i.is_dir() {
                    if let Ok(sub_dir) = fs::read_dir(&i) {
                        for i in sub_dir.map(|x| x.unwrap().path()).collect::<Vec<PathBuf>>() {
                            size += i.metadata().ok()?.len();
                        }
                    }
                }

                out.push_str(&format!(
                    r#"<div class="file"><i class="fa fa-{}"></i><a href="/files{}">{}</a><p class="size">{}</p><p class="modified">{}</p></div>"#,
                    match i.is_file() {
                        true => "file",
                        _ => "folder",
                    },
                    url,
                    name.strip_prefix('/')
                        .unwrap_or(&name)
                        .strip_prefix('\\')
                        .unwrap_or(&name),
                    best_size(size),
                    format!("{} ago", best_time(i.metadata().ok()?.modified().ok()?.elapsed().ok()?.as_secs()))
                ));
            }

            return Some(
                Response::new()
                    .text(
                        Template::new(
                            fs::read_to_string("./data/web/template/files.html")
                                .unwrap_or_else(|_| "{{FILES}}".to_owned()),
                        )
                        .template("PATH", path.file_name()?.to_str()?)
                        .template("FILES", out)
                        .template("VERSION", VERSION)
                        .build(),
                    )
                    .header(Header::new("Content-Type", "text/html; charset=utf-8")),
            );
        }

        let file = match fs::read(&path) {
            Ok(i) => i,
            Err(_) => {
                return Some(
                    Response::new()
                        .status(404)
                        .text(format!("File `{}` not found", file_path)),
                )
            }
        };

        if req.query.get("download").is_some() || req.query.get("raw").is_some() {
            return Some(
                Response::new()
                    .bytes(file)
                    .header(Header::new("Content-Type", "application/octet-stream")),
            );
        }

        Some(
            show_response(path)
                .unwrap_or_else(|| {
                    Response::new().header(Header::new("Content-Type", "application/octet-stream"))
                })
                .bytes(file),
        )
    }
}

fn show_response(file: PathBuf) -> Option<Response> {
    let content_type = match file.extension()?.to_str()?.to_lowercase().as_str() {
        "txt" => "text/plain; charset=utf-8",
        "html" => "text/html; charset=utf-8",
        "css" => "text/css; charset=utf-8",
        "js" => "text/javascript; charset=utf-8",
        "json" => "application/json; charset=utf-8",
        "png" => "image/png",
        "jpg" => "image/jpeg",
        "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "ico" => "image/x-icon",
        "woff" => "application/font-woff",
        "woff2" => "application/font-woff2",
        "ttf" => "application/font-ttf",
        "otf" => "application/font-otf",
        "mp3" => "audio/mpeg",
        "mp4" => "video/mp4",
        _ => "application/octet-stream",
    };

    Some(Response::new().header(Header::new("Content-Type", content_type)))
}
