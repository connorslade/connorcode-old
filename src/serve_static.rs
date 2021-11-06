use afire::*;
use std::fs;

use crate::VERSION;

/// Dir to find files to serve
const DATA_DIR: &str = "./data/static";

/// Disabled files
const DISABLED_FILES: &[&str] = &[];

pub fn attach(server: &mut afire::Server) {
    server.all(|req| {
        if DISABLED_FILES.contains(&req.path.as_str()) {
            return Response::new()
                .status(404)
                .text("Not Found")
                .header(Header::new("Content-Type", "text/html"));
        }

        let mut path = format!("{}{}", DATA_DIR, req.path.replace("/..", ""));

        // Add Index.html if path ends with /
        if path.ends_with('/') {
            path.push_str("index.html");
        }

        // Also add '/index.html' if path dose not end with a file
        if !path.split('/').last().unwrap_or_default().contains('.') {
            path.push_str("/index.html");
        }

        // Try to read File
        match fs::read(&path) {
            // If its found send it as response
            Ok(content) => Response::new()
                .bytes(add_version(content))
                .header(Header::new("Content-Type", get_type(&path))),

            // If not send 404.html
            Err(_) => Response::new()
                .status(404)
                .bytes(
                    fs::read(format!("{}/404.html", DATA_DIR))
                        .unwrap_or_else(|_| "Not Found :/".as_bytes().to_owned()),
                )
                .header(Header::new("Content-Type", "text/html")),
        }
    });
}

/// Get the type MMIE content type of a file from its extension
pub fn get_type(path: &str) -> &str {
    match path.split('.').last() {
        Some(ext) => match ext {
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "ico" => "image/x-icon",
            "svg" => "image/svg+xml",
            _ => "application/octet-stream",
        },

        None => "application/octet-stream",
    }
}

fn add_version(content: Vec<u8>) -> Vec<u8> {
    match String::from_utf8(content.clone()) {
        Ok(i) => return i.replace("{{VERSION}}", VERSION).into_bytes(),
        Err(_) => return content,
    }
}
