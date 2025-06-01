//! 文档模块
//!
//! 提供各种文档的实现和管理

use std::error::Error;

pub mod autoload;
pub mod documentation;
pub mod generator;
pub mod registry;

// 导入具体文档模块
pub mod babel;
pub mod css;
pub mod html;
pub mod javascript;
pub mod rust;
pub mod typescript;

// 重新导出
pub use documentation::Documentation;
pub use crate::core::macros::doc_macros::*;
pub use registry::{
    add_doc, all_docs, find_doc, find_doc_with_version, 
    generate_manifest, get_registry, get_scraper, 
    load_docs_from_disk, register_all_scrapers, register_scraper,
};

// 重新导出宏
pub use crate::{create_entries_filter, create_filter, register_doc};

/// 初始化文档系统
///
/// 注册所有可用的文档抓取器
pub fn init() -> crate::core::error::Result<()> {
    // 注册具体文档
    babel::register();
    
    // 可以在这里添加更多文档的注册
    
    // 注册所有抓取器
    register_all_scrapers()?;
    
    Ok(())
}

/// 运行文档抓取器
pub fn run_scraper(name: &str, version: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if let Some(mut scraper) = get_scraper(name) {
        // 设置输出路径
        // 注意：这里我们不能直接设置 output_path，因为 ScraperTrait 没有提供 setter 方法
        // 在具体的抓取器实现中，如 babel/scraper.rs 中的 run_scraper 函数会处理这个参数
        
        // 如果提供了版本信息，可以在这里使用
        // 注意：同样的原因，我们不能直接设置 version
        // 打印抓取器信息
        println!("抓取器信息:");
        println!("  名称: {}", scraper.name());
        println!("  版本: {:?}", scraper.version());
        println!("  基础 URL: {}", scraper.base_url());
        println!("  输出路径: {}", scraper.output_path());
        
        if let Some(doc_type) = scraper.doc_type() {
            println!("  文档类型: {}", doc_type);
        }
        
        if let Some(release) = scraper.release() {
            println!("  发布版本: {}", release);
        }
        
        // 获取最新版本
        match scraper.get_latest_version() {
            Ok(latest_version) => println!("  最新版本: {}", latest_version),
            Err(e) => println!("  获取最新版本失败: {}", e),
        }
        
        // 运行抓取器
        println!("\n运行抓取器: {}", name);
        scraper.run()
    } else {
        Err(Box::new(crate::core::error::Error::Message(format!("未找到文档抓取器: {}", name))))
    }
}

/// 列出所有可用的文档
pub fn list_docs() -> Vec<String> {
    all_docs().into_iter().map(|doc| doc.slug).collect()
}

/// 加载文档系统
///
/// 从磁盘加载所有文档并生成清单
pub fn load(docs_path: &str) -> crate::core::error::Result<()> {
    // 初始化文档系统
    init()?;
    
    // 从磁盘加载文档
    load_docs_from_disk(docs_path)?;
    
    // 生成清单
    generate_manifest(docs_path)?;
    
    Ok(())
}
