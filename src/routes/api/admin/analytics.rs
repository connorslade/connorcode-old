use std::collections::HashMap;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

use afire::{Header, Method, Response, Server};
use sha2::{Digest, Sha256};

use crate::analytics::Stats;
use crate::common::get_header;
use crate::config::{ANALYTICS_PATH, ANALYTICS_SERVE, PASS};

pub fn attach(server: &mut Server) {
    if !*ANALYTICS_SERVE {
        return;
    }

    server.route(Method::GET, "/api/analytics", |req| {
        // Check Auth
        let auth = match get_header(req.headers, "Auth") {
            Some(i) => i,
            None => {
                return Response::new().status(403).text("No Authorization Header");
            }
        };

        // Make sure Auth is not too long before hashing
        if auth.len() > 100 {
            return Response::new().status(403).text("Auth Header is *way* too long");
        }

        let mut hasher = Sha256::new();
        hasher.update(auth.into_bytes());
        let result = hasher.finalize();

        if format!("{:02x}", result) != *PASS {
            return Response::new().status(403).text("Invalid Auth Header");
        }

        // Get Data From Disk
        let folder = Path::new(&*ANALYTICS_PATH);
        let files = fs::read_dir(folder).expect("Error Reading Dir");
        let mut all_data: HashMap<String, Vec<Stats>> = HashMap::new();

        for i in files {
            // Read file
            let file = i.unwrap();
            if !file.path().is_file() || file.path().extension() != Some(OsStr::new("aan")) {
                continue;
            }

            let data = fs::read(file.path()).expect("Error Reading Analytics File");

            // Parse Data
            let data: HashMap<String, Vec<Stats>> = bincode::deserialize(&data).expect("Error Deserializeing Data");

            // Marge data to all_data
            for i in data {
                let ip = i.0;
                let data = i.1;

                if all_data.contains_key(&ip) {
                    let mut new = all_data.get(&ip).unwrap().to_vec();
                    new.extend(data);
                    all_data.insert(ip.to_owned(), new);
                    continue;
                }

                all_data.insert(ip.to_owned(), data);
            }
        }

        if all_data.is_empty() {
            return Response::new()
                .status(425)
                .reason("Too Early")
                .text(r#"{"error": "No Data Yet"}"#)
                .header(Header::new("Content-Type", "application/json"));
        }

        let mut working = String::new();

        for i in all_data {
            let data = i.1;
            let mut segment = String::new();

            for i in data {
                segment.push_str(&format!(
                    r#"{{"time": {}, "method": "{}", "path": "{}", "agent": "{}", "referer": "{}"}}, "#,
                    i.time,
                    i.method.to_string().replace('\"', "\\\""),
                    i.path.replace('\"', "\\\""),
                    i.user_agent.clone().unwrap_or_else(|| "".to_owned()).replace('\"', "\\\""),
                    i.referer.clone().unwrap_or_else(|| "".to_owned()).replace('\"', "\\\"")
                ))
            }

            if segment.len() >= 2 {
                segment.pop();
                segment.pop();
            }

            working.push_str(&format!(r#""{}": ["#, i.0));
            working.push_str(&segment);
            working.push_str("], ");
        }

        working.pop();
        working.pop();

        Response::new()
            .text(format!("{{{}}}", working))
            .header(Header::new("Content-Type", "application/json"))
    });
}
