//! Doc 模块
//!
//! 严格对齐原版 Ruby 项目中的 core/doc.rb 实现

use crate::core::models::{Doc, Entry};
use crate::core::page_db::PageDb;
use crate::core::Result;
use crate::core::entry_index::EntryIndex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 文档文件名常量
pub const INDEX_FILENAME: &str = "index.json";
pub const DB_FILENAME: &str = "db.json";
pub const META_FILENAME: &str = "meta.json";

/// 设置错误
#[derive(Debug)]
pub struct SetupError {
    pub message: String,
}

impl std::fmt::Display for SetupError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SetupError {}

/// 文档类
/// 
/// 对应原版 Ruby 的 Doc 类
pub struct Doc {
    // 类级别属性
    name: Option<String>,
    slug: Option<String>,
    doc_type: Option<String>,
    release: Option<String>,
    is_abstract: bool,
    links: Option<HashMap<String, String>>,
    
    // 版本相关
    version: Option<String>,
    versions: Option<Vec<Box<Doc>>>,
}

impl Doc {
    /// 创建新的文档
    /// 
    /// 对应原版 Ruby 的 new 方法
    pub fn new() -> Result<Self, SetupError> {
        let doc = Self {
            name: None,
            slug: None,
            doc_type: None,
            release: None,
            is_abstract: false,
            links: None,
            version: None,
            versions: None,
        };
        
        if doc.is_abstract {
            return Err(SetupError {
                message: format!("{} is an abstract class and cannot be instantiated.", 
                    std::any::type_name::<Self>())
            });
        }
        
        Ok(doc)
    }

    /// 继承方法（模拟类继承）
    /// 
    /// 对应原版 Ruby 的 inherited 方法
    pub fn inherited(&self, subclass_type: Option<String>) -> Self {
        Self {
            name: self.name.clone(),
            slug: self.slug.clone(),
            doc_type: subclass_type.or_else(|| self.doc_type.clone()),
            release: self.release.clone(),
            is_abstract: self.is_abstract,
            links: self.links.clone(),
            version: None,
            versions: None,
        }
    }

    /// 版本管理
    /// 
    /// 对应原版 Ruby 的 version 方法
    pub fn version<F>(&mut self, version: Option<String>, block: Option<F>) -> Option<Box<Doc>>
    where
        F: FnOnce(&mut Doc),
    {
        if let Some(block_fn) = block {
            let mut klass = Box::new(self.clone());
            klass.name = self.name.clone();
            klass.slug = self.slug.clone();
            klass.version = version;
            klass.release = self.release.clone();
            klass.links = self.links.clone();
            
            block_fn(&mut klass);
            
            if self.versions.is_none() {
                self.versions = Some(Vec::new());
            }
            self.versions.as_mut().unwrap().push(klass.clone());
            
            Some(klass)
        } else {
            None
        }
    }

    /// 设置版本
    /// 
    /// 对应原版 Ruby 的 version= 方法
    pub fn set_version(&mut self, version: String) {
        self.version = Some(version);
    }

    /// 获取所有版本
    /// 
    /// 对应原版 Ruby 的 versions 方法
    pub fn versions(&self) -> Vec<&Doc> {
        if let Some(versions) = &self.versions {
            versions.iter().map(|v| v.as_ref()).collect()
        } else {
            vec![self]
        }
    }

    /// 检查是否有版本
    /// 
    /// 对应原版 Ruby 的 version? 方法
    pub fn has_version(&self) -> bool {
        self.version.is_some()
    }

    /// 检查是否有多个版本
    /// 
    /// 对应原版 Ruby 的 versioned? 方法
    pub fn is_versioned(&self) -> bool {
        self.versions.is_some()
    }

    /// 获取名称
    /// 
    /// 对应原版 Ruby 的 name 方法
    pub fn name(&self) -> String {
        self.name.clone().unwrap_or_else(|| {
            // 模拟 Ruby 的 super.demodulize
            std::any::type_name::<Self>()
                .split("::")
                .last()
                .unwrap_or("Doc")
                .to_string()
        })
    }

