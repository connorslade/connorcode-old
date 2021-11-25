use std::fs;
use std::path::PathBuf;

use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;
use rand::seq::SliceRandom;

use crate::DATA_DIR;

lazy_static! {
    static ref NOSES: Vec<PathBuf> = {
        let mut working = Vec::new();
        let all_noses = fs::read_dir(format!("{}/assets/Noses", DATA_DIR)).unwrap();

        for nose in all_noses {
            let nose = nose.unwrap().path();

            if nose.is_file() {
                working.push(nose);
            }
        }

        working
    };
}

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/randomnose", |_req| {
        let random_nose = NOSES.choose(&mut rand::thread_rng()).unwrap();
        let random_nose_str = random_nose.to_str().unwrap().replace('\\', "");
        let random_nose_str = random_nose_str.split('/').last().unwrap();

        Response::new()
            .bytes(fs::read(random_nose).unwrap())
            .header(Header::new("Content-Type", get_type(random_nose_str)))
            .header(Header::new("X-Nose-ID", random_nose_str))
    });
}

/// Get MIME type for common image formats
fn get_type(path: &str) -> &str {
    match path.split('.').last() {
        Some(ext) => match ext {
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "gif" => "image/gif",

            _ => "application/octet-stream",
        },

        None => "application/octet-stream",
    }
}
