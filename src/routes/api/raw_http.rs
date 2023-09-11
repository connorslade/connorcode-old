use afire::{Content, Method, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::ANY, "/api/rawhttp", |ctx| {
        let mut out = format!("{} {} {}\n", ctx.req.method, ctx.req.path, ctx.req.version);

        for i in ctx.req.headers.iter() {
            out.push_str(i.to_string().as_str());
            out.push('\n');
        }

        let mut out = out.as_bytes().to_vec();
        out.push(b'\n');
        out.extend_from_slice(&ctx.req.body);

        ctx.bytes(out).content(Content::TXT).send()?;
        Ok(())
    });
}
