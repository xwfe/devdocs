//! Filter 基类
//!
//! 严格对齐原版 Ruby 项目中的 core/filter.rb 实现

use crate::core::url::URL;
use nipper::Document; // Assuming nipper is used here now

/// 过滤器上下文
#[derive(Debug, Clone)]
pub struct FilterContext {
    pub base_url: Option<URL>,
    pub links: Option<Vec<String>>,
    pub url: Option<URL>,
    pub root_url: Option<URL>,
    pub root_path: Option<String>,
    pub version: Option<String>,
    pub release: Option<String>,
    pub initial_paths: Vec<String>,
}

impl Default for FilterContext {
    fn default() -> Self {
        Self {
            base_url: None,
            links: None,
            url: None,
            root_url: None,
            root_path: None,
            version: None,
            release: None,
            initial_paths: Vec::new(),
        }
    }
}

/// 过滤器基类
/// 
/// 对应原版 Ruby 的 Filter 类
pub struct Filter {
    pub doc: Document,
    pub context: FilterContext,
}

impl Filter {
    /// 创建新的过滤器
    pub fn new(html: Document, context: FilterContext) -> Self {
        Self { doc: html, context }
    }

    // CSS 选择器相关方法已移除，因为生命周期复杂性
    // 过滤器应该直接在 apply 方法中使用 scraper::Html 和 Selector

    // Temporarily commented out due to scraper dependency during nipper migration
    /*
    pub fn xpath(&self, _xpath: &str) -> Vec<scraper::ElementRef> {
        unimplemented!()
    }
    */

    // Temporarily commented out due to scraper dependency during nipper migration
    /*
    pub fn at_xpath(&self, _xpath: &str) -> Option<scraper::ElementRef> {
        unimplemented!()
    }
    */

    /// 获取基础 URL
    /// 
    /// 对应原版 Ruby 的 base_url 方法
    pub fn base_url(&self) -> Option<&URL> {
        self.context.base_url.as_ref()
    }

    /// 获取链接
    /// 
    /// 对应原版 Ruby 的 links 方法
    pub fn links(&self) -> Option<&Vec<String>> {
        self.context.links.as_ref()
    }

    /// 获取当前 URL
    /// 
    /// 对应原版 Ruby 的 current_url 方法
    pub fn current_url(&self) -> Option<&URL> {
        self.context.url.as_ref()
    }

    /// 获取根 URL
    /// 
    /// 对应原版 Ruby 的 root_url 方法
    pub fn root_url(&self) -> Option<&URL> {
        self.context.root_url.as_ref()
    }

    /// 获取根路径
    /// 
    /// 对应原版 Ruby 的 root_path 方法
    pub fn root_path(&self) -> Option<&String> {
        self.context.root_path.as_ref()
    }

    /// 获取版本
    /// 
    /// 对应原版 Ruby 的 version 方法
    pub fn version(&self) -> Option<&String> {
        self.context.version.as_ref()
    }

    /// 获取发布版本
    /// 
    /// 对应原版 Ruby 的 release 方法
    pub fn release(&self) -> Option<&String> {
        self.context.release.as_ref()
    }

    /// 获取子路径
    /// 
    /// 对应原版 Ruby 的 subpath 方法
    pub fn subpath(&self) -> String {
        if let Some(current_url) = self.current_url() {
            self.subpath_to(current_url)
        } else {
            String::new()
        }
    }

    /// 获取到指定 URL 的子路径
    /// 
    /// 对应原版 Ruby 的 subpath_to 方法
    pub fn subpath_to(&self, url: &URL) -> String {
        if let Some(base_url) = self.base_url() {
            base_url.subpath_to(url, Some(true)).unwrap_or_default()
        } else {
            String::new()
        }
    }

    /// 获取 slug
    /// 
    /// 对应原版 Ruby 的 slug 方法
    pub fn slug(&self) -> String {
        let subpath = self.subpath();
        subpath
            .trim_start_matches('/')
            .trim_end_matches(".html")
            .to_string()
    }

    /// 检查是否为根页面
    /// 
    /// 对应原版 Ruby 的 root_page? 方法
    pub fn is_root_page(&self) -> bool {
        let subpath = self.subpath();
        subpath.is_empty() || subpath == "/" || Some(&subpath) == self.root_path()
    }

    /// 检查是否为初始页面
    /// 
    /// 对应原版 Ruby 的 initial_page? 方法
    pub fn is_initial_page(&self) -> bool {
        self.is_root_page() || self.context.initial_paths.contains(&self.subpath())
    }

    /// 检查是否为片段 URL 字符串
    /// 
    /// 对应原版 Ruby 的 fragment_url_string? 方法
    pub fn is_fragment_url_string(&self, s: &str) -> bool {
        s.starts_with('#')
    }

    /// 检查是否为数据 URL 字符串
    /// 
    /// 对应原版 Ruby 的 data_url_string? 方法
    pub fn is_data_url_string(&self, s: &str) -> bool {
        s.starts_with("data:")
    }

    /// 检查是否为相对 URL 字符串
    /// 
    /// 对应原版 Ruby 的 relative_url_string? 方法
    pub fn is_relative_url_string(&self, s: &str) -> bool {
        !self.is_absolute_url_string(s) 
            && !self.is_fragment_url_string(s) 
            && !self.is_data_url_string(s)
    }

    /// 检查是否为绝对 URL 字符串
    /// 
    /// 对应原版 Ruby 的 absolute_url_string? 方法
    pub fn is_absolute_url_string(&self, s: &str) -> bool {
        // 简化的方案式检查
        s.contains("://") || s.starts_with("//")
    }

    /// 解析 HTML
    /// 
    /// 对应原版 Ruby 的 parse_html 方法
    pub fn parse_html(&self, html: &str) -> Document {
        if std::env::var("RACK_ENV") != Ok("test".to_string()) {
            eprintln!("WARNING: {} is re-parsing the document", std::any::type_name::<Self>());
        }
        Document::from(html)
    }

    /// 清理路径
    /// 
    /// 对应原版 Ruby 的 clean_path 方法
    pub fn clean_path(&self, path: &str) -> String {
        path.chars()
            .map(|c| match c {
                '!' | ';' | ':' => '-',
                '+' => '_',
                _ => c,
            })
            .collect::<String>()
            .replace("+", "_plus_")
    }
}