use afire::{Method, Server};

use crate::{app::App, serve_static::not_found};

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/r/{code}", |ctx| {
        let code = ctx.param("code").unwrap();
        let links = ctx.app().redirects.read();

        let link = match links.get(code) {
            Some(i) => i,
            None => {
                not_found(&ctx.req.path)
                    .write(ctx.req.socket.clone(), &ctx.server.default_headers)?;
                return Ok(());
            }
        };

        ctx.status(308)
            .reason("Permanent Redirect")
            .text(format!(r#"<a href={link}>{link}</a>"#, link = link))
            .header("Content-Type", "text/html")
            .header("Location", link)
            .send()?;
        Ok(())
    });
}
