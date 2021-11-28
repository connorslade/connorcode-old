use std::collections::HashMap;
use std::fs;
use std::path::Path;

use afire::{Header, Method, Response, Server};
use bincode;
use sha2::{Digest, Sha256};

use crate::analytics::Stats;
use crate::config::{ANALYTICS_PASS, ANALYTICS_PATH, ANALYTICS_SERVE};

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

        let mut hasher = Sha256::new();
        hasher.update(auth.into_bytes());
        let result = hasher.finalize();

        if format!("{:02x}", result) != *ANALYTICS_PASS {
            return Response::new().status(403).text("Invalid Pass Header");
        }

        // Get Data From Disk
        let folder = Path::new(&*ANALYTICS_PATH);
        let files = fs::read_dir(folder).expect("Error Reading Dir");
        let mut all_data: HashMap<String, Vec<Stats>> = HashMap::new();

        for i in files {
            // Read file
            let file = i.unwrap();
            let data = fs::read(file.path()).expect("Error Reading Analytics File");

            // Parse Data
            let data: HashMap<String, Vec<Stats>> = bincode::deserialize(&data).expect("Error Deserializeing Data");

            // Marge data to all_data
            for i in data {
                let ip = i.0;
                for data in i.1 {

                if all_data.contains_key(&ip) {
                    let mut new = all_data.get(&ip).unwrap().to_vec();
                    new.push(data);
                    all_data.insert(ip.to_owned(), new);
                    continue;
                }

                all_data.insert(ip.to_owned(), vec![data]);
                }
            }
        }

        if all_data.len() == 0 {
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
                    i.method.to_string(),
                    i.path,
                    i.user_agent.clone().unwrap_or_else(|| "".to_owned()),
                    i.referer.clone().unwrap_or_else(|| "".to_owned())
                ))
            }

            if segment.len() >= 2 {
                segment.pop();
                segment.pop();
            }

            working.push_str(&format!(r#""{}": ["#, i.0));
            working.push_str(&segment);
            working.push(']');
        }

        Response::new()
            .text(format!("{{{}}}", working))
            .header(Header::new("Content-Type", "application/json"))
    });
}

fn get_header(headers: Vec<Header>, header: &str) -> Option<String> {
    for i in headers {
        if i.name == header {
            return Some(i.value);
        }
    }
    None
}
