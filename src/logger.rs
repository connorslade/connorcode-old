use afire::{
    error::Result,
    middleware::{MiddleRequest, Middleware},
    Request,
};

use crate::common::get_ip;

pub struct Logger;

impl Middleware for Logger {
    fn pre(&self, req: &Result<Request>) -> MiddleRequest {
        let req = match req {
            Ok(i) => i,
            Err(_) => return MiddleRequest::Continue,
        };

        let path = match req.path.as_str() {
            "" => "/",
            _ => &req.path,
        };

        println!("[{}] {} {}{}", get_ip(req), req.method, path, req.query);

        MiddleRequest::Continue
    }
}
