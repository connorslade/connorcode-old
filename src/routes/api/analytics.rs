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
            let file = i.unwrap();
            let data = fs::read(file.path()).unwrap();
            let data: HashMap<String, Vec<Stats>> = bincode::deserialize(&data).unwrap();
            all_data.extend(data);
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

            working.push_str(&segment);
        }

        Response::new()
            .text(format!("[{}]", working))
            .header(Header::new("Content-Type", "application/json"))
    });
}
