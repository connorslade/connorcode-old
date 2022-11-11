use afire::Server;

use crate::app::App;

mod article;
pub use article::Article;

pub fn attach(server: &mut Server<App>) {
    // Load Articles
    server.state.as_ref().unwrap().reload_articles();
}

// enum Mango<'a> {
//     GoGo(&'a str),
// }

// impl<'a> Mango<'a> {
//     fn say(&self) {
//         match self {
//             Mango::GoGo(i) => println!("{} world", i),
//         }
//     }
// }
