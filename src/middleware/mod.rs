use afire::{Middleware, Server};

mod cache_header;
mod onion;

pub fn attach(server: &mut Server) {
    onion::Onion::new().attach(server);
    cache_header::Cache::new().attach(server);
}
