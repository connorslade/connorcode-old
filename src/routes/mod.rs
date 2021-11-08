use afire::Server;

mod api;
mod index;
mod randomimg;

pub fn attach(server: &mut Server) {
    index::attach(server);
    api::attach(server);
    randomimg::attach(server);
}
