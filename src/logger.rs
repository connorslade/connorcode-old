use afire::{
    middleware::{MiddleRequest, Middleware},
    Request,
};

use crate::common::get_ip;

pub struct Logger;

impl Middleware for Logger {
    fn pre(&mut self, req: Request) -> MiddleRequest {
        let path = match req.path.as_str() {
            "" => "/",
            _ => &req.path,
        };

        println!(
            "[{}] {} {}{}",
            get_ip(&req),
            req.method.to_string(),
            path,
            req.query.to_string()
        );

        MiddleRequest::Continue
    }
}
