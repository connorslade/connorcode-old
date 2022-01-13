use std::fs;

use afire::{middleware::MiddleResponse, Request, Response};

use super::Component;

lazy_static! {
    static ref FOOTER: String =
        fs::read_to_string("data/web/components/footer.html").expect("Error Reading Footer");
}

pub struct Footer;

impl Component for Footer
where
    Self: Sized,
{
    fn name(&mut self) -> String {
        "Footer".to_owned()
    }

    fn process(&mut self, _req: Request, res: Response, text: String) -> MiddleResponse {
        MiddleResponse::Add(res.text(text.replace(
            "{{CMP: Footer}}",
            &FOOTER.replace("{{VERSION}}", crate::VERSION),
        )))
    }
}
