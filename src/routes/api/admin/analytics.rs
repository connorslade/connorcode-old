use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use afire::{Method, Response, Server};
use ahash::{HashMap, HashMapExt};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::analytics::Stats;
use crate::app::App;
use crate::common::get_header;

pub fn attach(server: &mut Server<App>) {
    if !server.state.as_ref().unwrap().config.analytics_serve {
        return;
    }

    server.stateful_route(Method::GET, "/api/analytics", |app, req| {
        // Check Auth
        let auth = match get_header(req.headers, "Auth") {
            Some(i) => i,
            None => {
                return Response::new().status(403).text("No Authorization Header");
            }
        };

        // Make sure Auth is not too long before hashing
        if auth.len() > 100 {
            return Response::new()
                .status(403)
                .text("Auth Header is *way* too long");
        }

        let mut hasher = Sha256::new();
        hasher.update(auth.into_bytes());
        let result = hasher.finalize();

        if format!("{:02x}", result) != app.config.pass {
            return Response::new().status(403).text("Invalid Auth Header");
        }

        // Get Data From Disk
        let folder = Path::new(&app.config.analytics_path);
        let files = fs::read_dir(folder).expect("Error Reading Dir");
        let mut all_data: HashMap<String, Vec<Stats>> = HashMap::new();

        for i in files {
            // Read file
            let file = i.expect("Error getting analytics file");
            if !file.path().is_file() || file.path().extension() != Some(OsStr::new("aan")) {
                continue;
            }

            let data = fs::read(file.path()).expect("Error Reading Analytics File");

            // Parse Data
            let data = bincode::deserialize::<HashMap<String, Vec<Stats>>>(&data)
                .expect("Error Deserializeing Data");

            // Marge data to all_data
            for (ip, data) in data {
                if let Some(new) = all_data.get(&ip) {
                    let mut new = new.to_vec();
                    new.extend(data);
                    all_data.insert(ip.to_owned(), new);
                    continue;
                }

                all_data.insert(ip.to_owned(), data);
            }
        }

        // Add in-memory data
        for (ip, data) in app.analytics_data.lock().iter() {
            if let Some(new) = all_data.get(ip) {
                let mut new = new.to_vec();
                new.extend(data.to_owned());
                all_data.insert(ip.to_owned(), new);
                continue;
            }

            all_data.insert(ip.to_owned(), data.to_owned());
        }

        if all_data.is_empty() {
            return Response::new()
                .status(425)
                .reason("Too Early")
                .text(r#"{"error": "No Data Yet"}"#)
                .header("Content-Type", "application/json");
        }

        Response::new()
            .text(json!(all_data))
            .header("Content-Type", "application/json")
    });
}
