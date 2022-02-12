use afire::{Header, Method, Response, Server};

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/rawhttp", |req| {
        Response::new()
            .text(String::from_utf8_lossy(&req.raw_data).to_string())
            .header(Header::new("Content-Type", "text/plain; charset=utf-8"))
    });
}
