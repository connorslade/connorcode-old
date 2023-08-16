use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use afire::internal::encoding::url;
use afire::HeaderType;
use afire::{
    middleware::{MiddleResult, Middleware},
    trace, Method, Request, Response, Server,
};

use crate::app::App;
use crate::common::{best_size, best_time};
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

pub struct Files(pub Arc<App>);

impl Middleware for Files {
    fn attach<State>(self, server: &mut Server<State>)
    where
        Self: 'static + Send + Sync + Sized,
        State: 'static + Send + Sync,
    {
        if !self.0.config.file_serve {
            return;
        }

        trace!("📦 Adding Middleware {}", type_name::<Self>());

        server.middleware.push(Box::new(self));
    }

    fn pre(&self, req: &mut Request) -> MiddleResult {
        let Some(mut file_path) = url::decode(&req.path) else {
            return MiddleResult::Continue;
        };

        if !file_path.starts_with("/files") || req.method != Method::GET {
            return MiddleResult::Continue;
        }

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

        let path = PathBuf::from(&self.0.config.file_serve_path).join(&file_path);

        if path.is_dir() {
            let mut dir = match fs::read_dir(&path) {
                Ok(i) => i,
                Err(_) => {
                    return MiddleResult::Send(
                        Response::new()
                            .status(404)
                            .text(format!("Folder `{}` not found", file_path)),
                    )
                }
            }
            .map(|x| x.expect("Error getting sub-file").path())
            .collect::<Vec<PathBuf>>();

            dir.sort();

            let mut out = String::new();

            if path != PathBuf::from(&self.0.config.file_serve_path) {
                out.push_str(&format!(
                    r#"<div class="file"><i class="fa fa-folder"></i><a href="/files{}">..</a><p class="size"></p></div>"#,
                    path.parent().unwrap().to_string_lossy().replacen(
                        self.0.config.file_serve_path.as_str(),
                        "",
                        1
                    )
                ));
            }

            for i in dir {
                let j = i.to_string_lossy();
                let url = j
                    .split(&self.0.config.file_serve_path)
                    .nth(1)
                    .unwrap()
                    .to_owned();
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

            return MiddleResult::Send(
                Response::new()
                    .text(
                        fs::read_to_string("./web/dist/template/files.html")
                            .unwrap_or_else(|_| "{{FILES}}".to_owned())
                            .replace("{{PATH}}", path.file_name().unwrap().to_str().unwrap())
                            .replace("{{FILES}}", &out)
                            .replace("{{VERSION}}", VERSION),
                    )
                    .header("Content-Type", "text/html; charset=utf-8"),
            );
        }

        let file = match File::open(&path) {
            Ok(i) => i,
            Err(_) => {
                return MiddleResult::Send(
                    Response::new()
                        .status(404)
                        .text(format!("File `{}` not found", file_path)),
                )
            }
        };

        let mut res = Response::new().header(
            HeaderType::ContentType,
            get_content_type(path).unwrap_or("application/octet-stream"),
        );
        if let Ok(i) = file.metadata() {
            res.headers.add("Content-Length", i.len().to_string());
        }

        if req.query.get("download").is_some() || req.query.get("raw").is_some() {
            if let Some(i) = res
                .headers
                .iter()
                .position(|x| x.name == HeaderType::ContentType)
            {
                res.headers.remove(i);
            }
        }

        MiddleResult::Send(res.stream(file))
    }
}

fn get_content_type(file: PathBuf) -> Option<&'static str> {
    Some(match file.extension()?.to_str()?.to_lowercase().as_str() {
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
    })
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
