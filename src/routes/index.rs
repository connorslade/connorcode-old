use std::fs;

use afire::{Content, Header, Method, Response, Server};
use simple_config_parser::Config;

use crate::VERSION;

static mut PROJECTS: Vec<Project> = Vec::new();
static mut BASE_PAGE: String = String::new();
static mut BASE: String = String::new();

#[derive(Clone)]
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

    fn jsonify(&self) -> String {
        format!(
            r#"{{"id": "{}", "name": "{}", "date": "{}", "link": "{}", "image": "{}"}}"#,
            self.id, self.name, self.date, self.link, self.image
        )
    }
}

pub fn attach(server: &mut Server) {
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

    unsafe {
        PROJECTS = projects;
        BASE_PAGE = base_page;
        BASE = base_template;
    }

    // Serve Main Page
    server.route(Method::GET, "/", |_req| {
        let base = unsafe { BASE_PAGE.clone() };
        let raw_projects = unsafe { PROJECTS.clone() };
        let template = unsafe { BASE.clone() };

        let mut projects = String::new();
        for i in raw_projects {
            projects.push_str(&i.format(&template));
            projects.push('\n');
        }

        Response::new()
            .text(
                base.replace("{{ITEMS}}", &projects)
                    .replace("{{VERSION}}", VERSION),
            )
            .content(Content::HTML)
    });

    server.route(Method::GET, "/api/projects", |_req| {
        let projects = unsafe { PROJECTS.clone() };
        let mut json = String::new();

        for i in projects {
            json.push_str(&i.jsonify());
            json.push_str(", ");
        }

        json.truncate(json.len() - 2);

        Response::new()
            .text(format!("[{}]", json))
            .header(Header::new("Content-Type", "application/json"))
    });
}
