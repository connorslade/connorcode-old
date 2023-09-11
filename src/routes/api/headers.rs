use afire::{Content, Method, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/headers", |ctx| {
        let mut resp = String::new();

        for i in ctx.req.headers.iter() {
            resp.push_str(i.to_string().as_str());
            resp.push('\n');
        }

        ctx.text(resp).content(Content::TXT).send()?;
        Ok(())
    });
}
