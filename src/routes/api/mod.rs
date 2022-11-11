use afire::Server;

use crate::app::App;

mod admin;
mod git;
mod headers;
mod ip;
mod random;
mod random_color;
mod random_nose;
mod raw_http;

pub fn attach(server: &mut Server<App>) {
    admin::attach(server);
    git::attach(server);
    headers::attach(server);
    ip::attach(server);
    random_color::attach(server);
    random::attach(server);
    random_nose::attach(server);
    raw_http::attach(server);
}
