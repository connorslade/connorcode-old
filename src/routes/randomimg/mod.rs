use afire::Server;

mod get_image;

pub fn attach(server: &mut Server) {
    get_image::attach(server);
}
