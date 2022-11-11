use afire::Server;

use crate::app::App;

mod articles_json;
mod articles_rss;
mod like;

pub fn attach(server: &mut Server<App>) {
    articles_json::attach(server);
    articles_rss::attach(server);
    like::attach(server);
}