    /// 获取 slug
    /// 
    /// 对应原版 Ruby 的 slug 方法
    pub fn slug(&self) -> Result<String, String> {
        let base_slug = self.slug.clone()
            .or_else(|| self.default_slug())
            .ok_or_else(|| "slug is required".to_string())?;
        
        if self.has_version() {
            Ok(format!("{}~{}", base_slug, self.version_slug()))
        } else {
            Ok(base_slug)
        }
    }

    /// 获取版本 slug
    /// 
    /// 对应原版 Ruby 的 version_slug 方法
    pub fn version_slug(&self) -> String {
        if let Some(version) = &self.version {
            let mut slug = version.to_lowercase();
            slug = slug.replace('+', "p");
            slug = slug.replace('#', "s");
            // 替换非字母数字字符为下划线
            slug = slug.chars()
                .map(|c| if c.is_alphanumeric() || c == '_' || c == '.' { c } else { '_' })
                .collect();
            slug
        } else {
            String::new()
        }
    }

    /// 获取路径
    /// 
    /// 对应原版 Ruby 的 path 方法
    pub fn path(&self) -> Result<String, String> {
        self.slug()
    }

    /// 获取索引路径
    /// 
    /// 对应原版 Ruby 的 index_path 方法
    pub fn index_path(&self) -> Result<String, String> {
        Ok(format!("{}/{}", self.path()?, INDEX_FILENAME))
    }

    /// 获取数据库路径
    /// 
    /// 对应原版 Ruby 的 db_path 方法
    pub fn db_path(&self) -> Result<String, String> {
        Ok(format!("{}/{}", self.path()?, DB_FILENAME))
    }

    /// 获取元数据路径
    /// 
    /// 对应原版 Ruby 的 meta_path 方法
    pub fn meta_path(&self) -> Result<String, String> {
        Ok(format!("{}/{}", self.path()?, META_FILENAME))
    }

    /// 转换为 JSON
    /// 
    /// 对应原版 Ruby 的 as_json 方法
    pub fn as_json(&self) -> DocJson {
        let mut json = DocJson {
            name: self.name(),
            slug: self.slug().unwrap_or_default(),
            doc_type: self.doc_type.clone(),
            links: None,
            version: None,
            release: None,
        };

        if let Some(links) = &self.links {
            if !links.is_empty() {
                json.links = Some(links.clone());
            }
        }

        if self.has_version() || self.version.is_some() {
            json.version = self.version.clone();
        }

        if let Some(release) = &self.release {
            if !release.is_empty() {
                json.release = Some(release.clone());
            }
        }

        json
    }

    /// 存储单个页面
    /// 
    /// 对应原版 Ruby 的 store_page 方法
    pub fn store_page<S>(&self, store: &mut S, id: &str) -> bool
    where
        S: Store,
    {
        let mut index = EntryIndex::new();
        let mut pages = PageDb::new();

        match store.open(&self.path().unwrap_or_default()) {
            Ok(_) => {
                if let Ok(page) = self.build_page(id) {
                    if self.should_store_page(&page) {
                        index.add_entries(page.entries);
                        pages.add(page.path, page.output.clone());
                        
                        let _ = self.store_index(store, INDEX_FILENAME, &index, false);
                        let _ = self.store_index_pages(store, DB_FILENAME, &pages, false);
                        let _ = store.write(&page.store_path, &page.output);
                        
                        true
                    } else {
                        false
                    }
                } else {
                    false
                }
            }
            Err(error) => {
                eprintln!("ERROR: {}", error);
                false
            }
        }
    }

