use afire::{Content, Method, Response, Server};
use sha2::{Digest, Sha256};

use crate::app::App;
use crate::common::get_header;
use crate::template::Template;

const OUT_FORMAT: &str = r#"{"os": {"type": "{{OS_TYPE}}", "release": "{{OS_RELEASE}}"}, "disk": {"total": {{DISK_TOTAL}}, "free": {{DISK_FREE}}}, "memory": {"total": {{MEM_TOTAL}}, "free": {{MEM_FREE}}}, "load": {"1m": {{LOAD_1}}, "5m": {{LOAD_5}}, "15m": {{LOAD_15}}}, "processes": {{PROC}}}"#;

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
            .text(
                Template::new(OUT_FORMAT)
                    .template("DISK_TOTAL", disk.total)
                    .template("DISK_FREE", disk.free)
                    .template("MEM_TOTAL", mem.total)
                    .template("MEM_FREE", mem.free)
                    .template("LOAD_1", load.one)
                    .template("LOAD_5", load.five)
                    .template("LOAD_15", load.fifteen)
                    .template("PROC", proc)
                    .template("OS_TYPE", os)
                    .template("OS_RELEASE", os_rel)
                    .build(),
            )
            .content(Content::JSON)
    });
}
