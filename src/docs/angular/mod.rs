//! Angular 文档模块

mod scraper;
mod clean;
mod entries;

pub use scraper::AngularScraper;
pub use clean::AngularCleanHtmlFilter;
pub use entries::AngularEntriesFilter;

/// 注册 Angular 文档抓取器
pub fn register() {
    let scraper = AngularScraper::new("latest", "output/angular");
    crate::docs::registry::register_scraper("angular", scraper);
    println!("注册了 Angular 文档抓取器");
}
