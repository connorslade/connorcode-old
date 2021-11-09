use std::env;
use std::time::Duration;

use afire::{Header, Response, Server};
use simple_config_parser::Config;

mod arg_parse;
mod routes;
mod serve_static;
#[macro_use]
mod color;
use color::Color;

pub const VERSION: &str = "5.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file: &str =
        arg_parse::get_arg_value(&args, "--config").unwrap_or("./data/config/config.cfg");

    println!(
        "{}",
        color::color_bold(
            format!("[*] Starting Connorcode (V{})", VERSION),
            Color::Green
        )
    );

    let cfg = Config::new()
        .file(config_file)
        .expect("Error reading the config file");

    let host = cfg.get_str("ip").unwrap();
    let port = cfg.get::<u16>("port").unwrap();

    let mut server = Server::new(&host, port);

    server.set_error_handler(|_req, err| {
        Response::new()
            .status(500)
            .text(format!(
                "<h1>Internal Server Error</h1><p>Error: {}</p>",
                err
            ))
            .header(Header::new("Content-Type", "text/html"))
    });

    // TMP
    server.middleware(Box::new(|req| {
        println!("[{}] {}", req.method, req.path);

        None
    }));

    // Set defult headers
    server.add_default_header(Header::new("X-Content-Type-Options", "nosniff"));
    server.add_default_header(Header::new("X-Frame-Options", "DENY"));
    server.add_default_header(Header::new("X-Version", format!("Connorcode/{}", VERSION)));
    server.add_default_header(Header::new("X-Server", "afire/0.2.0"));
    server.set_socket_timeout(Some(Duration::from_secs(1)));

    serve_static::attach(&mut server);
    routes::attach(&mut server);

    color_print!(Color::Blue, "[*] Starting server on {}:{}\n", host, port);

    server.start().unwrap();
}