    /// 存储所有页面
    /// 
    /// 对应原版 Ruby 的 store_pages 方法
    pub fn store_pages<S>(&self, store: &mut S) -> bool
    where
        S: Store,
    {
        let mut index = EntryIndex::new();
        let mut pages = PageDb::new();

        match store.replace(&self.path().unwrap_or_default()) {
            Ok(_) => {
                let mut has_pages = false;
                self.build_pages(|page| {
                    if !self.should_store_page(&page) {
                        return;
                    }
                    
                    let _ = store.write(&page.store_path, &page.output);
                    index.add_entries(page.entries);
                    pages.add(page.path, page.output);
                    has_pages = true;
                });

                if has_pages {
                    let _ = self.store_index(store, INDEX_FILENAME, &index, true);
                    let _ = self.store_index_pages(store, DB_FILENAME, &pages, true);
                    let _ = self.store_meta(store);
                    true
                } else {
                    false
                }
            }
            Err(error) => {
                eprintln!("ERROR: {}", error);
                false
            }
        }
    }

    /// 构建页面（需要子类实现）
    /// 
    /// 对应原版 Ruby 的 build_page 方法
    pub fn build_page(&self, _id: &str) -> Result<PageData, String> {
        Err("NotImplementedError".to_string())
    }

    /// 构建所有页面（需要子类实现）
    /// 
    /// 对应原版 Ruby 的 build_pages 方法
    pub fn build_pages<F>(&self, _block: F)
    where
        F: FnMut(PageData),
    {
        // NotImplementedError
    }

    /// 获取爬虫版本
    /// 
    /// 对应原版 Ruby 的 get_scraper_version 方法
    pub fn get_scraper_version(&self, _opts: &HashMap<String, String>) -> Option<String> {
        // 如果定义了 options[:release]，返回它
        // 否则返回 DevDocs 生产环境中文档最后修改的时间戳
        None // 简化实现
    }

    /// 获取最新版本（需要子类实现）
    /// 
    /// 对应原版 Ruby 的 get_latest_version 方法
    pub fn get_latest_version(&self, _opts: &HashMap<String, String>) -> Result<String, String> {
        Err("NotImplementedError".to_string())
    }

    /// 检查过时状态
    /// 
    /// 对应原版 Ruby 的 outdated_state 方法
    pub fn outdated_state(&self, scraper_version: &str, latest_version: &str) -> String {
        let scraper_parts: Vec<i32> = scraper_version
            .split(|c: char| c == '-' || c == '.')
            .filter_map(|s| s.parse().ok())
            .collect();
        
        let latest_parts: Vec<i32> = latest_version
            .split(|c: char| c == '-' || c == '.')
            .filter_map(|s| s.parse().ok())
            .collect();

        // 只检查前两个部分，第三部分是补丁更新
        for i in 0..2 {
            if i >= scraper_parts.len() || i >= latest_parts.len() {
                break;
            }
            
            if i == 0 && latest_parts[i] > scraper_parts[i] {
                return "Outdated major version".to_string();
            }
            
            if i == 1 && latest_parts[i] > scraper_parts[i] {
                if (latest_parts[0] == 0 && scraper_parts[0] == 0) ||
                   (latest_parts[0] == 1 && scraper_parts[0] == 1) {
                    return "Outdated major version".to_string();
                } else {
                    return "Outdated minor version".to_string();
                }
            }
            
            if latest_parts[i] < scraper_parts[i] {
                return "Up-to-date".to_string();
            }
        }

        "Up-to-date".to_string()
    }

    // 私有方法

