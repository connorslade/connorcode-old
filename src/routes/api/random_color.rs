use std::fs;

use afire::Content;
use afire::Method;
use afire::Response;
use afire::Server;
use rand::seq::SliceRandom;

use crate::app::App;

const COLORS: [&str; 11] = [
    "Red", "Orange", "Yellow", "Green", "Blue", "Purple", "Pink", "Brown", "Black", "Grey", "White",
];

pub fn attach(server: &mut Server<App>) {
    let data_dir = &server.state.as_ref().unwrap().config.data_dir;
    let raw = fs::read_to_string(data_dir.join("colornamegen/words.txt"))
        .expect("Error Reading Words File");
    let words = raw.lines().map(|x| x.to_owned()).collect::<Vec<_>>();

    server.route(Method::GET, "/api/randomcolor", move |_req| {
        let random_name = words
            .choose(&mut rand::thread_rng())
            .expect("Error Picking Word");
        let random_color = COLORS
            .choose(&mut rand::thread_rng())
            .expect("Error Picking Color");

        Response::new()
            .text(format!("{} {}", random_name, random_color))
            .content(Content::TXT)
    });
}
