use std::fs::File;

use afire::{Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/key", |app, _req| {
        Response::new().stream(
            File::open(app.config.data_dir.join("key.asc")).expect("Error Reading key file"),
        )
    });
}
