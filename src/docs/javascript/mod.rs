//! JavaScript 文档模块
//!
//! 包含 JavaScript 文档的抓取器和过滤器实现

mod clean;
mod entries;
mod scraper;

pub use clean::JavaScriptCleanHtmlFilter;
pub use entries::JavaScriptEntriesFilter;
pub use scraper::JavaScriptScraper;

/// 注册 JavaScript 文档抓取器
pub fn register() {
    let scraper = JavaScriptScraper::new("latest", "output/javascript");
    crate::docs::registry::register_scraper("javascript", scraper);
    println!("注册了 JavaScript 文档抓取器");
}
