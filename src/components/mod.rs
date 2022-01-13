use afire::{
    middleware::{MiddleResponse, Middleware},
    Request, Response, Server,
};
use regex::Regex;

mod footer;
mod header;

struct ComponentManager {
    components: Vec<(String, Box<dyn Component>)>,
}

pub trait Component {
    fn name(&mut self) -> String;
    fn process(&mut self, req: Request, res: Response, text: String) -> MiddleResponse;
}

impl Middleware for ComponentManager {
    fn post(&mut self, req: Request, mut res: Response) -> MiddleResponse {
        for i in &mut self.components {
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
            components: Vec::new(),
        }
    }

    fn add(&mut self, mut cmp: Box<dyn Component>) {
        self.components.push((cmp.name(), cmp));
    }
}

pub fn attach(server: &mut Server) {
    let mut cmp = ComponentManager::new();
    cmp.add(Box::new(footer::Footer));
    cmp.add(Box::new(header::Header));

    cmp.attach(server);
}
