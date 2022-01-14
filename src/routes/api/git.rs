use afire::{Content, Method, Response, Server};

pub fn attach(server: &mut Server) {
    server.route(Method::GET, "/api/git", |_req| {
        Response::new().text(env!("GIT_INFO")).content(Content::TXT)
    });
}
