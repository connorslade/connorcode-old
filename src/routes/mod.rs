use afire::Server;

mod api;
mod index;
mod key;
mod randomimg;

pub fn attach(server: &mut Server) {
    index::attach(server);
    api::attach(server);
    key::attach(server);
    randomimg::attach(server);
}
