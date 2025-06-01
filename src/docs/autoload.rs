//! 文档自动加载模块
//!
//! 提供自动发现和加载文档的功能，遵循原版 Ruby 项目的设计模式

use crate::core::error::{Error, Result};
use crate::core::filter_base::{Filter, FilterStack};
use crate::core::scraper::{Scraper, ScraperTrait, UrlScraper};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// 文档注册表
#[derive(Default)]
pub struct DocRegistry {
    /// 已注册的文档抓取器
    scrapers: HashMap<String, Box<dyn ScraperTrait>>,
}

/// 全局文档注册表
static DOC_REGISTRY: Lazy<Arc<Mutex<DocRegistry>>> = Lazy::new(|| {
    Arc::new(Mutex::new(DocRegistry::default()))
});

impl DocRegistry {
    /// 创建新的文档注册表
    pub fn new() -> Self {
        Self {
            scrapers: HashMap::new(),
        }
    }

    /// 注册文档抓取器
    pub fn register<S: ScraperTrait + 'static>(&mut self, name: &str, scraper: S) {
        self.scrapers.insert(name.to_string(), Box::new(scraper));
    }

    /// 获取文档抓取器
    pub fn get(&self, name: &str) -> Option<&Box<dyn ScraperTrait>> {
        self.scrapers.get(name)
    }

    /// 获取所有已注册的文档名称
    pub fn get_all_names(&self) -> Vec<String> {
        self.scrapers.keys().cloned().collect()
    }

    /// 自动加载文档
    pub fn autoload(&mut self, docs_dir: &Path) -> Result<()> {
        // 遍历文档目录
        if !docs_dir.exists() || !docs_dir.is_dir() {
            return Err(Error::Message(format!("文档目录不存在: {:?}", docs_dir)).into());
        }

        // 这里应该实现实际的自动加载逻辑
        // 在实际实现中，我们需要:
        // 1. 遍历 docs_dir 下的所有子目录
        // 2. 对每个子目录，尝试加载其中的过滤器和抓取器
        // 3. 将加载的抓取器注册到注册表中

        // 由于 Rust 不支持像 Ruby 那样的动态加载，我们需要一个不同的方法
        // 一个可能的方法是使用宏来自动生成注册代码

        Ok(())
    }
}

/// 自动加载辅助特征
///
/// 提供自动加载文档的辅助方法
pub trait AutoloadHelper {
    /// 自动加载所有过滤器
    fn autoload_filters(&mut self, filter_path: &str);

    /// 自动加载特定过滤器
    fn autoload_filter(&mut self, filter_name: &str, filter_path: &str);
}

impl AutoloadHelper for Scraper {
    fn autoload_filters(&mut self, filter_path: &str) {
        // 在实际实现中，我们需要:
        // 1. 查找 filter_path 下的所有过滤器
        // 2. 对每个过滤器，调用 autoload_filter

        // 由于 Rust 不支持像 Ruby 那样的动态加载，我们需要一个不同的方法
        // 这里只是一个占位符
    }

    fn autoload_filter(&mut self, filter_name: &str, filter_path: &str) {
        // 在实际实现中，我们需要:
        // 1. 加载 filter_path 中的过滤器
        // 2. 将过滤器添加到适当的过滤器栈中

        // 由于 Rust 不支持像 Ruby 那样的动态加载，我们需要一个不同的方法
        // 这里只是一个占位符
    }
}

/// 文档构建器
///
/// 用于构建文档抓取器的辅助结构
pub struct DocBuilder {
    /// 基础抓取器
    pub scraper: Scraper,
}

impl DocBuilder {
    /// 创建新的文档构建器
    pub fn new(name: &str, doc_type: &str) -> Self {
        Self {
            scraper: Scraper::new(name, doc_type),
        }
    }

    /// 设置版本
    pub fn with_version(mut self, version: &str) -> Self {
        self.scraper.set_version(version);
        self
    }

    /// 设置发布版本
    pub fn with_release(mut self, release: &str) -> Self {
        self.scraper.set_release(release);
        self
    }

    /// 设置基础 URL
    pub fn with_base_url(mut self, url: &str) -> Result<Self> {
        self.scraper.set_base_url(url)?;
        Ok(self)
    }

    /// 设置根路径
    pub fn with_root_path(mut self, path: &str) -> Self {
        self.scraper.set_root_path(path);
        self
    }

    /// 添加初始路径
    pub fn with_initial_path(mut self, path: &str) -> Self {
        self.scraper.add_initial_path(path);
        self
    }

    /// 添加链接
    pub fn with_link(mut self, key: &str, url: &str) -> Self {
        self.scraper.add_link(key, url);
        self
    }

    /// 设置选项
    pub fn with_option(mut self, key: &str, value: &str) -> Self {
        self.scraper.set_option(key, value);
        self
    }

    /// 添加 HTML 过滤器
    pub fn with_html_filter<F: Filter + 'static>(mut self, filter: F) -> Self {
        self.scraper.add_html_filter(filter);
        self
    }

    /// 添加文本过滤器
    pub fn with_text_filter<F: Filter + 'static>(mut self, filter: F) -> Self {
        self.scraper.add_text_filter(filter);
        self
    }

    /// 构建 URL 抓取器
    pub fn build_url_scraper(self) -> UrlScraper {
        UrlScraper {
            scraper: self.scraper,
        }
    }
}

/// 注册文档抓取器
pub fn register_doc<S: ScraperTrait + 'static>(name: &str, scraper: S) {
    let mut registry = DOC_REGISTRY.lock().unwrap();
    registry.register(name, scraper);
}

/// 获取文档抓取器
pub fn get_doc(name: &str) -> Option<Box<dyn ScraperTrait>> {
    let registry = DOC_REGISTRY.lock().unwrap();
    registry.get(name).map(|s| s.box_clone())
}

/// 获取所有已注册的文档名称
pub fn get_all_doc_names() -> Vec<String> {
    let registry = DOC_REGISTRY.lock().unwrap();
    registry.get_all_names()
}

/// 自动加载所有文档
pub fn autoload_all_docs(docs_dir: &Path) -> Result<()> {
    let mut registry = DOC_REGISTRY.lock().unwrap();
    registry.autoload(docs_dir)
}

/// ScraperTrait 的扩展方法
pub trait ScraperTraitExt: ScraperTrait {
    /// 克隆抓取器
    fn box_clone(&self) -> Box<dyn ScraperTrait>;
}

impl<T: ScraperTrait + Clone + 'static> ScraperTraitExt for T {
    fn box_clone(&self) -> Box<dyn ScraperTrait> {
        Box::new(self.clone())
    }
}
