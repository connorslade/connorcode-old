use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use afire::{
    internal,
    middleware::{MiddleRequest, Middleware},
    Header, Request, Server,
};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{ANALYTICS_ENABLED, ANALYTICS_PATH, DUMP_PEROID};

type Ip = String;

#[derive(Clone, Serialize, Deserialize)]
pub struct Stats {
    pub time: u64,
    pub path: String,
    pub method: Method,
    pub user_agent: Option<String>,
    pub referer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Analytics {
    data: HashMap<Ip, Vec<Stats>>,
    last_dump: SystemTime,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Options,
    Head,
    Patch,
    Trace,
    Custom(String),
}

impl Middleware for Analytics {
    fn pre(&mut self, req: Request) -> MiddleRequest {
        self.save(&req);
        self.check_dump();

        MiddleRequest::Continue
    }

    fn attach(self, server: &mut Server)
    where
        Self: Sized + 'static,
    {
        if !*ANALYTICS_ENABLED {
            return;
        }

        server.middleware.push(Box::new(RefCell::new(self)));
    }
}

impl Stats {
    fn new(
        time: u64,
        path: String,
        method: Method,
        agent: Option<String>,
        referer: Option<String>,
    ) -> Self {
        Stats {
            time,
            path,
            method,
            user_agent: agent,
            referer,
        }
    }
}

impl Analytics {
    pub fn new() -> Self {
        Analytics {
            data: HashMap::new(),
            last_dump: SystemTime::now(),
        }
    }

    fn save(&mut self, req: &Request) -> Option<()> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Cant get time: Time went backwards!")
            .as_secs();
        let mut ip = internal::common::remove_address_port(req.address.to_owned());

        // If Ip is Localhost and 'X-Forwarded-For' Header is present
        // Use that as Ip
        if ip == "127.0.0.1" {
            if let Some(i) = req.headers.iter().find(|x| x.name == "X-Forwarded-For") {
                ip = i.value.to_owned();
            }
        }

        let path = internal::path::normalize_path(req.path.to_owned());
        let agent = get_header(&req.headers, "User-Agent");
        let referer = get_header(&req.headers, "Referer");
        let stats = Stats::new(
            time,
            path,
            Method::from_afire(req.method.clone()),
            agent,
            referer,
        );

        if self.data.contains_key(&ip) {
            let mut new = self.data.get(&ip)?.to_vec();
            new.push(stats);
            self.data.insert(ip, new);
            return Some(());
        }
        self.data.insert(ip, vec![stats]);
        Some(())
    }

    fn check_dump(&mut self) -> Option<()> {
        if self
            .last_dump
            .elapsed()
            .expect("Cant get elapsed time: Time went backwards!")
            .as_secs()
            < *DUMP_PEROID
        {
            return Some(());
        }

        println!("[*] Saveing Analytics");

        // Update Last Dump time
        self.last_dump = SystemTime::now();

        self.dump()?;

        Some(())
    }

    pub fn dump(&mut self) -> Option<()> {
        // Create Path
        let folder = Path::new(&*ANALYTICS_PATH);
        if !folder.exists() {
            fs::create_dir_all(folder).ok()?;
        }
        let filename = Local::now().format("Analytics-%Y-%m-%d.aan").to_string();
        let path = folder.join(filename);

        // Load Prev Data
        if path.exists() {
            let old = fs::read(path.clone()).ok()?;
            let mut old: HashMap<Ip, Vec<Stats>> = bincode::deserialize(&old).ok()?;

            // Add New Data
            for i in self.data.clone() {
                let ip = i.0;
                let data = i.1;

                if let Some(new) = old.get(&ip) {
                    let mut new = new.to_vec();
                    new.extend(data);
                    old.insert(ip.to_owned(), new);
                    continue;
                }

                old.insert(ip.to_owned(), data);
            }

            // Reset In Memory Analytics Cache thing
            self.data.clear();
            let new = bincode::serialize(&old).ok()?;

            // Write New File
            fs::write(path, new).ok()?;

            return Some(());
        }

        let new = bincode::serialize(&self.data).ok()?;
        fs::write(path, new).ok()?;
        Some(())
    }
}

impl Method {
    fn from_afire(old: afire::Method) -> Method {
        match old {
            afire::Method::GET => Method::Get,
            afire::Method::POST => Method::Post,
            afire::Method::PUT => Method::Put,
            afire::Method::DELETE => Method::Delete,
            afire::Method::OPTIONS => Method::Options,
            afire::Method::HEAD => Method::Head,
            afire::Method::PATCH => Method::Patch,
            afire::Method::TRACE => Method::Trace,
            afire::Method::CUSTOM(s) => Method::Custom(s),
            afire::Method::ANY => Method::Custom("ANY".to_owned()),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::Get => write!(f, "GET"),
            Method::Post => write!(f, "POST"),
            Method::Put => write!(f, "PUT"),
            Method::Delete => write!(f, "DELETE"),
            Method::Options => write!(f, "OPTIONS"),
            Method::Head => write!(f, "HEAD"),
            Method::Patch => write!(f, "PATCH"),
            Method::Trace => write!(f, "TRACE"),
            Method::Custom(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            r#"[ ({}) {} "{}" {} {} ]"#,
            self.time,
            self.method,
            self.path,
            self.referer.clone().unwrap_or_else(|| "".to_owned()),
            self.user_agent.clone().unwrap_or_else(|| "".to_owned())
        );

        f.write_str(&out)
    }
}

fn get_header(headers: &[Header], key: &str) -> Option<String> {
    for i in headers {
        if i.name == key {
            return Some(i.value.clone());
        }
    }
    None
}
