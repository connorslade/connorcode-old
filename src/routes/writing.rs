use afire::{Content, Method, Response, Server};

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/writing", |_req| {
        Response::new()
            .text(
                r#"[
            {"name": "Hello, World", "disc": "First Page!", "date": "01-10-22", "link": "https://duck.com"}
        ]"#,
            )
            .content(Content::JSON)
    });
}
