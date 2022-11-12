use std::sync::Arc;

use afire::{internal::common::remove_address_port, Server};
use afire_integration::RemoteControl;

use crate::App;

/*
 - update-articles | Reloads articles from disk
 - update-links    | Reloads links from disk
 - update-home     | Reloads projects for home page
*/

pub fn attach(server: &mut Server<App>) {
    // idk
    let app = server.state.as_ref().unwrap().clone();
    let link_app = app.clone();
    let home_app = app.clone();

    RemoteControl::new()
        .enabled(true)
        .any(|req, data| {
            println!(
                "[R] Control `{}` from `{}`",
                data.action,
                remove_address_port(&req.address)
            )
        })
        .system("update-articles", move |_| update_articles(app.clone()))
        .system("update-links", move |_| update_links(link_app.clone()))
        .system("update-home", move |_| update_home(home_app.clone()))
        .attach(server);
}

fn update_articles(app: Arc<App>) -> String {
    println!("[*] Reloading Articles");
    app.reload_articles();
    "Ok".to_owned()
}

fn update_links(app: Arc<App>) -> String {
    "Ok".to_owned()
}

fn update_home(app: Arc<App>) -> String {
    "Ok".to_owned()
}
