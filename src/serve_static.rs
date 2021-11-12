use afire::*;
use std::fs;

use crate::VERSION;

/// Dir to find files to serve
const DATA_DIR: &str = "./data/static";

/// Files not to serve
const DONT_SERVE_PATH: &str = "data/config/dont_serve.txt";

/// Disabled files
static mut DISABLED_FILES: Vec<String> = vec![];

pub fn attach(server: &mut afire::Server) {
    let dont_serve = fs::read_to_string(DONT_SERVE_PATH).unwrap();
    for line in dont_serve.lines() {
        if line.is_empty() || line.starts_with('#') || line.starts_with(';') {
            continue;
        }

        unsafe {
            DISABLED_FILES.push(line.to_owned());
        }
    }

    server.all(|req| {
        if ip_disabled(&req.path).is_some() {
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

fn ip_disabled(req_path: &str) -> Option<()> {
    let disabled = unsafe { DISABLED_FILES.clone() };
    if disabled.contains(&req_path.to_lowercase())
        || disabled.contains(&format!("*.{}", req_path.split('.').last()?))
    {
        return Some(());
    }
    None
}

fn add_version(content: Vec<u8>) -> Vec<u8> {
    match String::from_utf8(content.clone()) {
        Ok(i) => return i.replace("{{VERSION}}", VERSION).into_bytes(),
        Err(_) => return content,
    }
}

/// Get the type MMIE content type of a file from its extension
pub fn get_type(path: &str) -> &str {
    match path.split('.').last() {
        Some(ext) => match ext {
            // More Common Types
            "html" => "text/html",
            "css" => "text/css",
            "js" => "application/javascript",
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "gif" => "image/gif",
            "ico" => "image/x-icon",
            "svg" => "image/svg+xml",
            "txt" => "text/plain",

            // Lots more types
            "aac" => "audio/aac",
            "avi" => "video/x-msvideo",
            "bin" => "application/octet-stream",
            "bmp" => "image/bmp",
            "bz" => "application/x-bzip",
            "bz2" => "application/x-bzip2",
            "cda" => "application/x-cdf",
            "csv" => "text/csv",
            "doc" => "application/msword",
            "docx" => "application/vnd.openxmlformats-officedocument.wordprocessingml.document",
            "eot" => "application/vnd.ms-fontobject",
            "epub" => "application/epub+zip",
            "gz" => "application/gzip",
            "htm" => "text/html",
            "ics" => "text/calendar",
            "jar" => "application/java-archive",
            "json" => "application/json",
            "jsonld" => "application/ld+json",
            "midi" => "audio/midi audio/x-midi",
            "mid" => "audio/midi audio/x-midi",
            "mjs" => "text/javascript",
            "mp3" => "audio/mpeg",
            "mp4" => "video/mp4",
            "mpeg" => "video/mpeg",
            "mpkg" => "application/vnd.apple.installer+xml",
            "odp" => "application/vnd.oasis.opendocument.presentation",
            "ods" => "application/vnd.oasis.opendocument.spreadsheet",
            "odt" => "application/vnd.oasis.opendocument.text",
            "oga" => "audio/ogg",
            "ogv" => "video/ogg",
            "ogx" => "application/ogg",
            "opus" => "audio/opus",
            "otf" => "font/otf",
            "pdf" => "application/pdf",
            "php" => "application/x-httpd-php",
            "ppt" => "application/vnd.ms-powerpoint",
            "pptx" => "application/vnd.openxmlformats-officedocument.presentationml.presentation",
            "rar" => "application/vnd.rar",
            "rtf" => "application/rtf",
            "sh" => "application/x-sh",
            "swf" => "application/x-shockwave-flash",
            "tar" => "application/x-tar",
            "tif" => "image/tiff",
            "tiff" => "image/tiff",
            "ts" => "text/x-typescript",
            "ttf" => "font/ttf",
            "wav" => "audio/wav",
            "weba" => "audio/webm",
            "webm" => "video/webm",
            "webp" => "image/webp",
            "woff" => "font/woff",
            "woff2" => "font/woff2",
            "xhtml" => "application/xhtml+xml",
            "xls" => "application/vnd.ms-excel",
            "xlsx" => "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "xml" => "application/xml",
            "zip" => "application/zip",
            "7z" => "application/x-7z-compressed",
            _ => "application/octet-stream",
        },

        None => "application/octet-stream",
    }
}
