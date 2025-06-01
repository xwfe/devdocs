//! 过滤器特征模块
//!
//! 严格对齐原版 Ruby 项目中的过滤器接口定义

use crate::core::error::Result;
use std::any::Any;
use std::collections::HashMap;

/// 过滤器上下文
#[derive(Debug, Clone)]
pub struct FilterContext {
    pub current_path: String,
    pub current_url: String,
    pub base_url: String,
    pub root_url: String,
    pub root_path: String,
    pub links: Vec<(String, String)>,
    pub version: Option<String>,
    pub release: Option<String>,
    pub initial_paths: Vec<String>,
    pub options: HashMap<String, String>,
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
    /// 对应原版 Ruby 的 apply 方法
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String>;

    /// 克隆过滤器
    fn box_clone(&self) -> Box<dyn Filter>;

    /// 获取条目
    /// 
    /// 对应原版 Ruby 的 get_entries 方法
    fn get_entries(&self, _html: &str, _context: &FilterContext) -> Vec<(String, String, String)> {
        Vec::new()
    }

    /// 转换为 Any 类型
    fn as_any(&self) -> &dyn Any;

    /// 转换为可变 Any 类型
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// 过滤器栈
#[derive(Debug, Clone)]
pub struct FilterStack {
    filters: Vec<Box<dyn Filter>>,
}

impl FilterStack {
    /// 创建新的过滤器栈
    pub fn new() -> Self {
        Self {
            filters: Vec::new(),
        }
    }

    /// 添加过滤器
    pub fn push(&mut self, filter: Box<dyn Filter>) {
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

    /// 获取所有条目
    pub fn get_all_entries(&self, html: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let mut all_entries = Vec::new();
        for filter in &self.filters {
            let entries = filter.get_entries(html, context);
            all_entries.extend(entries);
        }
        all_entries
    }
}

impl Default for FilterStack {
    fn default() -> Self {
        Self::new()
    }
}