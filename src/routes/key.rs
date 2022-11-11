use std::fs;

use afire::{Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/key", |app, _req| {
        Response::new().bytes(
            fs::read(format!("{}/key.asc", app.config.data_dir)).expect("Error Reading key file"),
        )
    })
}
