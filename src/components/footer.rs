use afire::{middleware::MiddleResponse, Request, Response};

use crate::assets::FOOTER;

use super::Component;

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
