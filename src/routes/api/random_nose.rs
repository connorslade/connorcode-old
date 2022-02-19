use std::fs;
use std::path::PathBuf;

use afire::Method;
use afire::Response;
use afire::Server;
use rand::seq::SliceRandom;

use crate::config::DATA_DIR;

lazy_static! {
    static ref NOSES: Vec<PathBuf> = {
        let mut working = Vec::new();
        let all_noses =
            fs::read_dir(format!("{}/assets/Noses", *DATA_DIR)).expect("Error Reading Nose Dir");

        for nose in all_noses {
            let nose = nose.expect("Error getting subfiles").path();

            if nose.is_file() {
                working.push(nose);
            }
        }

        working
    };
}

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/randomnose", |_req| {
        let random_nose = NOSES
            .choose(&mut rand::thread_rng())
            .expect("Error Picking Nose");
        let random_nose_str = random_nose.to_string_lossy().replace('\\', "");
        let random_nose_str = random_nose_str
            .split('/')
            .last()
            .expect("Error Spliting on Slash");

        Response::new()
            .bytes(fs::read(random_nose).expect("Error Reading Nose"))
            .header("Content-Type", get_type(random_nose_str))
            .header("X-Nose-ID", random_nose_str)
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
