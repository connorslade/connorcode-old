use afire::Server;

mod randomimg;

pub fn attach(server: &mut Server) {
    randomimg::attach(server);
}
