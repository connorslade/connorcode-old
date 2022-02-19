use std::fs;


use afire::Method;
use afire::Response;
use afire::Server;
use rand::seq::SliceRandom;

use crate::config::DATA_DIR;

lazy_static! {
    static ref WORDS: Vec<String> = {
        let raw = fs::read_to_string(format!("{}/colornamegen/words.txt", *DATA_DIR))
            .expect("Error Reading Words File");
        raw.lines().map(|x| x.to_owned()).collect()
    };
}

const COLORS: [&str; 11] = [
    "Red", "Orange", "Yellow", "Green", "Blue", "Purple", "Pink", "Brown", "Black", "Grey", "White",
];

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/randomcolor", |_req| {
        let random_name = WORDS
            .choose(&mut rand::thread_rng())
            .expect("Error Picking Word");
        let random_color = COLORS
            .choose(&mut rand::thread_rng())
            .expect("Error Picking Color");

        Response::new()
            .text(format!("{} {}", random_name, random_color))
            .header("Content-Type", "text/plain")
    });
}
