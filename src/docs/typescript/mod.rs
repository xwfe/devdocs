//! TypeScript 文档模块
//! 
//! 包含 TypeScript 文档的抓取器和过滤器实现

mod scraper;

pub use scraper::TypeScriptScraper;

/// 注册 TypeScript 文档抓取器
pub fn register() {
    let scraper = TypeScriptScraper::new("latest", "output/typescript");
    crate::docs::registry::register_scraper("typescript", scraper);
    println!("注册了 TypeScript 文档抓取器");
}
