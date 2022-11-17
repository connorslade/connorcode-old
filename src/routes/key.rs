use std::fs;

use afire::{Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/key", |app, _req| {
        Response::new()
            .bytes(fs::read(app.config.data_dir.join("key.asc")).expect("Error Reading key file"))
    })
}
