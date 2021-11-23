use std::fs;

use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;
use rand::seq::SliceRandom;

lazy_static! {
    static ref WORDS: Vec<String> = {
        let raw = fs::read_to_string("data/static/colornamegen/words.txt").unwrap();
        raw.lines().map(|x| x.to_owned()).collect()
    };
}

const COLORS: [&str; 11] = [
    "Red", "Orange", "Yellow", "Green", "Blue", "Purple", "Pink", "Brown", "Black", "Grey", "White",
];

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/randomcolor", |_req| {
        let random_name = WORDS.choose(&mut rand::thread_rng()).unwrap();
        let random_color = COLORS.choose(&mut rand::thread_rng()).unwrap();

        Response::new()
            .text(format!("{} {}", random_name, random_color))
            .header(Header::new("Content-Type", "text/plain"))
    });
}
