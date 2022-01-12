use std::env;
use std::time::Duration;

use afire::{Header, Logger, Middleware, Response, Server};
#[macro_use]
extern crate lazy_static;

mod arg_parse;
mod routes;
mod serve_static;
mod template;
#[macro_use]
mod color;
mod analytics;
mod assets;
mod common;
mod config;
mod files;
mod middleware;
use analytics::Analytics;
use color::Color;
use config::{SERVER_HOST, SERVER_PORT};
use files::Files;
use middleware::Onion;
use template::Template;

pub const VERSION: &str = "6.0.0";

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_file: &str =
        arg_parse::get_arg_value(&args, "--config").unwrap_or("./data/config/config.cfg");

    config::load(config_file).expect("Error Reading Config");

    println!(
        "{}",
        color::color_bold(
            format!("[*] Starting Connorcode (V{})", VERSION),
            Color::Green
        )
    );

    let mut server = Server::new(&*SERVER_HOST, *SERVER_PORT)
        // Set defult headers
        .default_header(Header::new("X-Content-Type-Options", "nosniff"))
        .default_header(Header::new("X-Frame-Options", "DENY"))
        .default_header(Header::new("X-Version", format!("Connorcode/{}", VERSION)))
        .default_header(Header::new("Cache-Control", "max-age=3600"))
        // Set other things
        .default_header(Header::new("X-Server", format!("afire/{}", afire::VERSION)))
        .socket_timeout(Duration::from_secs(5));

    server.error_handler(|_req, err| {
        Response::new()
            .status(500)
            .text(
                Template::new(assets::ERROR_PAGE)
                    .template("VERSION", VERSION)
                    .template("ERROR", err)
                    .build(),
            )
            .header(Header::new("Content-Type", "text/html"))
    });

    // Serve Static Files
    serve_static::attach(&mut server);

    // Add Api Routes
    routes::attach(&mut server);

    // Add Logger, Analytics, Onion Brodcast and Files
    Files::new().attach(&mut server);
    Onion::new().attach(&mut server);
    Analytics::new().attach(&mut server);
    Logger::new().attach(&mut server);

    print_info();
    color_print!(
        Color::Blue,
        "[*] Starting server on {}:{}\n",
        SERVER_HOST,
        SERVER_PORT
    );

    server.start().unwrap();
}

#[rustfmt::skip]
fn print_info() {
    color_print!(Color::Magenta, "[=] Config");
    color_print!(Color::Magenta, " ├── Analytics");
    color_print!(Color::Magenta, " │   ├── Enabled: {}", config::ANALYTICS_ENABLED);
    color_print!(Color::Magenta, " │   ├── Peroid: {}", config::DUMP_PEROID);
    color_print!(Color::Magenta, " │   └── Serve: {}", config::ANALYTICS_SERVE);
    color_print!(Color::Magenta, " └── Other");
    color_print!(Color::Magenta, "     ├── Status Serve: {}", config::STATUS_SERVE);
    color_print!(Color::Magenta, "     └── Onion Brodcast: {}", config::BROADCAST_ONION);
}
