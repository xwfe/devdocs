//! 过滤器基类模块
//!
//! 提供所有过滤器的基础实现，遵循原版 Ruby 项目的设计模式

use crate::core::error::Result;
use crate::core::URL;
use std::collections::HashMap;
use std::any::Any;
use nipper::Document;

/// 过滤器上下文
#[derive(Debug, Clone)]
pub struct FilterContext {
    /// 当前路径
    pub current_path: String,
    /// 当前 URL
    pub current_url: String,
    /// 基础 URL
    pub base_url: String,
    /// 根 URL
    pub root_url: String,
    /// 根路径
    pub root_path: String,
    /// 链接列表
    pub links: Vec<(String, String)>,
    /// 版本
    pub version: Option<String>,
    /// 发布版本
    pub release: Option<String>,
    /// 初始路径列表
    pub initial_paths: Vec<String>,
    /// 选项
    pub options: HashMap<String, String>,
}

impl FilterContext {
    /// 创建新的过滤器上下文
    pub fn new() -> Self {
        Self::default()
    }

    /// 获取子路径
    pub fn subpath(&self) -> &str {
        &self.current_path
    }

    /// 是否为根页面
    pub fn is_root_page(&self) -> bool {
        self.current_path.is_empty() || self.current_path == "/" || self.current_path == self.root_path
    }

    /// 是否为初始页面
    pub fn is_initial_page(&self) -> bool {
        self.is_root_page() || self.initial_paths.contains(&self.current_path)
    }

    /// 获取 slug
    pub fn slug(&self) -> String {
        self.current_path
            .trim_start_matches('/')
            .trim_end_matches(".html")
            .to_string()
    }
}

impl Default for FilterContext {
    fn default() -> Self {
        Self {
            current_path: String::new(),
            current_url: String::new(),
            base_url: String::new(),
            root_url: String::new(),
            root_path: String::new(),
            links: Vec::new(),
            version: None,
            release: None,
            initial_paths: Vec::new(),
            options: HashMap::new(),
        }
    }
}

/// 过滤器特征
///
/// 对应原版 Ruby 的 Filter 接口
pub trait Filter: Send + Sync {
    /// 应用过滤器
    ///
    /// 对应原版 Ruby 的 call 方法
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String>;
    
    /// 克隆过滤器
    fn box_clone(&self) -> Box<dyn Filter>;
    
    /// 获取过滤器名称
    fn name(&self) -> &'static str {
        std::any::type_name::<Self>()
    }
    
    /// 将过滤器转换为 Any 类型
    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
    
    /// 解析 HTML
    fn parse_html(&self, html: &str) -> Document {
        Document::from(html)
    }
}

/// 过滤器栈
///
/// 用于管理多个过滤器
pub struct FilterStack {
    /// 过滤器列表
    pub filters: Vec<Box<dyn Filter>>,
}

impl FilterStack {
    /// 创建新的过滤器栈
    pub fn new() -> Self {
        Self { filters: Vec::new() }
    }
    
    /// 添加过滤器
    pub fn add_filter(&mut self, filter: Box<dyn Filter>) {
        self.filters.push(filter);
    }
    
    /// 应用所有过滤器
    pub fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        let mut result = html.to_string();
        
        for filter in &self.filters {
            result = filter.apply(&result, context)?;
        }
        
        Ok(result)
    }
    
    /// 获取过滤器数量
    pub fn len(&self) -> usize {
        self.filters.len()
    }
    
    /// 检查过滤器栈是否为空
    pub fn is_empty(&self) -> bool {
        self.filters.is_empty()
    }

    /// 创建过滤器栈的可继承副本
    pub fn inheritable_copy(&self) -> Self {
        let mut new_stack = Self::new();
        for filter in &self.filters {
            new_stack.filters.push(filter.box_clone());
        }
        new_stack
    }
}

impl Default for FilterStack {
    fn default() -> Self {
        Self::new()
    }
}
