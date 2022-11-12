use afire::{internal::common::remove_address_port, Server};
use afire_integration::RemoteControl;

use crate::App;

/*
 - update-articles | Reloads articles from disk
 - update-links    | Reloads links from disk
*/

pub fn attach(server: &mut Server<App>) {
    let app = server.state.as_ref().unwrap().clone();
    let link_app = app.clone();

    RemoteControl::new()
        .any(|req, data| {
            println!(
                "[R] Control `{}` from `{}`",
                data.action,
                remove_address_port(&req.address)
            )
        })
        .system("update-articles", move |_| ok(|| app.reload_articles()))
        .system("update-links", move |_| ok(|| link_app.reload_links()))
        .attach(server);
}

fn ok(exe: impl Fn()) -> String {
    exe();
    "Ok".to_owned()
}
