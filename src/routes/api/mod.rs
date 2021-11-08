use afire::Server;

mod headers;
mod ip;
mod random_nose;

pub fn attach(server: &mut Server) {
    ip::attach(server);
    headers::attach(server);
    random_nose::attach(server);
}
