use std::fs;

use afire::{middleware::MiddleResponse, Request, Response};

use super::Component;

lazy_static! {
    static ref FOOTER: String =
        fs::read_to_string("data/web/components/footer_rss.html").expect("Error Reading Footer");
}

pub struct FooterRss;

impl Component for FooterRss
where
    Self: Sized,
{
    fn name(&mut self) -> String {
        "FooterRss".to_owned()
    }

    fn process(&mut self, _req: Request, res: Response, text: String) -> MiddleResponse {
        MiddleResponse::Add(res.text(text.replace(
            "{{CMP: FooterRss}}",
            &FOOTER.replace("{{VERSION}}", crate::VERSION),
        )))
    }
}
