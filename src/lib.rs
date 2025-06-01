//! DevDocs Rust 实现
//!
//! 提供轻量级的 API 文档浏览器功能

#![allow(unused_imports)]
#![allow(dead_code)]

pub mod app;
pub mod cli;
pub mod core;
pub mod docs;
pub mod scraper;
pub mod storage;
pub mod web;

// 重新导出宏
pub use docs::macros;
pub use docs::scraper_macros;

// 重新导出核心类型
pub use core::error::{Error, Result};
pub use core::filter_base::{Filter, FilterContext, FilterStack};
pub use core::scraper::{Scraper, ScraperTrait};

// 重新导出文档类型
pub use docs::documentation::Documentation;
pub use docs::registry::{
    add_doc, all_docs, find_doc, find_doc_with_version, 
    generate_manifest, get_registry, get_scraper, 
    load_docs_from_disk, register_all_scrapers, register_scraper,
};

/// 初始化文档系统
pub fn init() -> Result<()> {
    docs::init()
}

/// 运行文档抓取器
pub fn run_scraper(name: &str, version: &str, output_path: &str) -> Result<()> {
    docs::run_scraper(name, version, output_path)
}

/// 列出所有可用的文档
pub fn list_docs() -> Vec<String> {
    docs::list_docs()
}

/// 加载文档系统
pub fn load(docs_path: &str) -> Result<()> {
    docs::load(docs_path)
}

/// 生成文档
pub fn generate_doc(name: &str, version: &str, output_path: &str) -> Result<()> {
    if let Some(scraper) = get_scraper(name) {
        docs::generator::generate_doc(
            &std::path::Path::new(output_path),
            scraper
        )
    } else {
        Err(Error::Message(format!("未找到文档抓取器: {}", name)).into())
    }
}

/// 异步运行文档抓取器
pub async fn scrape_async(name: &str, version: &str, output_path: &str) -> std::result::Result<(), Box<dyn std::error::Error>> {
    run_scraper(name, version, output_path)?;
    Ok(())
}

/// 获取版本号
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// 获取应用名称
pub fn app_name() -> &'static str {
    env!("CARGO_PKG_NAME")
}

/// 获取应用描述
pub fn app_description() -> &'static str {
    env!("CARGO_PKG_DESCRIPTION")
}

/// 获取应用作者
pub fn app_authors() -> &'static str {
    env!("CARGO_PKG_AUTHORS")
}
