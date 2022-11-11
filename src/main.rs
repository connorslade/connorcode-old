use std::time::Duration;

use afire::{Content, Middleware, Response, Server};
#[macro_use]
extern crate lazy_static;

mod routes;
mod serve_static;
mod template;
#[macro_use]
mod color;
mod analytics;
mod app;
mod assets;
mod common;
mod components;
mod config;
mod files;
mod logger;
mod middleware;
mod writing;
use analytics::Analytics;
use app::App;
use color::Color;
use files::Files;
use template::Template;

pub const VERSION: &str = "7.0.0";

fn main() {
    println!(
        "{}",
        color::color_bold(
            format!("[*] Starting Connorcode (V{})", VERSION),
            Color::Green
        )
    );

    // Make app
    let app = App::new();
    let host = app.config.server_host.clone();
    let port = app.config.server_port;

    // Setup HTTP Server
    let mut server = Server::new(&host, port)
        .state(app)
        // Set defult headers
        .default_header("X-Content-Type-Options", "nosniff")
        .default_header("X-Frame-Options", "DENY")
        .default_header("X-Version", format!("Connorcode/{}", VERSION))
        .default_header("X-Server", format!("afire/{}", afire::VERSION))
        // Set other things
        .socket_timeout(Duration::from_secs(5));

    server.error_handler(|_req, err| {
        Response::new()
            .status(500)
            .text(
                Template::new(assets::ERROR_PAGE)
                    .template("VERSION", VERSION)
                    .template("ERROR", &err)
                    .template("ERROR_BODY", err.replace(' ', "+"))
                    .build(),
            )
            .content(Content::HTML)
    });

    let app = server.state.as_ref().unwrap().clone();
    components::attach(&mut server);
    serve_static::attach(&mut server);
    routes::attach(&mut server);
    middleware::attach(&mut server);
    writing::attach(&mut server);
    Files(app.clone()).attach(&mut server);
    Analytics::new(app).attach(&mut server);
    logger::Logger.attach(&mut server);

    // print_info();
    color_print!(Color::Blue, "[*] Starting server on {}:{}\n", &host, port);

    server.start().expect("Server Port In Use");
}

// #[rustfmt::skip]
// fn print_info() {
//     color_print!(Color::Magenta, "[=] Config");
//     color_print!(Color::Magenta, " ├── Analytics");
//     color_print!(Color::Magenta, " │   ├── Enabled: {}", config::ANALYTICS_ENABLED);
//     color_print!(Color::Magenta, " │   ├── Peroid: {}", config::DUMP_PEROID);
//     color_print!(Color::Magenta, " │   └── Serve: {}", config::ANALYTICS_SERVE);
//     color_print!(Color::Magenta, " └── Other");
//     color_print!(Color::Magenta, "     ├── Status Serve: {}", config::STATUS_SERVE);
//     color_print!(Color::Magenta, "     └── Onion Brodcast: {}", config::BROADCAST_ONION);
// }
