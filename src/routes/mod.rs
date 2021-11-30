use afire::Server;

mod api;
mod index;
mod key;
mod link;
mod randomimg;

pub fn attach(server: &mut Server) {
    index::attach(server);
    api::attach(server);
    key::attach(server);
    link::attach(server);
    randomimg::attach(server);
}
