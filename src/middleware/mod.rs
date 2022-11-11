use afire::{Middleware, Server};

use crate::app::App;

mod cache_header;
mod onion;

pub fn attach(server: &mut Server<App>) {
    let app = server.state.as_ref().unwrap().clone();
    onion::Onion(app).attach(server);
    cache_header::Cache.attach(server);
}
