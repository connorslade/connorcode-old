use std::{env, sync::Arc};

use ahash::{HashMap, HashMapExt};
use parking_lot::Mutex;
use rusqlite::Connection;

use crate::{analytics::Stats, config::Config, writing::WritingCache};

pub struct App {
    // == App Styff ==
    /// App Config
    pub config: Config,

    /// Databse Connection
    pub database: Mutex<Connection>,

    /// Articles String -> Article
    pub articles: WritingCache,

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

            articles: WritingCache::new_empty(),
            analytics_data: Mutex::new(HashMap::new()),
        }
    }

    pub fn reload_articles(self: Arc<Self>) {
        self.articles.reload_articles(self.clone());
    }
}