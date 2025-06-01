//! Scraper 模块
//!
//! 严格对齐原版 Ruby 项目中的 scraper 相关功能

pub mod base;
pub mod filter;

pub use base::{Scraper as BaseScraper, UrlScraper};
pub use filter::{Filter, FilterContext};

use crate::core::URL;
use std::collections::HashMap;

/// 爬虫基类
///
/// 对应原版 Ruby 的 Scraper 类
pub struct Scraper {
    pub base_url: Option<URL>,
    pub root_path: Option<String>,
    pub initial_paths: Vec<String>,
    pub options: HashMap<String, String>,
    pub stubs: HashMap<String, String>,
}

impl Scraper {
    /// 创建新的爬虫
    pub fn new() -> Self {
        Self {
            base_url: None,
            root_path: None,
            initial_paths: Vec::new(),
            options: HashMap::new(),
            stubs: HashMap::new(),
        }
    }

    /// 设置基础 URL
    pub fn set_base_url(&mut self, url: &str) {
        self.base_url = Some(URL::parse(url).unwrap());
    }

    /// 设置根路径
    pub fn set_root_path(&mut self, path: String) {
        self.root_path = Some(path);
    }

    /// 添加初始路径
    pub fn add_initial_path(&mut self, path: String) {
        self.initial_paths.push(path);
    }

    /// 设置选项
    pub fn set_option(&mut self, key: String, value: String) {
        self.options.insert(key, value);
    }

    /// 获取选项
    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }

    /// 添加存根
    pub fn stub(&mut self, path: String, content: String) {
        self.stubs.insert(path, content);
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> Option<&URL> {
        self.base_url.as_ref()
    }

    /// 获取根 URL
    pub fn root_url(&self) -> Option<URL> {
        if let Some(base_url) = &self.base_url {
            if let Some(root_path) = &self.root_path {
                if !root_path.is_empty() && root_path != "/" {
                    Some(base_url.join(root_path))
                } else {
                    Some(base_url.clone())
                }
            } else {
                Some(base_url.clone())
            }
        } else {
            None
        }
    }

    /// 获取初始 URLs
    pub fn initial_urls(&self) -> Vec<String> {
        let mut urls = Vec::new();

        if let Some(root_url) = self.root_url() {
            urls.push(root_url.to_string());
        }

        for path in &self.initial_paths {
            if let Some(url) = self.url_for(path) {
                urls.push(url);
            }
        }

        urls
    }

    /// 构建 URL
    pub fn url_for(&self, path: &str) -> Option<String> {
        if let Some(base_url) = &self.base_url {
            if path.is_empty() || path == "/" {
                self.root_url().map(|u| u.to_string())
            } else {
                Some(base_url.join(path).to_string())
            }
        } else {
            None
        }
    }
}

impl Default for Scraper {
    fn default() -> Self {
        Self::new()
    }
}

/// 爬虫特征，对应原始 Ruby 项目中的 Docs::Scraper 类
pub trait ScraperTrait: Send + Sync {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn name(&self) -> &str;
    fn version(&self) -> Option<&str>;
    fn base_url(&self) -> &str;
    fn output_path(&self) -> &str;
    fn doc_type(&self) -> Option<&str>;
    fn release(&self) -> Option<&str>;
    fn get_latest_version(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn clone_box(&self) -> Box<dyn ScraperTrait>;
    
    /// 克隆抓取器
    fn box_clone(&self) -> Box<dyn ScraperTrait> {
        self.clone_box()
    }
}
