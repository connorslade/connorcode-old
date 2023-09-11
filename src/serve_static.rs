use std::fs;

use afire::HeaderName;
use afire::{extensions::ServeStatic, Middleware, Response, Server};

use crate::app::App;
use crate::VERSION;

/// Files not to serve
const DONT_SERVE_PATH: &str = "data/config/dont_serve.txt";

lazy_static! {
    static ref DONT_SERVE: Vec<String> = {
        let mut working = Vec::new();
        let dont_serve =
            fs::read_to_string(DONT_SERVE_PATH).expect("Error reading DONT_SERVE file");
        for line in dont_serve.lines() {
            if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
                continue;
            }

            working.push(line.to_owned());
        }

        working
    };
}

pub fn attach(server: &mut Server<App>) {
    let data_dir = &server.state.as_ref().unwrap().config.data_dir;
    ServeStatic::new(data_dir.to_string_lossy())
        // Set content encoding to utf-8
        .middleware(|_req, res, _suc| {
            if let Some(i) = res.headers.get_mut(HeaderName::ContentType) {
                if i == "text/html" {
                    *i = "text/html; charset=utf-8".into();
                }
            }
        })
        // Response with not found if file is disabled
        .middleware(|req, res, _suc| {
            if is_disabled(&req.path).is_some() {
                *res = not_found(&req.path);
            }
        })
        .not_found(|req, _| not_found(&req.path))
        .attach(server);
}

fn is_disabled(req_path: &str) -> Option<()> {
    if DONT_SERVE.contains(&req_path.to_lowercase())
        || DONT_SERVE.contains(&format!("*.{}", req_path.split('.').last()?))
    {
        return Some(());
    }
    None
}

pub fn not_found(path: &str) -> Response {
    Response::new()
        .status(404)
        .text(
            fs::read_to_string("web/dist/template/not_found.html")
                .unwrap_or_else(|_| "Not Found :/".to_owned())
                .replace("{{VERSION}}", VERSION)
                .replace("{{PAGE}}", path),
        )
        .header(("Content-Type", "text/html"))
}
