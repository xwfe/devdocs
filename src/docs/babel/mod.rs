//! Babel 文档抓取器

mod clean;
mod entries;
mod scraper;

use std::error::Error;

pub use clean::BabelCleanHtmlFilter;
pub use entries::BabelEntriesFilter;
pub use scraper::{BabelScraper, register as register_simplified, run_scraper};

/// 注册 Babel 文档抓取器
pub fn register() {
    // 创建并注册抓取器
    let scraper = BabelScraper::new("7");
    
    // 将抓取器注册到文档注册表中
    crate::docs::registry::register_scraper("babel", scraper);
    
    // 注册简化版 Babel 抓取器
    register_simplified();
    
    println!("注册了 Babel 文档抓取器");
}

/// 测试运行简化版 Babel 抓取器
pub fn test_simplified_scraper() -> Result<(), Box<dyn std::error::Error>> {
    println!("测试运行简化版 Babel 抓取器...");
    // 使用默认版本和输出路径
    run_scraper("7", "output/babel")
}
