use afire::{
    error::Result,
    middleware::{MiddleResponse, Middleware},
    Request, Response,
};

const STATIC_CACHE: &[&str] = &["woff", "woff2", "png", "webp"];
const STATIC_CACHE_LEN: u32 = 31536000;
const CACHE_LEN: u32 = 3600;

pub struct Cache;

impl Middleware for Cache {
    fn post(&self, req: &Result<Request>, res: &Result<Response>) -> MiddleResponse {
        let req = match req {
            Ok(i) => i,
            Err(_) => return MiddleResponse::Continue,
        };
        let res = match res {
            Ok(i) => i,
            Err(_) => return MiddleResponse::Continue,
        };

        if let Some(i) = req.path.rsplit_once('.') {
            if STATIC_CACHE.contains(&i.1) {
                return MiddleResponse::Add(
                    res.to_owned()
                        .header("Cache-Control", format!("max-age={}", STATIC_CACHE_LEN)),
                );
            }
        }

        MiddleResponse::Add(
            res.to_owned()
                .header("Cache-Control", format!("max-age={}", CACHE_LEN)),
        )
    }
}
