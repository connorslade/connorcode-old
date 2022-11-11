use afire::{Content, Method, Response, Server};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::app::App;
use crate::common::get_header;

pub fn attach(server: &mut Server<App>) {
    if !server.state.as_ref().unwrap().config.status_serve {
        return;
    }

    server.stateful_route(Method::GET, "/api/status", |app, req| {
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

        if format!("{:02x}", result) != app.config.pass {
            return Response::new().status(403).text("Invalid Pass Header");
        }

        let disk = sys_info::disk_info().expect("Error getting Disk info");
        let mem = sys_info::mem_info().expect("Error getting Memory info");
        let load = sys_info::loadavg().expect("Error getting Load history");
        let proc = sys_info::proc_total().expect("Error getting process count");
        let os = sys_info::os_type().expect("Error getting OS type");
        let os_rel = sys_info::os_release().expect("Error getting OS info");

        Response::new()
            .text(json!({
             "os": {
                 "type": os,
                 "release": os_rel
             },
             "disk": {
                 "total": disk.total,
                 "free": disk.free
             },
             "memory": {
                 "total": mem.total,
                 "free": mem.free,
             },
             "load": {
                 "1m": load.one,
                 "5m": load.five,
                 "15m": load.fifteen,
             },
             "processes": proc
            }))
            .content(Content::JSON)
    });
}
