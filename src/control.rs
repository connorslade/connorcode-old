// use afire::Server;
// use serde_json::Value;

// use crate::App;

/*
 - update-articles | Reloads articles from disk
 - update-links    | Reloads links from disk
*/

// pub fn attach(server: &mut Server<App>) {
//     let app = server.state.as_ref().unwrap().clone();
//     let link_app = app.clone();

//     RemoteControl::new()
//         .system("update-articles", move |_| ok(|| app.reload_articles()))
//         .system("update-links", move |_| ok(|| link_app.reload_links()));
//     .attach(server);
// }

// fn ok(exe: impl Fn()) -> Value {
//     exe();
//     Value::String("Ok".to_owned())
// }

// TODO: Article caching
/*
 - Make a new folder .cache maybe?
 - When an article is loaded if it is not in the cache add its html and markdown hash
 - If the article is already cached and the hash are the same serve that
*/
