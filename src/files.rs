use std::any::type_name;
use std::fs;
use std::path::{Path, PathBuf};

use afire::{
    error::Result,
    internal::common::trace,
    middleware::{MiddleRequest, Middleware},
    Method, Request, Response, Server,
};

use crate::common::{best_size, best_time};
use crate::config::{FILE_SERVE, FILE_SERVE_PATH};
use crate::Template;
use crate::VERSION;

#[rustfmt::skip]
const FILE_ICONS: &[(&str, &[&str])] = &[
    ("file-code-o",       &["rs", "python", "lua", "scriptable", "js", "html", "css", "scss"]),
    ("file-audio-o",      &["mp3", "wav", "flac"]),
    ("file-video-o",      &["mp4", "mov"]),
    ("file-image-o",      &["png", "jpg", "jpeg", "gif"]),
    ("file-archive-o",    &["zip", "7z", "rar", "tar", "gz"]),
    ("file-text-o",       &["txt", "md"]),
    ("file-powerpoint-o", &["pptx", "ppt"]),
    ("file-exel-o",       &["xlsx", "xls"]),
    ("file-word-o",       &["docx", "doc"]),
    ("file-pdf-o",        &["pdf"]),
];

pub struct Files;

impl Middleware for Files {
    fn attach<State>(self, server: &mut Server<State>)
    where
        Self: 'static + Send + Sync + Sized,
        State: 'static + Send + Sync,
    {
        if !*FILE_SERVE {
            return;
        }

        trace(format!("📦 Adding Middleware {}", type_name::<Self>()));

        server.middleware.push(Box::new(self));
    }

    fn pre(&self, req: &Result<Request>) -> MiddleRequest {
        let req = match req {
            Ok(i) => i,
            Err(_) => return MiddleRequest::Continue,
        };

        if !req.path.starts_with("/files") || req.method != Method::GET {
            return MiddleRequest::Continue;
        }

        let mut file_path = req.path.to_owned();
        while file_path.contains("/..") {
            file_path = file_path.replace("/..", "");
        }

        file_path = file_path
            .strip_prefix("/files")
            .expect("Error Striping /files prefix")
            .to_owned();

        while file_path.starts_with('/') {
            file_path.remove(0);
        }

        let path = PathBuf::from(FILE_SERVE_PATH.clone()).join(&file_path);

        if path.is_dir() {
            let mut dir = match fs::read_dir(&path) {
                Ok(i) => i,
                Err(_) => {
                    return MiddleRequest::Send(
                        Response::new()
                            .status(404)
                            .text(format!("Folder `{}` not found", file_path)),
                    )
                }
            }
            .map(|x| x.expect("Error getting subfile").path())
            .collect::<Vec<PathBuf>>();

            dir.sort();

            let mut out = String::new();

            if path != PathBuf::from(FILE_SERVE_PATH.clone()) {
                out.push_str(&format!(
                    r#"<div class="file"><i class="fa fa-folder"></i><a href="/files{}">..</a><p class="size"></p></div>"#,
                    path.parent().unwrap().to_string_lossy().replacen(
                        FILE_SERVE_PATH.as_str(),
                        "",
                        1
                    )
                ));
            }

            for i in dir {
                let j = i.to_string_lossy();
                let url = j.split(&*FILE_SERVE_PATH).nth(1).unwrap().to_owned();
                let name = j.split(path.to_str().unwrap()).nth(1).unwrap().to_owned();
                let mut size = i.metadata().unwrap().len();

                if i.is_dir() {
                    if let Ok(sub_dir) = fs::read_dir(&i) {
                        for i in sub_dir.map(|x| x.unwrap().path()).collect::<Vec<PathBuf>>() {
                            size += i.metadata().unwrap().len();
                        }
                    }
                }

                out.push_str(&format!(
                    r#"<div class="file"><i class="fa fa-{}"></i><a href="/files{}">{}</a><p class="size">{}</p><p class="modified">{} ago</p></div>"#,
                    path_icon(&i),
                    url,
                    name.strip_prefix('/')
                        .unwrap_or(&name)
                        .strip_prefix('\\')
                        .unwrap_or(&name),
                    best_size(size),
                    best_time(i.metadata().unwrap().modified().unwrap().elapsed().unwrap().as_secs())
                ));
            }

            return MiddleRequest::Send(
                Response::new()
                    .text(
                        Template::new(
                            fs::read_to_string("./data/web/template/files.html")
                                .unwrap_or_else(|_| "{{FILES}}".to_owned()),
                        )
                        .template("PATH", path.file_name().unwrap().to_str().unwrap())
                        .template("FILES", out)
                        .template("VERSION", VERSION)
                        .build(),
                    )
                    .header("Content-Type", "text/html; charset=utf-8"),
            );
        }

        let file = match fs::read(&path) {
            Ok(i) => i,
            Err(_) => {
                return MiddleRequest::Send(
                    Response::new()
                        .status(404)
                        .text(format!("File `{}` not found", file_path)),
                )
            }
        };

        if req.query.get("download").is_some() || req.query.get("raw").is_some() {
            return MiddleRequest::Send(
                Response::new()
                    .bytes(file)
                    .header("Content-Type", "text/html; charset=utf-8"),
            );
        }

        MiddleRequest::Send(
            show_response(path)
                .unwrap_or_else(|| {
                    Response::new().header("Content-Type", "text/html; charset=utf-8")
                })
                .bytes(file),
        )
    }
}

impl Files {
    pub fn new() -> Self {
        Self
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

    Some(Response::new().header("Content-Type", content_type))
}

fn path_icon(path: &Path) -> String {
    if path.is_dir() {
        return "folder".to_owned();
    }

    for i in FILE_ICONS {
        for j in i.1 {
            let path = path
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .into_owned()
                .to_lowercase();

            if *j == path {
                return i.0.to_owned();
            }
        }
    }

    "file".to_owned()
}
