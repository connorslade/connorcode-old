use std::fs;

use afire::Method;
use afire::Response;
use afire::Server;
use rand::seq::SliceRandom;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    let mut noses = Vec::new();
    let data_dir = &server.state.as_ref().unwrap().config.data_dir;

    let all_noses = fs::read_dir(data_dir.join("assets/Noses")).expect("Error Reading Nose Dir");

    for nose in all_noses {
        let nose = nose.expect("Error getting subfiles").path();

        if nose.is_file() {
            noses.push(nose);
        }
    }

    server.route(Method::GET, "/api/randomnose", move |_req| {
        let random_nose = noses
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
    path.rsplit_once('.')
        .map(|x| match x.1 {
            "png" => "image/png",
            "jpg" => "image/jpeg",
            "jpeg" => "image/jpeg",
            "gif" => "image/gif",

            _ => "application/octet-stream",
        })
        .unwrap_or("application/octet-stream")
}
