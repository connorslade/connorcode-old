use std::any::type_name;

use afire::{
    error::Result,
    internal::common::trace,
    middleware::{MiddleResponse, Middleware},
    Request, Response, Server,
};

use crate::config::{BROADCAST_ONION, ONION_SITE};

pub struct Onion;

impl Onion {
    pub fn new() -> Self {
        Onion
    }
}

impl Middleware for Onion {
    fn post(&self, req: &Result<Request>, res: &Result<Response>) -> MiddleResponse {
        let req = match req {
            Ok(i) => i,
            Err(_) => return MiddleResponse::Continue,
        };
        let res = match res {
            Ok(i) => i,
            Err(_) => return MiddleResponse::Continue,
        };

        MiddleResponse::Add(
            res.to_owned()
                .header("Onion-Location", format!("{}{}", *ONION_SITE, req.path)),
        )
    }

    fn attach<State>(self, server: &mut Server<State>)
    where
        Self: 'static + Send + Sync + Sized,
        State: 'static + Send + Sync,
    {
        if !*BROADCAST_ONION {
            return;
        }

        trace(format!("📦 Adding Middleware {}", type_name::<Self>()));

        server.middleware.push(Box::new(self));
    }
}
