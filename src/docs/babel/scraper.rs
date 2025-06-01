//! Babel 文档抓取器
//!
//! 使用 define_scraper 宏实现的 Babel 文档抓取器
//! 对应原始 Ruby 项目中的 Docs::Babel 类

use crate::docs::define_scraper;
use crate::core::scraper::ScraperTrait;
use std::error::Error;
use std::collections::HashMap;
use nipper::Document; // 使用 nipper 库进行 HTML 解析

// 使用 define_scraper 宏定义 Babel 抓取器
define_scraper!(
    name: BabelScraper,
    doc_name: "Babel",
    version: "latest",
    base_url: "https://babeljs.io/docs",
    output_path: "output",
    doc_type: "simple",
    release: "7.21.4",
    links: {
        "home" => "https://babeljs.io/",
        "code" => "https://github.com/babel/babel"
    },
    options: {
        "trailing_slash" => "true",
        "attribution" => "&copy; 2014-present Sebastian McKenzie<br>Licensed under the MIT License."
    },
    skip_patterns: [
        "/usage/",
        "/configuration/",
        "/learn/",
        "/v7-migration/",
        "/v7-migration-api/",
        "/editors/",
        "/presets/",
        "/caveats/",
        "/faq/",
        "/roadmap/"
    ]
);

/// 注册 Babel 文档抓取器
pub fn register() {
    // 创建并注册抓取器
    let scraper = BabelScraper::new("7");
    
    // 将抓取器注册到文档注册表中
    crate::docs::registry::register_scraper("babel", scraper);
    
    println!("注册了 Babel 文档抓取器");
}

// ScraperTrait 特征已经通过宏实现
/// 运行 Babel 文档抓取器
pub fn run_scraper(version: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 创建并运行 Babel 抓取器
    let mut scraper = BabelScraper::new(version);
    // 设置输出路径
    scraper.output_path = output_path.to_string();
    
    // 打印抓取器信息
    println!("Babel 抓取器信息:");
    println!("  名称: {}", scraper.name());
    println!("  版本: {:?}", scraper.version());
    println!("  基础 URL: {}", scraper.base_url());
    println!("  输出路径: {}", scraper.output_path());
    println!("  文档类型: {:?}", scraper.doc_type());
    println!("  发布版本: {:?}", scraper.release());
    
    // 运行抓取器
    scraper.run()
}
