use std::fs;
use std::path::PathBuf;

use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;
use rand::seq::SliceRandom;

use crate::serve_static::get_type;

static mut NOSES: Vec<PathBuf> = Vec::new();

pub fn attach(server: &mut Server) {
    let all_noses = fs::read_dir("data/static/assets/Noses").unwrap();

    for nose in all_noses {
        let nose = nose.unwrap().path();

        if nose.is_file() {
            unsafe { NOSES.push(nose) };
        }
    }

    server.route(Method::GET, "/api/randomnose", |_req| {
        let noses = unsafe { NOSES.clone() };
        let random_nose = noses.choose(&mut rand::thread_rng()).unwrap();
        let random_nose_str = random_nose.to_str().unwrap().replace('\\', "");
        let random_nose_str = random_nose_str.split('/').last().unwrap();

        Response::new()
            .bytes(fs::read(random_nose).unwrap())
            .header(Header::new("Content-Type", get_type(&random_nose_str)))
            .header(Header::new("X-Nose-ID", random_nose_str))
    });
}
