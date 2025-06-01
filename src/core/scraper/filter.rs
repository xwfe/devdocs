//! Scraper Filter 模块
//!
//! 严格对齐原版 Ruby 项目中的过滤器接口定义

use crate::core::error::Result;
use crate::core::models::Entry;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;
use nipper::Document;

/// 过滤器上下文
#[derive(Debug, Clone)]
pub struct FilterContext {
    pub entries: Vec<Entry>,
    pub current_path: String,
    pub current_url: String,
    pub base_url: String,
    pub root_url: String,
    pub root_path: Option<String>,
    pub links: Vec<(String, String)>,
    pub version: Option<String>,
    pub release: Option<String>,
    pub initial_paths: Vec<String>,
    pub options: HashMap<String, String>,
}

impl FilterContext {
    /// 创建新的过滤器上下文
    pub fn new(_content: &str, path: &str) -> Self {
        Self {
            entries: Vec::new(),
            current_path: path.to_string(),
            current_url: String::new(),
            base_url: String::new(),
            root_url: String::new(),
            root_path: None,
            links: Vec::new(),
            version: None,
            release: None,
            initial_paths: Vec::new(),
            options: HashMap::new(),
        }
    }
}

impl Default for FilterContext {
    fn default() -> Self {
        Self {
            entries: Vec::new(),
            current_path: String::new(),
            current_url: String::new(),
            base_url: String::new(),
            root_url: String::new(),
            root_path: None,
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

pub trait EntryDefinitionProvider: Send + Sync {
    fn get_entry_name(&self, document: &Document, context: &FilterContext) -> Option<String>;
    fn get_entry_type(&self, name: &str, context: &FilterContext) -> Option<String>;
    // fn additional_entries(&self, document: &Document, context: &FilterContext) -> Vec<EntryDefinition>; // Placeholder

    fn box_clone(&self) -> Box<dyn EntryDefinitionProvider>;
}

impl Clone for Box<dyn EntryDefinitionProvider> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct GenericEntriesFilter {
    provider: Box<dyn EntryDefinitionProvider>,
}

impl GenericEntriesFilter {
    pub fn new(provider: Box<dyn EntryDefinitionProvider>) -> Self {
        Self { provider }
    }
}

impl Filter for GenericEntriesFilter {
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        let document = Document::from(html);

        let current_path_is_empty = context.current_path.is_empty();
        let is_root_path_current = match &context.root_path {
            Some(rp) => rp == &context.current_path,
            None => false,
        };
        
        let root_page = current_path_is_empty || context.current_path == "/" || is_root_path_current;

        let entry_name_opt: Option<String>;
        let entry_type_opt: Option<String>;

        if root_page {
            entry_name_opt = None;
            entry_type_opt = None;
        } else {
            entry_name_opt = self.provider.get_entry_name(&document, context);
            if let Some(ref name_val) = entry_name_opt {
                entry_type_opt = self.provider.get_entry_type(name_val, context);
            } else {
                entry_type_opt = None;
            }
        }

        if let Some(name) = entry_name_opt {
            let entry = Entry::new(
                Some(name),
                Some(context.current_path.clone()),
                entry_type_opt,
            ).map_err(|e| crate::core::error::Error::from(e))?;
            context.entries.push(entry);
        }
        
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}