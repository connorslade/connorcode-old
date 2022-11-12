use std::{env, fs, path::PathBuf};

use ahash::{HashMap, HashMapExt};
use parking_lot::{Mutex, RwLock};
use rusqlite::Connection;

use crate::{analytics::Stats, config::Config, writing::WritingCache};

pub struct App {
    // == App Styff ==
    /// App Config
    pub config: Config,

    /// Databse Connection
    pub database: Mutex<Connection>,

    // == Data ==
    /// Articles String -> Article
    pub articles: WritingCache,

    /// Redirects
    /// /r/<code> code -> url
    pub redirects: RwLock<HashMap<String, String>>,

    /// Current analytics_data (cleared on dump)
    pub analytics_data: Mutex<HashMap<String, Vec<Stats>>>,

    // == Misc ==
    pub config_dir: PathBuf,
}

impl App {
    pub fn new() -> Self {
        let config_dir = PathBuf::from(
            env::vars()
                .find(|x| x.0 == "config")
                .map(|x| x.1)
                .unwrap_or_else(|| "./data/config".to_owned()),
        );
        let cfg = Config::new(config_dir.join("config.cfg"));

        let db = Connection::open(&cfg.database_path).unwrap();
        db.pragma_update(None, "journal_mode", "WAL").unwrap();
        db.pragma_update(None, "synchronous", "NORMAL").unwrap();
        for i in [
            include_str!("sql/create_article_views.sql"),
            include_str!("sql/create_article_likes.sql"),
        ] {
            db.execute(i, []).unwrap();
        }

        Self {
            config: cfg,
            database: Mutex::new(db),

            articles: WritingCache::new_empty(),
            redirects: RwLock::new(HashMap::new()),
            analytics_data: Mutex::new(HashMap::new()),
            config_dir,
        }
    }

    pub fn reload_articles(&self) {
        self.articles.reload_articles(&self.config);
    }

    pub fn reload_links(&self) {
        let mut links = self.redirects.write();
        links.clear();

        let data = fs::read_to_string(self.config_dir.join("link.cfg")).unwrap();
        for i in data.lines() {
            if i.is_empty() || i.starts_with(';') || i.starts_with('#') {
                continue;
            }
            let mut parts = i.splitn(2, '=');
            let code = parts.next().unwrap().trim().to_owned();
            let link = parts.next().unwrap().trim().to_owned();
            links.insert(code, link);
        }
    }
}
