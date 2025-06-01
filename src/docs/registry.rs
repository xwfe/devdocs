//! 文档注册表管理（新版）
//!
//! 提供自动发现和加载文档的功能，遵循原版 Ruby 项目的设计模式

use crate::core::error::{Error, Result};
use crate::core::scraper::ScraperTrait;
use crate::docs::autoload::get_all_doc_names;
use std::collections::HashMap;
use std::path::Path;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// 文档信息
#[derive(Debug, Clone)]
pub struct Documentation {
    /// 文档名称
    pub name: String,
    /// 文档别名
    pub slug: String,
    /// 文档版本
    pub version: String,
    /// 文档发布版本
    pub release: String,
    /// 修改时间
    pub mtime: u64,
    /// 数据库大小
    pub db_size: usize,
    /// 索引大小
    pub index_size: usize,
}

impl Documentation {
    /// 创建新的文档信息
    pub fn new(name: &str, slug: &str, version: &str) -> Self {
        Self {
            name: name.to_string(),
            slug: slug.to_string(),
            version: version.to_string(),
            release: String::new(),
            mtime: 0,
            db_size: 0,
            index_size: 0,
        }
    }

    /// 设置发布版本
    pub fn with_release(mut self, release: &str) -> Self {
        self.release = release.to_string();
        self
    }

    /// 设置修改时间
    pub fn with_mtime(mut self, mtime: u64) -> Self {
        self.mtime = mtime;
        self
    }

    /// 设置数据库大小
    pub fn with_db_size(mut self, db_size: usize) -> Self {
        self.db_size = db_size;
        self
    }

    /// 设置索引大小
    pub fn with_index_size(mut self, index_size: usize) -> Self {
        self.index_size = index_size;
        self
    }
}

/// 管理可用文档的注册表
pub struct DocRegistry {
    /// 已注册的文档
    docs: Vec<Documentation>,
    /// 文档抓取器
    scrapers: HashMap<String, Box<dyn ScraperTrait>>,
}

/// 全局文档注册表
static DOC_REGISTRY: Lazy<Arc<Mutex<DocRegistry>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DocRegistry::default()))
});

impl DocRegistry {
    /// 创建新的空注册表
    pub fn new() -> Self {
        Self {
            docs: Vec::new(),
            scrapers: HashMap::new(),
        }
    }

    /// 添加文档到注册表
    pub fn add(&mut self, doc: Documentation) {
        self.docs.push(doc);
    }

    /// 获取所有可用文档
    pub fn all(&self) -> &[Documentation] {
        &self.docs
    }

    /// 通过别名查找文档
    pub fn find(&self, slug: &str) -> Option<&Documentation> {
        self.docs.iter().find(|doc| doc.slug == slug)
    }

    /// 通过别名和版本查找文档
    pub fn find_with_version(&self, slug: &str, version: &str) -> Option<&Documentation> {
        self.docs
            .iter()
            .find(|doc| doc.slug == slug && doc.version == version)
    }

