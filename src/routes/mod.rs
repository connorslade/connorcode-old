use afire::Server;

mod index;
mod randomimg;

pub fn attach(server: &mut Server) {
    randomimg::attach(server);
    index::attach(server);
}
