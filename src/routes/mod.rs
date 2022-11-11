use afire::Server;

use crate::app::App;

mod api;
mod article;
mod index;
mod key;
mod link;
mod randomimg;
mod writing;

pub fn attach(server: &mut Server<App>) {
    api::attach(server);
    index::attach(server);
    key::attach(server);
    link::attach(server);
    randomimg::attach(server);
    writing::attach(server);
    article::attach(server);
}
