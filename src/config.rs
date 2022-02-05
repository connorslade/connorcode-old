use lazy_static::initialize;
use simple_config_parser::{Config, ConfigError};

static mut CONFIG: Config = Config { data: Vec::new() };

macro_rules! config {
    () => {{
        unsafe { &CONFIG }
    }};
}

macro_rules! init_lazy {
    ($($exp:expr),+) => {
        $(initialize(&$exp);)*
    };
}

lazy_static! {
    // Server Config
    pub static ref SERVER_HOST: String = config!().get_str("ip").unwrap();
    pub static ref SERVER_PORT: u16 = config!().get::<u16>("port").unwrap();
    pub static ref EXTERNAL_URI: String = config!().get_str("external_uri").unwrap();
    pub static ref DATA_DIR: String = config!().get_str("data_dir").unwrap();

    // File Serve
    pub static ref FILE_SERVE: bool = config!().get::<bool>("file_serve").unwrap();
    pub static ref FILE_SERVE_PATH: String = config!().get_str("file_serve_path").unwrap();

    // Writing
    pub static ref WRITING_PATH: String = config!().get_str("writing_path").unwrap();


    // Analytics Config
    pub static ref ANALYTICS_ENABLED: bool = config!().get::<bool>("analytics_enabled").unwrap();
    pub static ref ANALYTICS_SERVE: bool = config!().get::<bool>("analytics_serve").unwrap();
    pub static ref ANALYTICS_PATH: String = config!().get_str("analytics_path").unwrap();
    pub static ref DUMP_PEROID: u64 = config!().get::<u64>("dump_peroid").unwrap();

    // Admin Other
    pub static ref STATUS_SERVE: bool = config!().get::<bool>("status_serve").unwrap();
    pub static ref PASS: String = config!().get_str("pass").unwrap();

    // Other
    pub static ref DATABASE_PATH: String = config!().get_str("database_path").unwrap();
    pub static ref BROADCAST_ONION: bool = config!().get::<bool>("onion_brodcast").unwrap();
    pub static ref ONION_SITE: String = config!().get_str("onion_site").unwrap();
}

pub fn load(path: &str) -> Result<(), ConfigError> {
    let cfg = Config::new().file(path)?;

    unsafe {
        CONFIG = cfg;
    }

    // Init the lazy config values
    init_lazy! {
        SERVER_HOST, SERVER_PORT, EXTERNAL_URI, DATA_DIR, FILE_SERVE,
        FILE_SERVE_PATH, WRITING_PATH, ANALYTICS_ENABLED, ANALYTICS_SERVE,
        ANALYTICS_PATH, DUMP_PEROID, STATUS_SERVE, PASS, DATABASE_PATH,
        BROADCAST_ONION, ONION_SITE
    }

    Ok(())
}
