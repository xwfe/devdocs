//! 文档发现模块
//!
//! 提供自动发现和加载文档的功能

use crate::core::error::{Error, Result};
use crate::docs::autoload::register_doc;
use crate::docs::registry_new::Documentation;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 文档发现器
///
/// 用于自动发现和加载文档
pub struct DocDiscovery {
    /// 文档目录
    docs_dir: PathBuf,
    /// 已发现的文档
    discovered_docs: HashMap<String, Documentation>,
}

impl DocDiscovery {
    /// 创建新的文档发现器
    pub fn new(docs_dir: &Path) -> Self {
        Self {
            docs_dir: docs_dir.to_path_buf(),
            discovered_docs: HashMap::new(),
        }
    }
    
    /// 扫描文档目录
    pub fn scan(&mut self) -> Result<()> {
        // 检查文档目录是否存在
        if !self.docs_dir.exists() || !self.docs_dir.is_dir() {
            return Err(Error::Message(format!("文档目录不存在: {:?}", self.docs_dir)).into());
        }
        
        // 清空已发现的文档
        self.discovered_docs.clear();
        
        // 遍历文档目录
        let entries = fs::read_dir(&self.docs_dir)?;
        
        for entry_result in entries {
            let entry = entry_result?;
            let path = entry.path();
            
            if path.is_dir() {
                // 尝试加载文档
                if let Some(doc) = self.load_doc(&path) {
                    self.discovered_docs.insert(doc.slug.clone(), doc);
                }
            }
        }
        
        Ok(())
    }
    
    /// 加载文档
    fn load_doc(&self, path: &Path) -> Option<Documentation> {
        // 获取目录名
        let dirname = path.file_name()?.to_str()?;
        
        // 解析目录名
        let (slug, version) = if dirname.contains('~') {
            let parts: Vec<&str> = dirname.split('~').collect();
            (parts[0].to_string(), parts[1].to_string())
        } else {
            (dirname.to_string(), String::new())
        };
        
        // 检查必要的文件是否存在
        let index_path = path.join("index.json");
        let meta_path = path.join("meta.json");
        let db_path = path.join("db.json");
        
        if !index_path.exists() || !db_path.exists() {
            return None;
        }
        
        // 提取基本信息
        let index_size = fs::metadata(&index_path).map(|m| m.len() as usize).unwrap_or(0);
        let db_size = fs::metadata(&db_path).map(|m| m.len() as usize).unwrap_or(0);
        
        // 获取修改时间
        let mtime = fs::metadata(path)
            .and_then(|m| m.modified())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);
        
        // 创建文档信息
        let mut doc = Documentation::new(&slug, &slug, &version)
            .with_mtime(mtime)
            .with_db_size(db_size)
            .with_index_size(index_size);
        
        // 尝试读取元数据文件
        if meta_path.exists() {
            if let Ok(meta_content) = fs::read_to_string(&meta_path) {
                if let Ok(meta_json) = serde_json::from_str::<serde_json::Value>(&meta_content) {
                    if let Some(release) = meta_json.get("release").and_then(|v| v.as_str()) {
                        doc = doc.with_release(release);
                    }
                    if let Some(name) = meta_json.get("name").and_then(|v| v.as_str()) {
                        doc.name = name.to_string();
                    }
                }
            }
        }
        
        Some(doc)
    }
    
    /// 获取已发现的文档
    pub fn get_docs(&self) -> Vec<Documentation> {
        self.discovered_docs.values().cloned().collect()
    }
    
    /// 获取特定文档
    pub fn get_doc(&self, slug: &str) -> Option<&Documentation> {
        self.discovered_docs.get(slug)
    }
    
    /// 加载所有已发现的文档
    pub fn load_all(&self) -> Result<()> {
        for doc in self.discovered_docs.values() {
            // 在实际实现中，我们需要根据文档类型创建适当的抓取器
            // 由于 Rust 不支持像 Ruby 那样的动态加载，这里只是一个占位符
            println!("加载文档: {} (版本: {})", doc.slug, doc.version);
        }
        
        Ok(())
    }
}

/// 扫描文档目录
pub fn scan_docs_dir(docs_dir: &Path) -> Result<Vec<Documentation>> {
    let mut discovery = DocDiscovery::new(docs_dir);
    discovery.scan()?;
    Ok(discovery.get_docs())
}

/// 自动发现并加载所有文档
pub fn discover_and_load_docs(docs_dir: &Path) -> Result<()> {
    let mut discovery = DocDiscovery::new(docs_dir);
    discovery.scan()?;
    discovery.load_all()?;
    Ok(())
}

/// 自动发现并注册文档模块
pub fn discover_doc_modules() -> Result<()> {
    // 在实际实现中，我们需要扫描 src/docs 目录，查找所有文档模块
    // 由于 Rust 不支持像 Ruby 那样的动态加载，这里只是一个占位符
    
    // 手动注册已知的文档模块
    crate::docs::react::register();
    crate::docs::example::register();
    
    // 可以在这里添加更多文档的注册
    
    Ok(())
}
