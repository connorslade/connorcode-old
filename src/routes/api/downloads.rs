use std::{
    sync::Mutex,
    time::{Duration, Instant},
    vec,
};

use afire::{internal::sync::ForceLockMutex, Content, Method, Query, Server};
use ahash::{HashMap, HashMapExt};
use anyhow::Result;
use serde_json::{json, Value};

use crate::app::App;

const CACHE_DURATION: Duration = Duration::from_secs(60 * 60 * 12);

pub fn attach(server: &mut Server<App>) {
    let cache = Mutex::new(HashMap::<Service, Cached<u64>>::new());

    server.route(Method::GET, "/api/downloads", move |ctx| {
        let services = Service::from_query(&ctx.req.query);

        let mut downloads = 0;
        let mut cache = cache.force_lock();
        for service in services {
            if let Some(i) = cache.get(&service) {
                if !i.older_than(CACHE_DURATION) {
                    downloads += i.get();
                    continue;
                }
            }

            let i = service.get_downloads()?;
            cache.insert(service.clone(), Cached::new(i));
            downloads += i;
        }

        ctx.text(json!({ "downloads": downloads }))
            .content(Content::JSON)
            .send()?;
        Ok(())
    });
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Service {
    Github { repo: String },
    Modrinth { project: String },
    CurseForge { project: String },
}

struct Cached<T> {
    value: T,
    last_updated: Instant,
}

impl Service {
    fn from_query(query: &Query) -> Vec<Self> {
        let mut services = Vec::new();
        if let Some(repo) = query.get("github") {
            services.push(Self::Github {
                repo: repo.to_string(),
            });
        }

        if let Some(project) = query.get("modrinth") {
            services.push(Self::Modrinth {
                project: project.to_string(),
            });
        }

        if let Some(project) = query.get("curseforge") {
            services.push(Self::CurseForge {
                project: project.to_string(),
            });
        }

        services
    }

    fn get_downloads(&self) -> Result<u64> {
        match self {
            Self::Github { repo } => {
                let url = format!("https://api.github.com/repos/{repo}/releases");
                let json = ureq::get(&url).call()?.into_json::<Value>()?;
                let downloads = json
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|release| {
                        release["assets"]
                            .as_array()
                            .unwrap_or(&vec![])
                            .iter()
                            .map(|asset| asset["download_count"].as_u64().unwrap_or(0))
                            .sum::<u64>()
                    })
                    .sum::<u64>();
                Ok(downloads)
            }
            Self::Modrinth { project } => {
                let url = format!("https://api.modrinth.com/v2/project/{project}");
                let json = ureq::get(&url).call()?.into_json::<Value>()?;
                Ok(json["downloads"].as_u64().unwrap_or(0))
            }
            Self::CurseForge { project } => {
                let url = format!("https://api.cfwidget.com/minecraft/mc-mods/{project}/download");
                let json = ureq::get(&url).call()?.into_json::<Value>()?;
                Ok(json["downloads"]["total"].as_u64().unwrap_or(0))
            }
        }
    }
}

impl<T> Cached<T> {
    fn new(value: T) -> Self {
        Self {
            value,
            last_updated: Instant::now(),
        }
    }

    fn older_than(&self, duration: Duration) -> bool {
        self.last_updated.elapsed() > duration
    }

    fn get(&self) -> &T {
        &self.value
    }
}
