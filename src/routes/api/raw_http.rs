use afire::{Header, Method, Response, Server};

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/rawhttp", |req| {
        Response::new()
            .text(rem_end_null(
                String::from_utf8_lossy(&req.raw_data).to_string(),
            ))
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
