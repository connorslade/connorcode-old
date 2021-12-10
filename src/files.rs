use std::fs;
use std::path::PathBuf;

use afire::{Header, Method, Response, Server};

use crate::config::{FILE_SERVE, FILE_SERVE_PATH};
use crate::Template;
use crate::VERSION;

pub fn attach(server: &mut Server) {
    if !*FILE_SERVE {
        return;
    }

    server.middleware(Box::new(|req| {
        if req.method != Method::GET || !req.path.starts_with("/files") {
            return None;
        }

        let file_path = req.path.replace("/..", "");
        let mut file_path = file_path.strip_prefix("/files").unwrap().to_owned();

        while file_path.starts_with('/') {
            file_path = file_path.replacen("/", "", 1);
        }

        let path = PathBuf::from(FILE_SERVE_PATH.clone()).join(&file_path);

        if path.is_dir() {
            let dir = match fs::read_dir(&path) {
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

            let mut out = String::new();

            for i in dir {
                let j = i.to_str().unwrap();
                let url = j.split(&*FILE_SERVE_PATH).nth(1).unwrap().to_owned();
                let name = j.split(path.to_str().unwrap()).nth(1).unwrap().to_owned();

                out.push_str(&format!(
                    r#"<div class="file"><a href="/files{}"><i class="fa fa-{}"></i> {}</a></div>"#,
                    url,
                    if i.is_file() { "file" } else { "folder" },
                    name.strip_prefix('/').unwrap_or(&name)
                ));
            }

            return Some(
                Response::new()
                    .text(
                        Template::new(
                            fs::read_to_string("./data/template/files.html")
                                .unwrap_or_else(|_| "{{FILES}}".to_owned()),
                        )
                        .template("FILES", out)
                        .template("VERSION", VERSION)
                        .build(),
                    )
                    .header(Header::new("Content-Type", "text/html; charset=utf-8")),
            );
        }

        let file = match fs::read(path) {
            Ok(i) => i,
            Err(_) => {
                return Some(
                    Response::new()
                        .status(404)
                        .text(format!("File `{}` not found", file_path)),
                )
            }
        };
        return Some(
            Response::new()
                .bytes(file)
                .header(Header::new("Content-Type", "text/plain; charset=utf-8")),
        );
    }));
}
