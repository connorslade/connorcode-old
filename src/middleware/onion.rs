use std::cell::RefCell;

use afire::{
    middleware::{MiddleResponse, Middleware},
    Header, Request, Response, Server,
};

use crate::config::{BROADCAST_ONION, ONION_SITE};

pub struct Onion;

impl Onion {
    pub fn new() -> Self {
        Onion
    }
}

impl Middleware for Onion {
    fn post(&mut self, req: Request, res: Response) -> MiddleResponse {
        MiddleResponse::Add(res.header(Header::new(
            "Onion-Location",
            format!("{}{}", *ONION_SITE, req.path),
        )))
    }

    fn attach(self, server: &mut Server) {
        if !*BROADCAST_ONION {
            return;
        }

        server.middleware.push(Box::new(RefCell::new(self)));
    }
}
