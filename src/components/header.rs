use std::fs;

use afire::{middleware::MiddleResponse, Request, Response};
use regex::Regex;

use super::Component;

lazy_static! {
    static ref HEADER: String =
        fs::read_to_string("data/web/components/header.html").expect("Error Reading Header");
    static ref HEADER_REGEX: Regex =
        Regex::new(r#"\{\{CMP:( )*Header\(".*"\)\}\}"#).expect("Error Initalizing Regex");
}

pub struct Header;

impl Component for Header
where
    Self: Sized,
{
    fn name(&mut self) -> String {
        r#"Header\(".*"\)"#.to_owned()
    }

    fn process(&mut self, _req: Request, res: Response, text: String) -> MiddleResponse {
        let desc = text
            .split_once("Header(\"")
            .unwrap_or_default()
            .1
            .split_once("\")")
            .unwrap_or_default()
            .0;

        MiddleResponse::Add(
            res.text(HEADER_REGEX.replace(&text, &HEADER.replace("{{DESC}}", desc))),
        )
    }
}
