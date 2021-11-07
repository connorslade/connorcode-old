use std::fs;

use afire::Header;
use afire::Method;
use afire::Response;
use afire::Server;
use simple_config_parser::Config;

use crate::VERSION;

static mut PROJECTS: Option<Vec<Project>> = None;
static mut BASE_PAGE: Option<String> = None;
static mut BASE: Option<String> = None;

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
}

pub fn attach(server: &mut Server) {
    let cfg = Config::new().file("data/config/projects.cfg").unwrap();
    let base_page = fs::read_to_string("data/template/index.html").unwrap();
    let base_template = fs::read_to_string("data/template/project.html").unwrap();
    let mut projects = Vec::new();

    for i in cfg.data {
        if i[0].starts_with("project_") {
            let parts: Vec<String> = i[1].split(",").map(|x| x.to_string()).collect();
            let id = i[0].split("project_").nth(1).unwrap();

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
        PROJECTS = Some(projects);
        BASE_PAGE = Some(base_page);
        BASE = Some(base_template);
    }

    server.route(Method::GET, "/", |_req| {
        let base = unsafe { BASE_PAGE.clone().unwrap() };
        let raw_projects = unsafe { PROJECTS.clone().unwrap() };
        let template = unsafe { BASE.clone().unwrap() };

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
            .header(Header::new("Content-Type", "text/html"))
    });
}
