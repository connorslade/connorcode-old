use afire::{Content, Method, Server};
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    if !server.state.as_ref().unwrap().config.status_serve {
        return;
    }

    server.route(Method::GET, "/api/status", |ctx| {
        let app = ctx.app();

        // Check Auth
        let auth = match ctx.req.headers.get("Auth") {
            Some(i) => i,
            None => {
                return Ok(ctx.status(403).text("No Authorization Header").send()?);
            }
        };

        let mut hasher = Sha256::new();
        hasher.update(auth.as_bytes());
        let result = hasher.finalize();

        if format!("{:02x}", result) != app.config.pass {
            return Ok(ctx.status(403).text("Invalid Pass Header").send()?);
        }

        let disk = sys_info::disk_info().expect("Error getting Disk info");
        let mem = sys_info::mem_info().expect("Error getting Memory info");
        let load = sys_info::loadavg().expect("Error getting Load history");
        let proc = sys_info::proc_total().expect("Error getting process count");
        let os = sys_info::os_type().expect("Error getting OS type");
        let os_rel = sys_info::os_release().expect("Error getting OS info");

        ctx.text(json!({
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
        .send()?;
        Ok(())
    });
}
