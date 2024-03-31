use std::{
    sync::Mutex,
    time::{Duration, Instant},
    vec,
};

use afire::{internal::sync::ForceLockMutex, Content, Method, Query, Server};
use ahash::{HashMap, HashMapExt};
use anyhow::Result;
use serde_json::{json, Value};

use crate::{app::App, common::best_number};

const CACHE_DURATION: Duration = Duration::from_secs(60 * 60 * 12);

pub fn attach(server: &mut Server<App>) {
    let cache = Mutex::new(HashMap::<Service, Cached<u64>>::new());

    server.route(Method::GET, "/api/downloads", move |ctx| {
        let services = Service::from_query(&ctx.req.query);
        let mut out = json!({});

        let mut downloads = 0;
        let mut cache = cache.force_lock();
        for service in &services {
            if let Some(i) = cache.get(&service) {
                if !i.older_than(CACHE_DURATION) {
                    downloads += i.get();
                    out[service.service.name()] = (*i.get()).into();
                    continue;
                }
            }

            let i = service.get_downloads()?;
            cache.insert(service.clone(), Cached::new(i));
            downloads += i;
            out[service.service.name()] = i.into();
        }
        drop(cache);

        out["total"] = downloads.into();
        out["total-human"] = best_number(downloads).into();
        ctx.text(out).content(Content::JSON).send()?;
        Ok(())
    });
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ServiceType {
    Github,
    Modrinth,
    CurseForge,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Service {
    service: ServiceType,
    project: String,
}

struct Cached<T> {
    value: T,
    last_updated: Instant,
}

impl ServiceType {
    const ALL: [Self; 3] = [Self::Github, Self::Modrinth, Self::CurseForge];

    fn name(&self) -> &str {
        match self {
            Self::Github => "github",
            Self::Modrinth => "modrinth",
            Self::CurseForge => "curseforge",
        }
    }
}

impl Service {
    fn from_query(query: &Query) -> Vec<Self> {
        let mut services = Vec::new();

        for service in ServiceType::ALL.iter() {
            if let Some(project) = query.get(service.name()) {
                services.push(Self {
                    service: service.clone(),
                    project: project.to_string(),
                });
            }
        }

        services
    }

    fn get_downloads(&self) -> Result<u64> {
        let project = &self.project;
        match self.service {
            ServiceType::Github => {
                let url = format!("https://api.github.com/repos/{project}/releases");
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
            ServiceType::Modrinth => {
                let url = format!("https://api.modrinth.com/v2/project/{project}");
                let json = ureq::get(&url).call()?.into_json::<Value>()?;
                Ok(json["downloads"].as_u64().unwrap_or(0))
            }
            ServiceType::CurseForge => {
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
