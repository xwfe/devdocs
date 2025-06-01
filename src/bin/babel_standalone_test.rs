//! 独立测试 Babel 抓取器的功能

use std::error::Error;
use xwdoc::docs::babel::BabelScraper;

fn main() -> Result<(), Box<dyn Error>> {
    println!("创建 Babel 抓取器...");
    
    // 创建文档输出路径
    let output_path = "./test_docs/babel_standalone";

    // 确保输出目录存在
    if !std::path::Path::new(output_path).exists() {
        std::fs::create_dir_all(output_path)?;
    }

    // 创建 Babel 抓取器
    let mut scraper = BabelScraper::new("7");
    
    println!("Babel 抓取器信息:");
    println!("名称: {}", scraper.name);
    println!("版本: {}", scraper.version);
    println!("基础 URL: {}", scraper.base_url);
    println!("输出路径: {}", scraper.output_path);
    
    if let Some(doc_type) = &scraper.doc_type {
        println!("文档类型: {}", doc_type);
    }
    
    if let Some(release) = &scraper.release {
        println!("发布版本: {}", release);
    }
    
    println!("链接数量: {}", scraper.links.len());
    println!("选项数量: {}", scraper.options.len());
    println!("跳过模式数量: {}", scraper.skip_patterns.len());
    
    // 打印详细信息
    println!("\n链接详情:");
    for (key, value) in &scraper.links {
        println!("  {} => {}", key, value);
    }
    
    println!("\n选项详情:");
    for (key, value) in &scraper.options {
        println!("  {} => {}", key, value);
    }
    
    println!("\n跳过模式详情:");
    for pattern in &scraper.skip_patterns {
        println!("  {}", pattern);
    }
    
    println!("\nBabel 抓取器测试成功完成!");
    Ok(())
}
