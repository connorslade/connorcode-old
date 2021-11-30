use afire::Server;

mod analytics;
mod status;

pub fn attach(server: &mut Server) {
    analytics::attach(server);
    status::attach(server);
}
