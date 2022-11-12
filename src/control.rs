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
        .system("update-articles", move |_| ok(|| app.reload_articles()))
        .system("update-links", move |_| ok(|| link_app.reload_links()))
        // .system("update-home", move |_| ok(home_app.clone()))
        .attach(server);
}

fn ok(exe: impl Fn()) -> String {
    exe();
    "Ok".to_owned()
}
