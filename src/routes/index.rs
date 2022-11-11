use std::fs;

use afire::{Content, Method, Response, Server};
use serde::Serialize;
use serde_json::json;
use simple_config_parser::Config;

use crate::{app::App, VERSION};

#[derive(Clone, Serialize)]
struct Project {
    id: String,
    name: String,
    date: String,
    link: String,
    image: String,
}

impl Project {
    fn format(&self, template: &str) -> String {
        template
            .replace("{{ID}}", &self.id)
            .replace("{{NAME}}", &self.name)
            .replace("{{DATE}}", &self.date)
            .replace("{{LINK}}", &self.link)
            .replace("{{IMAGE}}", &self.image)
    }
}

pub fn attach(server: &mut Server<App>) {
    let cfg = Config::new()
        .file("data/config/projects.cfg")
        .expect("Error Reading Project Config");

    let base_page =
        fs::read_to_string("data/web/template/index.html").expect("Error Reading BasePage");

    let base_template = fs::read_to_string("data/web/template/project.html")
        .expect("Error Reading Project Template");

    let mut projects = Vec::new();

    for i in cfg.data {
        if i[0].starts_with("project_") {
            let parts: Vec<String> = i[1].split(',').map(|x| x.to_string()).collect();
            let id = i[0]
                .split("project_")
                .nth(1)
                .expect("Error Parsing Project line");

            projects.push(Project {
                id: id.to_owned(),
                name: parts[0].replace('_', ",").trim().to_string(),
                date: parts[1].trim().to_string(),
                link: parts[2].trim().to_string(),
                image: parts[3].trim().to_string(),
            });
        }
    }

    let mut projects_html = String::new();
    let mut projects_json = String::new();

    for i in projects {
        projects_html.push_str(&i.format(&base_template));
        projects_html.push('\n');

        projects_json.push_str(&json!(i).to_string());
        projects_json.push_str(", ");
    }

    projects_json.truncate(projects_json.len() - 2);

    let projects_html = base_page
        .replace("{{ITEMS}}", &projects_html)
        .replace("{{VERSION}}", VERSION);
    let projects_json = format!("[{}]", projects_json);

    // Serve Main Page
    server.route(Method::GET, "/", move |_req| {
        Response::new().text(&projects_html).content(Content::HTML)
    });

    server.route(Method::GET, "/api/projects", move |_req| {
        Response::new().text(&projects_json).content(Content::JSON)
    });
}
