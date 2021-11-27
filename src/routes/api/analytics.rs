use std::collections::HashMap;
use std::fs;
use std::path::Path;

use afire::{Header, Method, Response, Server};
use bincode;

use crate::analytics::Stats;
use crate::config::{ANALYTICS_PATH, ANALYTICS_SERVE};

pub fn attach(server: &mut Server) {
    if !*ANALYTICS_SERVE {
        return;
    }

    server.route(Method::GET, "/api/analytics", |_req| {
        // Get Data From Disk
        let folder = Path::new(&*ANALYTICS_PATH);
        let files = fs::read_dir(folder).unwrap();
        let mut all_data: HashMap<String, Vec<Stats>> = HashMap::new();

        for i in files {
            // Read file
            let file = i.unwrap();
            let data = fs::read(file.path()).unwrap();

            // Parse Data
            let data: HashMap<String, Vec<Stats>> = bincode::deserialize(&data).unwrap();

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

        for i in all_data.keys() {
            let data = all_data.get(i).unwrap();
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

            working.push_str(&format!(r#""{}": ["#, i));
            working.push_str(&segment);
            working.push(']');
        }

        Response::new()
            .text(format!("{{{}}}", working))
            .header(Header::new("Content-Type", "application/json"))
    });
}
