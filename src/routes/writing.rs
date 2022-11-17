use afire::{Content, Method, Response, Server};
use unindent::unindent;

use crate::{app::App, assets::WRITING_HOME};

pub fn attach(server: &mut Server<App>) {
    server.stateful_route(Method::GET, "/writing", |app, _req| {
        let articles = app.articles.articles.read();
        let mut article_vec = articles.iter().map(|x| x.1).collect::<Vec<_>>();
        article_vec.sort_unstable_by(|x, y| y.epoch.cmp(&x.epoch));

        let mut documents = String::new();
        for i in article_vec.iter().filter(|x| !x.hidden) {
            documents.push_str(&unindent(&format!(
                r#"<div class="article">
                <i class="icon"><i data-feather="file-text"></i></i>
                <p class="name">{}</p>
                <p class="disc">{desc}</p>
                <p class="date"><i data-feather="calendar"></i> {}</p>
                <a href="/writing/{}" aria-label="Read more about {desc}"><span class="div-link"></span></a>
            </div>"#,
                /*i.icon,*/ i.title, i.date, i.path, desc = i.description
            )));
            // TODO: Resolve icon not being displayed
        }

        Response::new()
            .text(WRITING_HOME.replace("{{ARTICLES}}", &documents))
            .content(Content::HTML)
    });
}
