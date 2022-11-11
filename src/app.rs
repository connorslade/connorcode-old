use std::{env, path::PathBuf};

use ahash::{HashMap, HashMapExt};
use parking_lot::{Mutex, RwLock};
use rusqlite::Connection;

use crate::{
    analytics::Stats,
    config::Config,
    writing::{self, Article},
};

pub struct App {
    // == App Styff ==
    /// App Config
    pub config: Config,

    /// Databse Connection
    pub database: Mutex<Connection>,

    /// Articles String -> Article
    pub articles: RwLock<HashMap<String, Article>>,

    /// Current analytics_data (cleared on dump)
    pub analytics_data: Mutex<HashMap<String, Vec<Stats>>>,
}

impl App {
    pub fn new() -> Self {
        let cfg_path = env::vars()
            .find(|x| x.0 == "config")
            .map(|x| x.1)
            .unwrap_or_else(|| "./data/config/config.cfg".to_owned());
        let cfg = Config::new(cfg_path);

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

            articles: RwLock::new(HashMap::new()),
            analytics_data: Mutex::new(HashMap::new()),
        }
    }

    /// Reload articles from disk
    pub fn reload_articles(&self) {
        let mut articles = self.articles.write();
        for i in Article::load_documents(PathBuf::from(&self.config.writing_path)) {
            let path = i.path.to_owned();
            let insert = articles.insert(path.to_owned(), i);

            if insert.is_some() {
                println!("[-] Article with path `{}` already defined. Overwriting.", path)
            }
        }
    }
}
