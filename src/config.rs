use lazy_static::initialize;
use simple_config_parser::{Config, ConfigError};

static mut CONFIG: Config = Config { data: Vec::new() };

macro_rules! get_config {
    ($name:expr) => {
        unsafe { &CONFIG }
            .get($name)
            .expect(concat!("Error getting `", $name, "` from Config"))
    };
}

macro_rules! init_lazy {
    ($($exp:expr),+) => {
        $(initialize(&$exp);)*
    };
}

lazy_static! {
    // Server Config
    pub static ref SERVER_HOST: String = get_config!("ip");
    pub static ref SERVER_PORT: u16 = get_config!("port");
    pub static ref EXTERNAL_URI: String = get_config!("external_uri");
    pub static ref DATA_DIR: String = get_config!("data_dir");

    // File Serve
    pub static ref FILE_SERVE: bool = get_config!("file_serve");
    pub static ref FILE_SERVE_PATH: String = get_config!("file_serve_path");

    // Writing
    pub static ref WRITING_PATH: String = get_config!("writing_path");


    // Analytics Config
    pub static ref ANALYTICS_ENABLED: bool = get_config!("analytics_enabled");
    pub static ref ANALYTICS_SERVE: bool = get_config!("analytics_serve");
    pub static ref ANALYTICS_PATH: String = get_config!("analytics_path");
    pub static ref DUMP_PEROID: u64 = get_config!("dump_peroid");

    // Admin Other
    pub static ref STATUS_SERVE: bool = get_config!("status_serve");
    pub static ref PASS: String = get_config!("pass");

    // Other
    pub static ref DATABASE_PATH: String = get_config!("database_path");
    pub static ref BROADCAST_ONION: bool = get_config!("onion_brodcast");
    pub static ref ONION_SITE: String = get_config!("onion_site");
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
