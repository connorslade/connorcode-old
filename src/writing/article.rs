use std::fs::{self, DirEntry};
use std::path::PathBuf;

use chrono::{Date, NaiveTime, TimeZone, Utc};
use serde::Serialize;
use serde_json::{json, Value};
use simple_config_parser::Config;
use unindent::unindent;

use crate::color::{color, Color};

#[derive(Serialize)]
pub struct Article {
    /// Web path
    pub path: String,
    /// .md file path
    pub file_path: PathBuf,

    pub title: String,
    pub date: String,
    pub author: String,
    pub description: String,

    #[serde(skip)]
    pub epoch: Date<Utc>,
    pub tags: Vec<String>,
    pub hidden: bool,
    pub words: usize,
    pub icon: String,
}

impl Article {
    // Note: Output is un-sorted
    pub fn load_documents(path: PathBuf) -> Vec<Self> {
        let mut out = Vec::new();

        let files = fs::read_dir(path).expect("Error Reading Writing Dir");
        for i in files {
            let i = i.unwrap();

            if i.path().is_dir() {
                out.append(&mut Self::load_documents(i.path()));
            }

            if i.file_name().to_string_lossy().to_lowercase() == "readme.md" {
                continue;
            }

            if let Some(doc) = Self::load_document(i) {
                out.push(doc);
            };
        }

        out
    }

    fn load_document(i: DirEntry) -> Option<Self> {
        if i.path().extension()?.to_str()?.to_lowercase().as_str() != "md" {
            return None;
        }

        let data = fs::read_to_string(i.path()).expect("Error Reading a Writing");
        let mut parts = data.splitn(2, "---");

        let cfg = Config::new()
            .text(parts.next().unwrap())
            .expect("Error Parseing a Writing Config");

        let words = parts.next().unwrap().split_whitespace().count();

        let tags = cfg
            .get_str("@Tags")
            .unwrap_or_default()
            .split(',')
            .map(|x| x.trim().to_owned())
            .collect();

        let icon = cfg
            .get_str("@Icon")
            .unwrap_or_else(|_| "file-text-o".to_owned());

        let file_path = i.path();
        let path = safe_config(&cfg, "Path", i.path());
        let title = safe_config(&cfg, "Title", i.path());
        let date = safe_config(&cfg, "Date", i.path())?;
        let description = safe_config(&cfg, "Description", i.path());
        let author = cfg
            .get_str("@Author")
            .unwrap_or_else(|_| "Connor Slade".to_owned());
        let hidden = cfg.get("@Hidden").unwrap_or(false);

        let epoch_parts = date.splitn(3, '-').collect::<Vec<_>>();
        let epoch = Utc.ymd(
            epoch_parts[2].parse().unwrap(),
            epoch_parts[0].parse().unwrap(),
            epoch_parts[1].parse().unwrap(),
        );

        Some(Self {
            file_path,
            path: path?,
            title: title?,
            description: description?,
            date,
            words,
            author,
            epoch,
            hidden,
            tags,
            icon,
        })
    }

    pub fn jsonify(&self) -> Value {
        json!({
            "name": self.title,
            "disc": self.description,
            "date": self.date,
            "icon": self.icon,
            "link": "/writing/".to_owned() + &self.path,
            "tags": self.tags,
            "reading": (self.words as f64 / 3.5).round() as u64
        })
    }

    pub fn rssify(&self, external_uri: &str) -> String {
        let date = self.epoch.and_time(NaiveTime::from_hms(0, 0, 0)).unwrap();

        unindent(
            format!(
                r#"<item>
                     <title>{}</title>
                     <description>{}</description>
                     <pubDate>{}</pubDate>
                     <link>{}/writing/{}</link>
                 </item>"#,
                self.title,
                self.description,
                date.to_rfc2822(),
                external_uri,
                self.path
            )
            .as_str(),
        )
    }
}

fn safe_config(cfg: &Config, key: &str, path: PathBuf) -> Option<String> {
    let value = cfg.get_str(&("@".to_owned() + key));
    if value.is_err() {
        println!(
            "{}",
            color(
                format!(
                    "[-] Required Field `{}` not defiled on document `{}`",
                    key,
                    path.to_string_lossy()
                ),
                Color::Red
            )
        );
    }

    value.ok()
}
