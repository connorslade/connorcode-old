use afire::Server;

use crate::app::App;

mod articles_json;
mod articles_rss;

pub fn attach(server: &mut Server<App>) {
    articles_json::attatch(server);
    articles_rss::attatch(server);
}
