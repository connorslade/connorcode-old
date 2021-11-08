use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/headers", |req| {
        let mut resp = String::new();

        for i in req.headers {
            resp.push_str(i.to_string().as_str());
            resp.push('\n');
        }

        Response::new()
            .text(resp)
            .header(Header::new("Content-Type", "text/plain; charset=utf-8"))
    });
}