    /// 注册文档抓取器
    pub fn register_scraper<S: ScraperTrait + 'static>(&mut self, name: &str, scraper: S) {
        self.scrapers.insert(name.to_string(), Box::new(scraper));
    }

    /// 获取文档抓取器
    pub fn get_scraper(&self, name: &str) -> Option<&Box<dyn ScraperTrait>> {
        self.scrapers.get(name)
    }

    /// 加载所有文档从磁盘
    pub fn load_from_disk(&mut self, path: &str) -> Result<()> {
        use std::fs;
        use std::path::Path;
        use std::time::UNIX_EPOCH;

        let base_path = Path::new(path);
        if !base_path.exists() {
            return Err(Error::Message(format!("文档路径不存在: {}", path)).into());
        }

        // 清空当前文档列表
        self.docs.clear();

        // 遍历文档目录
        let entries = match fs::read_dir(base_path) {
            Ok(entries) => entries,
            Err(e) => return Err(Error::Message(format!("无法读取文档目录: {}", e)).into()),
        };

        for entry_result in entries {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(_) => continue,
            };

            let entry_path = entry.path();
            if !entry_path.is_dir() {
                continue;
            }

            // 获取文档信息
            if let Some(dirname) = entry_path.file_name().and_then(|n| n.to_str()) {
                // 解析目录名
                let (slug, version) = if dirname.contains('~') {
                    let parts: Vec<&str> = dirname.split('~').collect();
                    (parts[0].to_string(), parts[1].to_string())
                } else {
                    (dirname.to_string(), String::new())
                };

                // 尝试读取index.json和meta.json
                let index_path = entry_path.join("index.json");
                let meta_path = entry_path.join("meta.json");
                let db_path = entry_path.join("db.json");

                if !index_path.exists() || !db_path.exists() {
                    continue;
                }

                // 提取基本信息
                let index_size = fs::metadata(&index_path)
                    .map(|m| m.len() as usize)
                    .unwrap_or(0);
                let db_size = fs::metadata(&db_path)
                    .map(|m| m.len() as usize)
                    .unwrap_or(0);

                // 获取修改时间
                let mtime = match fs::metadata(&entry_path) {
                    Ok(metadata) => match metadata.modified() {
                        Ok(modified_time) => match modified_time.duration_since(UNIX_EPOCH) {
                            Ok(duration) => duration.as_secs(),
                            Err(_) => 0,
                        },
                        Err(_) => 0,
                    },
                    Err(_) => 0,
                };

                // 读取元数据
                let mut doc = Documentation::new(&slug, &slug, &version)
                    .with_mtime(mtime)
                    .with_db_size(db_size)
                    .with_index_size(index_size);

                // 尝试读取元数据文件
                if meta_path.exists() {
                    if let Ok(meta_content) = fs::read_to_string(&meta_path) {
                        if let Ok(meta_json) =
                            serde_json::from_str::<serde_json::Value>(&meta_content)
                        {
                            if let Some(release) = meta_json.get("release").and_then(|v| v.as_str())
                            {
                                doc = doc.with_release(release);
                            }
                            if let Some(name) = meta_json.get("name").and_then(|v| v.as_str()) {
                                doc.name = name.to_string();
                            }
                        }
                    }
                }

                // 添加到注册表
                self.add(doc);
            }
        }

        Ok(())
    }

    /// 生成清单JSON
    pub fn generate_manifest(&self, path: &str) -> Result<()> {
        use serde_json::{json, to_string_pretty};
        use std::fs;
        use std::path::Path;

        let manifest_path = Path::new(path).join("manifest.json");

        // 创建JSON数组
        let docs_json: Vec<serde_json::Value> = self
            .docs
            .iter()
            .map(|doc| {
                json!({
                    "name": doc.name,
                    "slug": doc.slug,
                    "version": doc.version,
                    "release": doc.release,
                    "mtime": doc.mtime,
                    "db_size": doc.db_size,
                    "index_size": doc.index_size
                })
            })
            .collect();

        // 创建整体JSON
        let manifest = json!({
            "docs": docs_json,
            "generated_at": chrono::Utc::now().timestamp()
        });

        // 写入文件
        let content = to_string_pretty(&manifest)
            .map_err(|e| Error::Message(format!("无法序列化清单JSON: {}", e)))?;

        fs::write(&manifest_path, content)
            .map_err(|e| Error::Message(format!("无法写入清单文件: {}", e)))?;

        Ok(())
    }

    /// 自动注册所有文档抓取器
    pub fn register_all_scrapers(&mut self) -> Result<()> {
        // 获取所有已注册的文档名称
        let doc_names = get_all_doc_names();
        
        // 这里应该实现实际的自动注册逻辑
        // 在实际实现中，我们需要:
        // 1. 遍历所有已注册的文档名称
        // 2. 对每个文档，获取其抓取器并注册到注册表中
        
        // 由于 Rust 不支持像 Ruby 那样的动态加载，我们需要一个不同的方法
        // 一个可能的方法是使用宏来自动生成注册代码
        
        Ok(())
    }
}

impl Default for DocRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 获取全局文档注册表
pub fn get_registry() -> Arc<Mutex<DocRegistry>> {
    DOC_REGISTRY.clone()
}

/// 添加文档到注册表
pub fn add_doc(doc: Documentation) {
    let mut registry = DOC_REGISTRY.lock().unwrap();
    registry.add(doc);
}

/// 获取所有可用文档
pub fn all_docs() -> Vec<Documentation> {
    let registry = DOC_REGISTRY.lock().unwrap();
    registry.all().to_vec()
}

/// 通过别名查找文档
pub fn find_doc(slug: &str) -> Option<Documentation> {
    let registry = DOC_REGISTRY.lock().unwrap();
    registry.find(slug).cloned()
}

/// 通过别名和版本查找文档
pub fn find_doc_with_version(slug: &str, version: &str) -> Option<Documentation> {
    let registry = DOC_REGISTRY.lock().unwrap();
    registry.find_with_version(slug, version).cloned()
}

/// 注册文档抓取器
pub fn register_scraper<S: ScraperTrait + 'static>(name: &str, scraper: S) {
    let mut registry = DOC_REGISTRY.lock().unwrap();
    registry.register_scraper(name, scraper);
}

/// 获取文档抓取器
pub fn get_scraper(name: &str) -> Option<Box<dyn ScraperTrait>> {
    let registry = DOC_REGISTRY.lock().unwrap();
    registry.get_scraper(name).map(|s| s.box_clone())
}

/// 加载所有文档从磁盘
pub fn load_docs_from_disk(path: &str) -> Result<()> {
    let mut registry = DOC_REGISTRY.lock().unwrap();
    registry.load_from_disk(path)
}

/// 生成清单JSON
pub fn generate_manifest(path: &str) -> Result<()> {
    let registry = DOC_REGISTRY.lock().unwrap();
    registry.generate_manifest(path)
}

/// 自动注册所有文档抓取器
pub fn register_all_scrapers() -> Result<()> {
    let mut registry = DOC_REGISTRY.lock().unwrap();
    registry.register_all_scrapers()
}

/// 文档抓取器特征扩展
pub trait ScraperTraitExt: ScraperTrait {
    /// 克隆抓取器
    fn box_clone(&self) -> Box<dyn ScraperTrait>;
}

impl<T: ScraperTrait + Clone + 'static> ScraperTraitExt for T {
    fn box_clone(&self) -> Box<dyn ScraperTrait> {
        Box::new(self.clone())
    }
}