    /// 默认 slug
    /// 
    /// 对应原版 Ruby 的 default_slug 方法
    fn default_slug(&self) -> Option<String> {
        let name = self.name();
        if name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            Some(name.to_lowercase())
        } else {
            None
        }
    }

    /// 检查是否应该存储页面
    /// 
    /// 对应原版 Ruby 的 store_page? 方法
    fn should_store_page(&self, page: &PageData) -> bool {
        !page.entries.is_empty()
    }

    /// 存储索引
    /// 
    /// 对应原版 Ruby 的 store_index 方法
    fn store_index<S>(&self, store: &mut S, filename: &str, index: &EntryIndex, read_write: bool) -> Result<(), String>
    where
        S: Store,
    {
        let old_json = if read_write {
            if let Ok(json) = store.read(filename) {
                Some(json)
            } else {
                None
            }
        } else {
            None
        };
        
        let new_json = index.to_json();
        
        // 这里应该有 instrument 调用，但简化实现
        
        if read_write {
            store.write(filename, &new_json).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }

    /// 存储页面索引
    /// 
    /// 对应原版 Ruby 的 store_index 方法 (用于 PageDb)
    fn store_index_pages<S>(&self, store: &mut S, filename: &str, pages: &PageDb, read_write: bool) -> Result<(), String>
    where
        S: Store,
    {
        let old_json = if read_write {
            if let Ok(json) = store.read(filename) {
                Some(json)
            } else {
                None
            }
        } else {
            None
        };
        
        let new_json = pages.to_json();
        
        if read_write {
            store.write(filename, &new_json).map_err(|e| e.to_string())?;
        }
        
        Ok(())
    }

    /// 存储元数据
    /// 
    /// 对应原版 Ruby 的 store_meta 方法
    fn store_meta<S>(&self, store: &mut S) -> Result<(), String>
    where
        S: Store,
    {
        let mut json = self.as_json();
        // 添加时间戳和数据库大小
        // json.mtime = Some(chrono::Utc::now().timestamp());
        // json.db_size = Some(store.size(DB_FILENAME));
        
        let json_str = serde_json::to_string(&json).map_err(|e| e.to_string())?;
        store.write(META_FILENAME, &json_str).map_err(|e| e.to_string())?;
        
        Ok(())
    }
}

impl Clone for Doc {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            slug: self.slug.clone(),
            doc_type: self.doc_type.clone(),
            release: self.release.clone(),
            is_abstract: self.is_abstract,
            links: self.links.clone(),
            version: self.version.clone(),
            versions: None, // 不克隆版本列表以避免循环引用
        }
    }
}

/// 页面数据
/// 
/// 对应原版 Ruby 中页面构建返回的数据结构
#[derive(Debug, Clone)]
pub struct PageData {
    pub entries: Vec<Entry>,
    pub path: String,
    pub output: String,
    pub store_path: String,
}

/// JSON 序列化结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DocJson {
    pub name: String,
    pub slug: String,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub doc_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release: Option<String>,
}

/// 存储接口
/// 
/// 对应原版 Ruby 中的存储操作
pub trait Store {
    type Error: std::fmt::Display;
    
    fn open(&mut self, path: &str) -> std::result::Result<(), Self::Error>;
    fn replace(&mut self, path: &str) -> std::result::Result<(), Self::Error>;
    fn read(&self, filename: &str) -> std::result::Result<String, Self::Error>;
    fn write(&mut self, filename: &str, content: &str) -> std::result::Result<(), Self::Error>;
    fn size(&self, filename: &str) -> usize;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_doc_creation() {
        let doc = Doc::new().unwrap();
        assert_eq!(doc.name(), "Doc");
        assert!(!doc.has_version());
    }

    #[test]
    fn test_version_slug() {
        let mut doc = Doc::new().unwrap();
        doc.set_version("1.2.3+beta#1".to_string());
        assert_eq!(doc.version_slug(), "1.2.3pbeta_1");
    }

    #[test]
    fn test_outdated_state() {
        let doc = Doc::new().unwrap();
        
        assert_eq!(doc.outdated_state("1.0.0", "2.0.0"), "Outdated major version");
        assert_eq!(doc.outdated_state("1.1.0", "1.2.0"), "Outdated minor version");
        assert_eq!(doc.outdated_state("1.1.1", "1.1.2"), "Up-to-date");
        assert_eq!(doc.outdated_state("2.0.0", "1.0.0"), "Up-to-date");
    }

    #[test]
    fn test_as_json() {
        let mut doc = Doc::new().unwrap();
        doc.name = Some("TestDoc".to_string());
        doc.slug = Some("testdoc".to_string());
        doc.set_version("1.0.0".to_string());
        
        let json = doc.as_json();
        assert_eq!(json.name, "TestDoc");
        assert_eq!(json.slug, "testdoc~1_0_0");
        assert_eq!(json.version, Some("1.0.0".to_string()));
    }
}