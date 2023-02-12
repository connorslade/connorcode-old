use afire::{
    middleware::{MiddleResult, Middleware},
    Header, Request, Response,
};

const STATIC_CACHE: &[&str] = &["woff", "woff2", "png", "webp"];
const STATIC_CACHE_LEN: u32 = 31536000;
const CACHE_LEN: u32 = 3600;

pub struct Cache;

impl Middleware for Cache {
    fn post(&self, req: &Request, res: &mut Response) -> MiddleResult {
        if let Some(i) = req.path.rsplit_once('.') {
            if STATIC_CACHE.contains(&i.1) {
                res.headers.push(Header::new(
                    "Cache-Control",
                    format!("max-age={}", STATIC_CACHE_LEN),
                ));
                return MiddleResult::Continue;
            }
        }

        res.headers.push(Header::new(
            "Cache-Control",
            format!("max-age={}", CACHE_LEN),
        ));
        MiddleResult::Continue
    }
}
