//! 抓取器基类模块
//!
//! 提供所有抓取器的基础实现，遵循原版 Ruby 项目的设计模式

use crate::core::error::{Error, Result};
use crate::core::filter_base::{Filter, FilterContext, FilterStack};
use crate::core::URL;
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// 抓取器基类
///
/// 对应原版 Ruby 的 Scraper 类
#[derive(Debug, Clone)]
pub struct Scraper {
    /// 文档名称
    pub name: String,
    /// 文档类型
    pub doc_type: String,
    /// 文档版本
    pub version: String,
    /// 文档发布版本
    pub release: String,
    /// 基础 URL
    pub base_url: Option<URL>,
    /// 根路径
    pub root_path: String,
    /// 初始路径列表
    pub initial_paths: Vec<String>,
    /// 链接
    pub links: HashMap<String, String>,
    /// HTML 过滤器栈
    pub html_filters: FilterStack,
    /// 文本过滤器栈
    pub text_filters: FilterStack,
    /// 选项
    pub options: HashMap<String, String>,
    /// 存根
    pub stubs: HashMap<String, String>,
}

impl Scraper {
    /// 创建新的抓取器
    pub fn new(name: &str, doc_type: &str) -> Self {
        Self {
            name: name.to_string(),
            doc_type: doc_type.to_string(),
            version: String::new(),
            release: String::new(),
            base_url: None,
            root_path: String::new(),
            initial_paths: Vec::new(),
            links: HashMap::new(),
            html_filters: FilterStack::new(),
            text_filters: FilterStack::new(),
            options: HashMap::new(),
            stubs: HashMap::new(),
        }
    }

    /// 设置版本
    pub fn set_version(&mut self, version: &str) {
        self.version = version.to_string();
    }

    /// 设置发布版本
    pub fn set_release(&mut self, release: &str) {
        self.release = release.to_string();
    }

    /// 设置基础 URL
    pub fn set_base_url(&mut self, url: &str) -> Result<()> {
        self.base_url = Some(URL::parse(url).map_err(|e| Error::Message(format!("无效的 URL: {}", e)))?);
        Ok(())
    }

    /// 设置根路径
    pub fn set_root_path(&mut self, path: &str) {
        self.root_path = path.to_string();
    }

    /// 添加初始路径
    pub fn add_initial_path(&mut self, path: &str) {
        self.initial_paths.push(path.to_string());
    }

    /// 添加链接
    pub fn add_link(&mut self, key: &str, url: &str) {
        self.links.insert(key.to_string(), url.to_string());
    }

    /// 设置选项
    pub fn set_option(&mut self, key: &str, value: &str) {
        self.options.insert(key.to_string(), value.to_string());
    }

    /// 获取选项
    pub fn get_option(&self, key: &str) -> Option<&String> {
        self.options.get(key)
    }

    /// 添加存根
    pub fn add_stub(&mut self, path: &str, content: &str) {
        self.stubs.insert(path.to_string(), content.to_string());
    }

