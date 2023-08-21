use std::fs::File;

use afire::{route::RouteContext, Method, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/key", |ctx| {
        ctx.stream(
            File::open(ctx.app().config.data_dir.join("key.asc"))
                .context("Error Reading key file")?,
        )
        .send()?;
        Ok(())
    });
}
