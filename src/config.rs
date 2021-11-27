use simple_config_parser::Config;

static mut CONFIG: Config = Config { data: Vec::new() };

macro_rules! config {
    () => {{
        unsafe { &CONFIG }
    }};
}

lazy_static! {
    // Server Config
    pub static ref SERVER_HOST: String = config!().get_str("ip").unwrap().to_owned();
    pub static ref SERVER_PORT: u16 = config!().get::<u16>("port").unwrap().to_owned();

    // Analytics Config
    pub static ref ANALYTICS_ENABLED: bool = config!().get::<bool>("analytics_enabled").unwrap().to_owned();
    pub static ref ANALYTICS_PATH: String = config!().get_str("analytics_path").unwrap().to_owned();
    pub static ref DUMP_PEROID: u64 = config!().get::<u64>("dump_peroid").unwrap().to_owned();

}

pub fn load(path: &str) -> Option<()> {
    let cfg = Config::new().file(path).ok()?;
    unsafe {
        CONFIG = cfg;
    }

    Some(())
}
