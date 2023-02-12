use std::{process, sync::Arc, time::Duration};

use afire::{
    trace::{set_log_level, Level},
    Content, Middleware, Response, Server,
};
#[macro_use]
extern crate lazy_static;

mod routes;
mod serve_static;
#[macro_use]
mod color;
mod analytics;
mod app;
mod assets;
mod common;
mod config;
mod control;
mod ctrlc;
mod files;
mod logger;
mod middleware;
mod writing;
use analytics::Analytics;
use app::App;
use color::Color;
use files::Files;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    set_log_level(Level::Trace);
    println!(
        "{}",
        color::color_bold(
            format!("[*] Starting Connorcode (V{})", VERSION),
            Color::Green
        )
    );

    let app = App::new();
    if !app.config.data_dir.join("..").exists() {
        color_print!(Color::Red, "[!] Data Directory Does Not Exist");
        process::exit(1);
    }

    // Make app
    let host = app.config.server_host.clone();
    let port = app.config.server_port;
    let threads = app.config.threads;

    // Setup HTTP Server
    let mut server = Server::new(host.as_str(), port)
        .state(app)
        // Set default headers
        .default_header("X-Content-Type-Options", "nosniff")
        .default_header("X-Frame-Options", "DENY")
        .default_header("X-Version", format!("Connorcode/{}", VERSION))
        .default_header("X-Server", format!("afire/{}", afire::VERSION))
        // Set other things
        .socket_timeout(Duration::from_secs(5));

    server.error_handler(|_app, _req, err| {
        Response::new()
            .status(500)
            .text(
                assets::ERROR_PAGE
                    .replace("{{VERSION}}", VERSION)
                    .replace("{{ERROR}}", &err)
                    .replace("{{ERROR_BODY}}", &urlencoding::encode(&err))
                    .replace(
                        "{{CMP: Footer}}",
                        &assets::FOOTER.replace("{{VERSION}}", VERSION),
                    ),
            )
            .content(Content::HTML)
    });

    let app = server.state.as_ref().unwrap().clone();
    app.reload_articles();
    app.reload_links();

    middleware::attach(&mut server);
    serve_static::attach(&mut server);
    Files(app.clone()).attach(&mut server);
    routes::attach(&mut server);
    control::attach(&mut server);
    Analytics::new(app.clone()).attach(&mut server);
    logger::Logger.attach(&mut server);

    ctrlc::init(app.clone());
    print_info(app);
    color_print!(Color::Blue, "[*] Starting server on {}:{}\n", &host, port);

    server.start_threaded(threads).expect("Server Port In Use");
}

#[rustfmt::skip]
fn print_info(app: Arc<App>) {
    color_print!(Color::Magenta, "[=] Config");
    color_print!(Color::Magenta, " ├── Analytics");
    color_print!(Color::Magenta, " │   ├── Enabled: {}", app.config.analytics_enabled);
    color_print!(Color::Magenta, " │   ├── Period: {}", app.config.dump_period);
    color_print!(Color::Magenta, " │   └── Serve: {}", app.config.analytics_serve);
    color_print!(Color::Magenta, " └── Other");
    color_print!(Color::Magenta, "     ├── Status Serve: {}", app.config.status_serve);
    color_print!(Color::Magenta, "     └── Onion Broadcast: {}", app.config.broadcast_onion);
}
