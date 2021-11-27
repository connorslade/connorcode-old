use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/ip", |req| {
        let mut ip = get_ip(req.address);

        // If Ip is Localhost and 'X-Forwarded-For' Header is present
        // Use that as Ip
        if ip == "127.0.0.1" {
            for i in req.headers {
                if i.name == "X-Forwarded-For" {
                    ip = i.value;
                }
            }
        }

        Response::new()
            .text(ip)
            .header(Header::new("Content-Type", "text/plain"))
    });
}

fn get_ip(addr: String) -> String {
    addr.split(':')
        .next()
        .expect("Error Getting Ip. No Idea now that happend but it did.")
        .to_owned()
}
