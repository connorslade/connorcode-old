use afire::Server;
use afire_integration::RemoteControl;

use crate::App;

pub fn attach(server: &mut Server<App>) {
    RemoteControl::new()
        .system("writing", |_data| "Mangos".to_owned())
        .attach(server);
}
