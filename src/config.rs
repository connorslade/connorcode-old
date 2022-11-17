use std::{
    path::{Path, PathBuf},
    str::FromStr,
};

use simple_config_parser::Config as Cfg;

pub struct Config {
    // Server Config
    pub server_host: String,
    pub server_port: u16,
    pub threads: usize,
    pub external_uri: String,
    pub data_dir: PathBuf,

    // File Serve
    pub file_serve: bool,
    pub file_serve_path: String,

    // Writing
    pub writing_path: String,

    // Analytics Config
    pub analytics_enabled: bool,
    pub analytics_serve: bool,
    pub analytics_path: String,
    pub dump_peroid: u64,

    // Admin Other
    pub status_serve: bool,
    pub pass: String,

    // Other
    pub database_path: String,
    pub broadcast_onion: bool,
    pub onion_site: String,
    pub tempest_station: String,
    pub tempest_token: String,
}

impl Config {
    pub fn new<T: AsRef<Path>>(file: T) -> Self {
        let cfg = Cfg::new().file(file).unwrap();

        Self {
            server_host: get_config(&cfg, "ip"),
            server_port: get_config(&cfg, "port"),
            threads: get_config(&cfg, "threads"),
            external_uri: get_config(&cfg, "external_uri"),
            data_dir: get_config(&cfg, "data_dir"),
            file_serve: get_config(&cfg, "file_serve"),
            file_serve_path: get_config(&cfg, "file_serve_path"),
            writing_path: get_config(&cfg, "writing_path"),
            analytics_enabled: get_config(&cfg, "analytics_enabled"),
            analytics_serve: get_config(&cfg, "analytics_serve"),
            analytics_path: get_config(&cfg, "analytics_path"),
            dump_peroid: get_config(&cfg, "dump_peroid"),
            status_serve: get_config(&cfg, "status_serve"),
            pass: get_config(&cfg, "pass"),
            database_path: get_config(&cfg, "database_path"),
            broadcast_onion: get_config(&cfg, "onion_brodcast"),
            onion_site: get_config(&cfg, "onion_site"),
            tempest_station: get_config(&cfg, "tempest_station"),
            tempest_token: get_config(&cfg, "tempest_token"),
        }
    }
}

fn get_config<T: FromStr>(cfg: &Cfg, name: &str) -> T {
    cfg.get(name)
        .unwrap_or_else(|_| panic!("Error getting `{}` from Config", name))
}