    /// 添加 HTML 过滤器
    pub fn add_html_filter<F: Filter + 'static>(&mut self, filter: F) {
        self.html_filters.push(filter);
    }

    /// 添加文本过滤器
    pub fn add_text_filter<F: Filter + 'static>(&mut self, filter: F) {
        self.text_filters.push(filter);
    }

    /// 获取基础 URL
    pub fn base_url(&self) -> Option<&URL> {
        self.base_url.as_ref()
    }

    /// 获取根 URL
    pub fn root_url(&self) -> Option<URL> {
        if let Some(base_url) = &self.base_url {
            if !self.root_path.is_empty() && self.root_path != "/" {
                Some(base_url.join(&self.root_path))
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

    /// 创建过滤器上下文
    pub fn create_context(&self, url: &str) -> FilterContext {
        let mut context = FilterContext::new();
        
        if let Some(base_url) = &self.base_url {
            context.base_url = base_url.to_string();
        }
        
        if let Some(root_url) = self.root_url() {
            context.root_url = root_url.to_string();
        }
        
        context.current_url = url.to_string();
        context.root_path = self.root_path.clone();
        context.initial_paths = self.initial_paths.clone();
        
        if let Some(version) = self.version.is_empty().then(|| None).or(Some(self.version.clone())) {
            context.version = Some(version);
        }
        
        if let Some(release) = self.release.is_empty().then(|| None).or(Some(self.release.clone())) {
            context.release = Some(release);
        }
        
        // 将链接转换为元组列表
        context.links = self.links.iter()
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();
        
        // 从 URL 中提取路径
        if let Some(base_url) = &self.base_url {
            if let Ok(url_obj) = URL::parse(url) {
                if let Some(path) = url_obj.path().strip_prefix(base_url.path()) {
                    context.current_path = path.to_string();
                }
            }
        }
        
        context
    }

    /// 处理 HTML 内容
    pub fn process_html(&self, html: &str, url: &str) -> Result<(String, Vec<(String, String, String)>)> {
        let mut context = self.create_context(url);
        
        // 应用 HTML 过滤器
        let processed_html = self.html_filters.call(html, &mut context)?;
        
        // 应用文本过滤器
        let processed_text = self.text_filters.call(&processed_html, &mut context)?;
        
        // 获取条目
        let entries = self.html_filters.get_all_entries(&processed_text, &context);
        
        Ok((processed_text, entries))
    }

    /// 构建页面
    pub fn build_page(&self, path: &str) -> Result<(String, Vec<(String, String, String)>)> {
        let url = self.url_for(path).ok_or_else(|| Error::Message(format!("无法构建 URL: {}", path)))?;
        
        // 检查是否有存根
        if let Some(content) = self.stubs.get(path) {
            return self.process_html(content, &url);
        }
        
        // 这里应该实现实际的 HTTP 请求逻辑
        // 为了简单起见，这里只返回一个错误
        Err(Error::Message(format!("未实现的 HTTP 请求: {}", url)).into())
    }

    /// 构建所有页面
    pub fn build_pages<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(String, Vec<(String, String, String)>) -> Result<()>,
    {
        let mut visited = HashSet::new();
        let mut queue: Vec<String> = self.initial_urls();
        
        for url in &queue {
            visited.insert(url.to_lowercase());
        }
        
        while let Some(url) = queue.pop() {
            // 这里应该实现实际的 HTTP 请求逻辑
            // 为了简单起见，这里只返回一个错误
            return Err(Error::Message(format!("未实现的 HTTP 请求: {}", url)).into());
            
            // 实际实现应该类似于:
            // let (content, entries) = self.process_html(response_body, &url)?;
            // callback(content, entries)?;
            // 
            // 然后处理内部链接:
            // for internal_url in extract_internal_urls(content) {
            //     if visited.insert(internal_url.to_lowercase()) {
            //         queue.push(internal_url);
            //     }
            // }
        }
        
        Ok(())
    }
}

/// URL 抓取器
///
/// 对应原版 Ruby 的 UrlScraper 类
pub struct UrlScraper {
    /// 基础抓取器
    pub scraper: Scraper,
}

impl UrlScraper {
    /// 创建新的 URL 抓取器
    pub fn new(name: &str, doc_type: &str) -> Self {
        Self {
            scraper: Scraper::new(name, doc_type),
        }
    }
}

/// 文件抓取器
///
/// 对应原版 Ruby 的 FileScraper 类
pub struct FileScraper {
    /// 基础抓取器
    pub scraper: Scraper,
    /// 输入路径
    pub input_path: String,
}

impl FileScraper {
    /// 创建新的文件抓取器
    pub fn new(name: &str, doc_type: &str, input_path: &str) -> Self {
        Self {
            scraper: Scraper::new(name, doc_type),
            input_path: input_path.to_string(),
        }
    }
}

/// 抓取器特征
///
/// 所有抓取器都应该实现这个特征
pub trait ScraperTrait {
    /// 运行抓取器
    fn run(&self) -> Result<()>;
    
    /// 获取抓取器名称
    fn name(&self) -> &str;
    
    /// 获取版本
    fn version(&self) -> &str;
    
    /// 获取基础抓取器
    fn scraper(&self) -> &Scraper;
    
    /// 获取可变基础抓取器
    fn scraper_mut(&mut self) -> &mut Scraper;
}
