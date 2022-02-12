use std::fs;

use afire::{Method, Response, Server};

use crate::config::DATA_DIR;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/key", |_req| {
        Response::new()
            .bytes(fs::read(format!("{}/key.asc", *DATA_DIR)).expect("Error Reading key file"))
    })
}
