use std::any::type_name;

use afire::{
    middleware::{MiddleResponse, Middleware},
    trace, Request, Response, Server,
};

use crate::config::{BROADCAST_ONION, ONION_SITE};

pub struct Onion;

impl Onion {
    pub fn new() -> Self {
        Onion
    }
}

impl Middleware for Onion {
    fn post(&self, req: Request, res: Response) -> MiddleResponse {
        MiddleResponse::Add(res.header("Onion-Location", format!("{}{}", *ONION_SITE, req.path)))
    }

    fn attach(self, server: &mut Server) {
        if !*BROADCAST_ONION {
            return;
        }

        trace!("📦 Adding Middleware {}", type_name::<Self>());

        server.middleware.push(Box::new(self));
    }
}
