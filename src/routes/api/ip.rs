use afire::{internal::common::remove_address_port, Content, Method, Response, Server};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.route(Method::GET, "/api/ip", |req| {
        let mut ip = remove_address_port(req.address);

        // If Ip is Localhost and 'X-Forwarded-For' Header is present
        // Use that as Ip
        if ip == "127.0.0.1" {
            for i in req.headers {
                if i.name == "X-Forwarded-For" {
                    ip = i.value;
                }
            }
        }

        Response::new().text(ip).content(Content::TXT)
    });
}
