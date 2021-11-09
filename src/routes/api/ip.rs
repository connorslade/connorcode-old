use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/ip", |req| {
        Response::new()
            .text(get_ip(req.address))
            .header(Header::new("Content-Type", "text/plain"))
    });
}

fn get_ip(addr: String) -> String {
    addr.split(':')
        .next()
        .expect("Error Getting Ip. No Idea now that happend but it did.")
        .to_owned()
}
