//! Babel 抓取器测试程序
//! 
//! 用于测试使用 define_scraper 宏实现的 Babel 抓取器

use std::error::Error;
use xwdoc::core::scraper::ScraperTrait;
use xwdoc::docs::babel::BabelScraper;

fn main() -> Result<(), Box<dyn Error>> {
    println!("创建简化版 Babel 抓取器...");
    // 创建文档输出路径
    let output_path = "./test_docs/babel_simplified";

    // 确保输出目录存在
    if !std::path::Path::new(output_path).exists() {
        std::fs::create_dir_all(output_path)?;
    }

    // 创建 Babel 抓取器
    let mut scraper = BabelScraper::new("7");
    
    // 打印抓取器信息
    println!("Babel 抓取器信息:");
    println!("  名称: {}", scraper.name());
    println!("  版本: {:?}", scraper.version());
    println!("  基础 URL: {}", scraper.base_url());
    println!("  输出路径: {}", scraper.output_path());
    println!("  文档类型: {:?}", scraper.doc_type());
    println!("  发布版本: {:?}", scraper.release());
    
    // 获取最新版本
    let latest_version = scraper.get_latest_version()?;
    println!("  最新版本: {}", latest_version);
    
    println!("\n运行简化版 Babel 抓取器...");
    scraper.run()?;

    println!("简化版 Babel 抓取器成功完成!");
    Ok(())
}
