use std::sync::Arc;

use afire::{
    middleware::{MiddleResult, Middleware},
    trace, Header, Request, Response, Server,
};

use crate::app::App;

pub struct Onion(pub Arc<App>);

impl Middleware for Onion {
    fn post(&self, req: &Request, res: &mut Response) -> MiddleResult {
        res.headers.push(Header::new(
            "Onion-Location",
            format!("{}{}", self.0.config.onion_site, req.path),
        ));

        MiddleResult::Continue
    }

    fn attach<State>(self, server: &mut Server<State>)
    where
        Self: 'static + Send + Sync + Sized,
        State: 'static + Send + Sync,
    {
        if !self.0.config.broadcast_onion {
            return;
        }

        trace!("📦 Adding Middleware {}", type_name::<Self>());

        server.middleware.push(Box::new(self));
    }
}
