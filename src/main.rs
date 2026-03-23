#[macro_use]
extern crate ego_tree;

mod custom2;
mod extractor;
mod parser;
mod util;

pub use extractor::ExtractorError;
pub use parser::{
    Argument, Field, Method, MethodArgs, Object, ObjectData, ParseError, Parsed, Type,
};
use serde::Serialize;
use std::{fs, path::PathBuf};

pub const CORE_TELEGRAM_URL: &str =
    "https://core.telegram.org";
pub const BOT_API_DOCS_URL: &str =
    "https://core.telegram.org/bots/api";

use extractor::Extractor;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Extractor: {0}")]
    Extractor(
        #[from]
        #[source]
        ExtractorError,
    ),
    #[error("Parser: {0}")]
    Parse(
        #[from]
        #[source]
        ParseError,
    ),
}

struct Serialized {
    content: String,
    path: String,
}

#[derive(Default)]
struct Indexer {
    publish_dir: PathBuf,
    inner: Vec<Serialized>,
}

pub fn get(html_doc: &str) -> Result<Parsed, Error> {
    let extractor = Extractor::from_str(html_doc);
    let extracted = extractor.extract()?;
    let parsed = parser::parse(extracted)?;
    Ok(parsed)
}

impl Indexer {
    fn new(publish_dir: &str) -> Self {
        Self {
            publish_dir: PathBuf::from(publish_dir),
            inner: vec![],
        }
    }

    fn add<T: Serialize>(&mut self, api: &T, path: String) -> anyhow::Result<()> {
        let content = serde_json::to_string_pretty(api)?;
        self.inner.push(Serialized {
            content,
            path: path.to_string(),
        });

        Ok(())
    }

    fn gen(self) -> anyhow::Result<()> {
        if !self.publish_dir.exists() {
            fs::create_dir_all(&self.publish_dir)?;
        }

        for Serialized { content, path } in self.inner {
            fs::write(self.publish_dir.join(&path), content)?;
        }

        Ok(())
    }
}


fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let url = std::env::args()
        .nth(1)
        .unwrap_or_else(|| BOT_API_DOCS_URL.to_string());

    let api = reqwest::blocking::get(&url)?.text()?;
    let parsed = get(&api)?;

    let mut indexer = Indexer::new("schema/");

    let custom_schema = custom2::generate(parsed);
    indexer.add(&custom_schema, "custom_v2.json".to_owned())?;

    indexer.gen()?;

    Ok(())
}
