use afire::Server;

mod headers;
mod ip;
mod random_color;
mod random_nose;
mod raw_http;

pub fn attach(server: &mut Server) {
    headers::attach(server);
    ip::attach(server);
    random_color::attach(server);
    random_nose::attach(server);
    raw_http::attach(server);
}
