use std::sync::Mutex;

use afire::{
    error::Result,
    middleware::{MiddleResponse, Middleware},
    Request, Response, Server,
};
use regex::Regex;

use crate::app::App;

mod footer;
mod footer_rss;
mod header;

struct ComponentManager {
    components: Mutex<Vec<(String, Box<dyn Component + Send + Sync>)>>,
}

pub trait Component {
    fn name(&mut self) -> String;
    fn process(&mut self, req: Request, res: Response, text: String) -> MiddleResponse;
}

impl Middleware for ComponentManager {
    fn post(&self, req: &Result<Request>, res: &Result<Response>) -> MiddleResponse {
        let req = match req {
            Ok(i) => i.to_owned(),
            Err(_) => return MiddleResponse::Continue,
        };
        let mut res = match res {
            Ok(i) => i.to_owned(),
            Err(_) => return MiddleResponse::Continue,
        };

        let mut components = self.components.lock().unwrap();

        for i in components.iter_mut() {
            let text = match String::from_utf8(res.data.clone()) {
                Ok(i) => i,
                Err(_) => return MiddleResponse::Continue,
            };

            if Regex::new(&format!(r#"\{{\{{CMP:( )*{}\}}\}}"#, &i.0))
                .expect("Error parseing Regex")
                .is_match(&text)
            {
                match (i.1).process(req.clone(), res.clone(), text.to_string()) {
                    MiddleResponse::Continue => {}
                    MiddleResponse::Add(i) => res = i,
                    MiddleResponse::Send(i) => return MiddleResponse::Send(i),
                }
            }
        }

        MiddleResponse::Add(res)
    }
}

impl ComponentManager {
    fn new() -> Self {
        ComponentManager {
            components: Mutex::new(Vec::new()),
        }
    }

    fn add(&self, mut cmp: Box<dyn Component + Send + Sync>) {
        self.components.lock().unwrap().push((cmp.name(), cmp));
    }
}

pub fn attach(server: &mut Server<App>) {
    let cmp = ComponentManager::new();
    cmp.add(Box::new(footer::Footer));
    cmp.add(Box::new(footer_rss::FooterRss));
    cmp.add(Box::new(header::Header));

    cmp.attach(server);
}
