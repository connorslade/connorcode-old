use afire::Server;

mod headers;
mod ip;
mod random_nose;
mod raw_http;

pub fn attach(server: &mut Server) {
    ip::attach(server);
    headers::attach(server);
    random_nose::attach(server);
    raw_http::attach(server);
}
