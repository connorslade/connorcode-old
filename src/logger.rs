use afire::{
    middleware::{MiddleResult, Middleware},
    Request, extension::RealIp,
};

pub struct Logger;

impl Middleware for Logger {
    fn pre(&self, req: &mut Request) -> MiddleResult {
        let path = match req.path.as_str() {
            "" => "/",
            _ => &req.path,
        };

        println!("[{}] {} {}{}", req.real_ip(), req.method, path, req.query);
        MiddleResult::Continue
    }
}
