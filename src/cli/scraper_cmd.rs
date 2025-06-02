//! 文档抓取命令处理

use std::error::Error;
use crate::docs::{init, get_scraper, get_scraper_names};

/// 列出所有可用的抓取器
pub fn list_scrapers() -> Result<(), Box<dyn Error>> {
    init()?;

    println!("可用的文档抓取器:");
    let names = get_scraper_names();
    if names.is_empty() {
        println!("  没有可用的文档抓取器");
    } else {
        for name in names {
            if let Some(scraper) = get_scraper(&name) {
                let version = scraper.version().unwrap_or("latest");
                println!("  {} - {} 文档抓取器 (版本: {})", name, scraper.name(), version);
            }
        }
    }

    Ok(())
}

/// 运行指定的抓取器
pub async fn run_scraper(name: &str, version: &str, output: Option<&str>) -> Result<(), Box<dyn Error>> {
    init()?;

    println!("运行抓取器: {} (版本: {})", name, version);

    let output_str = output.unwrap_or("docs");

    crate::scrape_async(name, version, output_str).await
}
