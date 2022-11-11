use std::{path::PathBuf, sync::Arc};

use ahash::{HashMap, HashMapExt};
use parking_lot::RwLock;
use serde_json::json;
use unindent::unindent;

use crate::app::App;

use super::Article;

pub struct WritingCache {
    pub articles: RwLock<HashMap<String, Article>>,

    pub writing_cache: RwLock<String>,
    pub api_cache: RwLock<String>,
    pub rss_cache: RwLock<String>,
}

impl WritingCache {
    pub fn new_empty() -> Self {
        Self {
            articles: RwLock::new(HashMap::new()),
            writing_cache: RwLock::new(String::new()),
            api_cache: RwLock::new(String::new()),
            rss_cache: RwLock::new(String::new()),
        }
    }

    /// Reload articles from disk
    pub fn reload_articles(&self, app: Arc<App>) {
        let mut articles = self.articles.write();
        for i in Article::load_documents(PathBuf::from(&app.config.writing_path)) {
            let path = i.path.to_owned();
            let insert = articles.insert(path.to_owned(), i);

            if insert.is_some() {
                println!(
                    "[-] Article with path `{}` already defined. Overwriting.",
                    path
                )
            }
        }

        // Make Article Vec
        let mut article_vec = articles.iter().map(|x| x.1).collect::<Vec<_>>();
        article_vec.sort_unstable_by(|x, y| y.epoch.cmp(&x.epoch));

        // API Cache
        *self.api_cache.write() = json!(article_vec
            .iter()
            .cloned()
            .map(Article::jsonify)
            .collect::<Vec<_>>())
        .to_string();

        // Rss Cache
        let mut out = String::new();
        for i in article_vec.iter().filter(|x| !x.hidden) {
            out.push_str(i.rssify(&app.config.external_uri).as_str());
            out.push_str("\n\n");
        }

        out.pop();
        out.pop();

        *self.rss_cache.write() = unindent(
            format!(
                r#"<?xml version="1.0" encoding="UTF-8" ?>
            <rss version="2.0">
            <channel>
             <title>ConnorCode</title>
             <description>ConnorCode Articles</description>
             <link>{}</link>
             <copyright>Connor Slade</copyright>
             <language>en</language>
             <ttl>1800</ttl>

             {}

            </channel>
            </rss>"#,
                app.config.external_uri, out
            )
            .as_str(),
        );
    }
}
