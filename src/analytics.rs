use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use afire::{Header, Request, Server};
use bincode;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{ANALYTICS_ENABLED, ANALYTICS_PATH, DUMP_PEROID};

type Ip = String;

#[derive(Clone, Serialize, Deserialize)]
struct Stats {
    time: u64,
    path: String,
    method: Method,
    user_agent: Option<String>,
    referer: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Analytics {
    data: HashMap<Ip, Vec<Stats>>,
    last_dump: SystemTime,
}

#[derive(Clone, Serialize, Deserialize)]
enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    OPTIONS,
    HEAD,
    PATCH,
    TRACE,
    CUSTOM(String),
}

pub fn attach(server: &mut Server) {
    if !*ANALYTICS_ENABLED {
        return;
    }

    let cell = RefCell::new(Analytics::new());

    server.middleware(Box::new(move |req| {
        cell.borrow_mut().save(req);
        cell.borrow_mut().check_dump();
        None
    }));
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
    fn new() -> Self {
        Analytics {
            data: HashMap::new(),
            last_dump: SystemTime::now(),
        }
    }

    fn save(&mut self, req: &Request) -> Option<()> {
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let ip = req.address.split(':').next()?;

        let mut path = req.path.clone();
        if !path.starts_with('/') {
            path = format!("/{}", path);
        }
        let agent = get_header(&req.headers, "User-Agent");
        let referer = get_header(&req.headers, "Referer");
        let stats = Stats::new(
            time,
            path.to_owned(),
            Method::from_afire(req.method.clone()),
            agent,
            referer,
        );

        if self.data.contains_key(ip) {
            let mut new = self.data.get(ip)?.to_vec();
            new.push(stats);
            self.data.insert(ip.to_owned(), new);
            return Some(());
        }
        self.data.insert(ip.to_owned(), vec![stats]);
        Some(())
    }

    fn check_dump(&mut self) -> Option<()> {
        if self.last_dump.elapsed().unwrap().as_secs() < *DUMP_PEROID {
            return Some(());
        }

        println!("[*] Saveing Analytics");

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
            old.extend(self.data.clone());
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
            afire::Method::GET => Method::GET,
            afire::Method::POST => Method::POST,
            afire::Method::PUT => Method::PUT,
            afire::Method::DELETE => Method::DELETE,
            afire::Method::OPTIONS => Method::OPTIONS,
            afire::Method::HEAD => Method::HEAD,
            afire::Method::PATCH => Method::PATCH,
            afire::Method::TRACE => Method::TRACE,
            afire::Method::CUSTOM(s) => Method::CUSTOM(s),
            afire::Method::ANY => Method::CUSTOM("ANY".to_owned()),
        }
    }
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Method::GET => write!(f, "GET"),
            Method::POST => write!(f, "POST"),
            Method::PUT => write!(f, "PUT"),
            Method::DELETE => write!(f, "DELETE"),
            Method::OPTIONS => write!(f, "OPTIONS"),
            Method::HEAD => write!(f, "HEAD"),
            Method::PATCH => write!(f, "PATCH"),
            Method::TRACE => write!(f, "TRACE"),
            Method::CUSTOM(s) => write!(f, "{}", s),
        }
    }
}

impl fmt::Debug for Stats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let out = format!(
            r#"[ ({}) {} "{}" {} {} ]"#,
            self.time,
            self.method.to_string(),
            self.path,
            self.referer.clone().unwrap_or_else(|| "".to_owned()),
            self.user_agent.clone().unwrap_or_else(|| "".to_owned())
        );

        f.write_str(&out)
    }
}

fn get_header(headers: &Vec<Header>, key: &str) -> Option<String> {
    for i in headers {
        if i.name == key {
            return Some(i.value.clone());
        }
    }
    None
}
