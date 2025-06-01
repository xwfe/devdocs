//! 文档生成器模块
//!
//! 提供生成文档 JSON 文件的功能

use crate::core::error::{Error, Result};
use crate::core::scraper::ScraperTrait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 文档生成器
///
/// 用于生成文档 JSON 文件
pub struct DocGenerator {
    /// 输出目录
    output_dir: PathBuf,
    /// 文档抓取器
    scraper: Box<dyn ScraperTrait>,
    /// 条目列表
    entries: Vec<(String, String, String)>,
    /// 页面内容
    pages: HashMap<String, String>,
}

impl DocGenerator {
    /// 创建新的文档生成器
    pub fn new(output_dir: &Path, scraper: Box<dyn ScraperTrait>) -> Self {
        Self {
            output_dir: output_dir.to_path_buf(),
            scraper,
            entries: Vec::new(),
            pages: HashMap::new(),
        }
    }
    
    /// 运行生成器
    pub fn run(&mut self) -> Result<()> {
        // 创建输出目录
        self.create_output_dir()?;
        
        // 抓取文档
        self.scrape_doc()?;
        
        // 生成索引文件
        self.generate_index()?;
        
        // 生成数据库文件
        self.generate_db()?;
        
        // 生成元数据文件
        self.generate_meta()?;
        
        Ok(())
    }
    
    /// 创建输出目录
    fn create_output_dir(&self) -> Result<()> {
        // 获取文档信息
        let slug = self.scraper.name().to_lowercase();
        let version = self.scraper.version().unwrap_or("");
        
        // 创建文档目录
        let doc_dir = if version.is_empty() {
            self.output_dir.join(&slug)
        } else {
            self.output_dir.join(format!("{}{}", slug, version))
        };
        
        // 如果目录不存在，创建它
        if !doc_dir.exists() {
            fs::create_dir_all(&doc_dir)?;
        }
        
        Ok(())
    }
    
    /// 抓取文档
    fn scrape_doc(&mut self) -> Result<()> {
        // 运行抓取器
        self.scraper.run()?;
        
        // 在实际实现中，我们需要从抓取器中获取条目和页面内容
        // 由于我们没有实际的抓取逻辑，这里只是一个占位符
        
        Ok(())
    }
    
    /// 生成索引文件
    fn generate_index(&self) -> Result<()> {
        // 获取文档信息
        let slug = self.scraper.name().to_lowercase();
        let version = self.scraper.version().unwrap_or("");
        
        // 创建文档目录
        let doc_dir = if version.is_empty() {
            self.output_dir.join(&slug)
        } else {
            self.output_dir.join(format!("{}{}", slug, version))
        };
        
        // 创建索引数据
        let mut index_data = Vec::new();
        
        for (name, path, type_str) in &self.entries {
            index_data.push(json!({
                "name": name,
                "path": path,
                "type": type_str
            }));
        }
        
        // 写入索引文件
        let index_path = doc_dir.join("index.json");
        let index_json = serde_json::to_string_pretty(&index_data)?;
        fs::write(index_path, index_json)?;
        
        Ok(())
    }
    
    /// 生成数据库文件
    fn generate_db(&self) -> Result<()> {
        // 获取文档信息
        let slug = self.scraper.name().to_lowercase();
        let version = self.scraper.version().unwrap_or("");
        
        // 创建文档目录
        let doc_dir = if version.is_empty() {
            self.output_dir.join(&slug)
        } else {
            self.output_dir.join(format!("{}{}", slug, version))
        };
        
        // 创建数据库数据
        let mut db_data = HashMap::new();
        
        for (path, content) in &self.pages {
            db_data.insert(path, content);
        }
        
        // 写入数据库文件
        let db_path = doc_dir.join("db.json");
        let db_json = serde_json::to_string(&db_data)?;
        fs::write(db_path, db_json)?;
        
        Ok(())
    }
    
    /// 生成元数据文件
    fn generate_meta(&self) -> Result<()> {
        // 获取文档信息
        let slug = self.scraper.name().to_lowercase();
        let version = self.scraper.version().unwrap_or("");
        
        // 创建文档目录
        let doc_dir = if version.is_empty() {
            self.output_dir.join(&slug)
        } else {
            self.output_dir.join(format!("{}{}", slug, version))
        };
        
        // 创建元数据
        let meta_data = json!({
            "name": self.scraper.name(),
            "slug": slug,
            "version": version
        });
        
        // 写入元数据文件
        let meta_path = doc_dir.join("meta.json");
        let meta_json = serde_json::to_string_pretty(&meta_data)?;
        fs::write(meta_path, meta_json)?;
        
        Ok(())
    }
}

/// 生成文档
pub fn generate_doc(output_dir: &Path, scraper: Box<dyn ScraperTrait>) -> Result<()> {
    let mut generator = DocGenerator::new(output_dir, scraper);
    generator.run()
}

/// 生成所有文档
pub fn generate_all_docs(output_dir: &Path, scrapers: Vec<Box<dyn ScraperTrait>>) -> Result<()> {
    for scraper in scrapers {
        generate_doc(output_dir, scraper)?;
    }
    
    Ok(())
}
