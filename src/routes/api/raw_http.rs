use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/rawhttp", |req| {
        Response::new()
            .text(rem_end_null(req.raw_data))
            .header(Header::new("Content-Type", "text/plain; charset=utf-8"))
    });
}

// Remove Trailing Null Chars
fn rem_end_null(str: String) -> String {
    let mut str = str;
    while str.ends_with('\0') {
        str.pop();
    }
    str
}
