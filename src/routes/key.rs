use std::fs;

use afire::{Method, Response, Server};

use crate::DATA_DIR;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/key", |_req| {
        Response::new().bytes(fs::read(format!("{}/key.asc", DATA_DIR)).unwrap())
    })
}
