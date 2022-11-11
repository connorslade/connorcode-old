use afire::Server;

use crate::app::App;

mod analytics;
mod status;

pub fn attach(server: &mut Server<App>) {
    analytics::attach(server);
    status::attach(server);
}
