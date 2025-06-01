//! 核心模块
//!
//! 严格对齐原版 Ruby 项目中的 lib/docs/core 目录结构

pub mod config;
pub mod db;
pub mod doc;
pub mod entry_index;
pub mod error;
pub mod filter_base;
pub mod filter_stack;
pub mod filters;
pub mod indexer;
pub mod instrumentable;
pub mod macros;
pub mod models;
pub mod page_db;
pub mod parser;
pub mod request;
pub mod requester;
pub mod response;
pub mod scraper;
pub mod search;
pub mod types;
pub mod url;
pub mod url_helper;

// 重新导出主要类型
pub use db::{Database, DatabaseManager, Page};
pub use doc::{Doc, DocJson, PageData, SetupError, Store};
pub use entry_index::{EntryIndex, IndexJson};
pub use error::{Error, Result};
pub use filter_base::{Filter, FilterContext};
pub use filter_stack::{FilterIndex, FilterStack};
pub use indexer::{Entry as IndexEntry, Index, Indexer};
pub use models::{Entry, EntryInvalid, EntryJson, Type, TypeJson};
pub use page_db::PageDb;
pub use parser::HtmlParser;
pub use request::{Request, RequestOptions};
pub use requester::Requester;
pub use response::Response;
pub use scraper::ScraperTrait;
pub use search::{SearchEngine, SearchResult};
pub use types::{ModifiedTime, Release, Size, Slug};
pub use url::URL;
pub use url_helper::{is_relative_url, to_absolute_url, normalize_url};
