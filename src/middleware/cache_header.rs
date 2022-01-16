use afire::{
    middleware::{MiddleResponse, Middleware},
    Header, Request, Response,
};

const STATIC_CACHE: &[&str] = &["woff", "woff2", "png"];
const STATIC_CACHE_LEN: u32 = 31536000;
const CACHE_LEN: u32 = 3600;

pub struct Cache;

impl Cache {
    pub fn new() -> Self {
        Cache
    }
}

impl Middleware for Cache {
    fn post(&mut self, req: Request, res: Response) -> MiddleResponse {
        if let Some(i) = req.path.rsplit_once('.') {
            if STATIC_CACHE.contains(&i.1) {
                return MiddleResponse::Add(res.header(Header::new(
                    "Cache-Control",
                    format!("max-age={STATIC_CACHE_LEN}"),
                )));
            }
        }

        MiddleResponse::Add(
            res.header(Header::new("Cache-Control", format!("max-age={CACHE_LEN}"))),
        )
    }
}
