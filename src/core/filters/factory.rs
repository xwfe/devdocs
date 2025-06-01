//! 过滤器工厂模块
//!
//! 提供创建和管理过滤器的工厂

use crate::core::filter_base::{Filter, FilterStack};
use crate::core::filters::core::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// 过滤器工厂
///
/// 用于创建和管理过滤器
#[derive(Default)]
pub struct FilterFactory {
    /// 已注册的过滤器
    filters: HashMap<String, Box<dyn Filter>>,
}

/// 全局过滤器工厂
static FILTER_FACTORY: Lazy<Arc<Mutex<FilterFactory>>> = Lazy::new(|| {
    let mut factory = FilterFactory::default();
    
    // 注册核心过滤器
    factory.register("apply_base_url", Box::new(ApplyBaseUrlFilter));
    factory.register("clean_html", Box::new(CleanHtmlFilter));
    factory.register("container", Box::new(ContainerFilter::default()));
    factory.register("internal_urls", Box::new(InternalUrlsFilter));
    factory.register("normalize_paths", Box::new(NormalizePathsFilter));
    factory.register("normalize_urls", Box::new(NormalizeUrlsFilter));
    factory.register("parse_cf_email", Box::new(ParseCfEmailFilter));
    
    Arc::new(Mutex::new(factory))
});

impl FilterFactory {
    /// 创建新的过滤器工厂
    pub fn new() -> Self {
        Self {
            filters: HashMap::new(),
        }
    }
    
    /// 注册过滤器
    pub fn register(&mut self, name: &str, filter: Box<dyn Filter>) {
        self.filters.insert(name.to_string(), filter);
    }
    
    /// 获取过滤器
    pub fn get(&self, name: &str) -> Option<Box<dyn Filter>> {
        self.filters.get(name).map(|f| f.box_clone())
    }
    
    /// 创建过滤器栈
    pub fn create_stack(&self, filter_names: &[&str]) -> FilterStack {
        let mut stack = FilterStack::new();
        
        for name in filter_names {
            if let Some(filter) = self.get(name) {
                stack.push(filter);
            }
        }
        
        stack
    }
    
    /// 创建默认的 HTML 过滤器栈
    pub fn create_default_html_stack(&self) -> FilterStack {
        self.create_stack(&[
            "apply_base_url",
            "container",
            "clean_html",
            "normalize_urls",
            "internal_urls",
            "normalize_paths",
            "parse_cf_email",
        ])
    }
    
    /// 创建默认的文本过滤器栈
    pub fn create_default_text_stack(&self) -> FilterStack {
        let mut stack = FilterStack::new();
        // 文本过滤器通常用于处理 HTML 到文本的转换
        // 这里可以添加默认的文本过滤器
        stack
    }
}

/// 获取全局过滤器工厂
pub fn get_filter_factory() -> Arc<Mutex<FilterFactory>> {
    FILTER_FACTORY.clone()
}

/// 注册过滤器
pub fn register_filter(name: &str, filter: Box<dyn Filter>) {
    let mut factory = FILTER_FACTORY.lock().unwrap();
    factory.register(name, filter);
}

/// 获取过滤器
pub fn get_filter(name: &str) -> Option<Box<dyn Filter>> {
    let factory = FILTER_FACTORY.lock().unwrap();
    factory.get(name)
}

/// 创建过滤器栈
pub fn create_filter_stack(filter_names: &[&str]) -> FilterStack {
    let factory = FILTER_FACTORY.lock().unwrap();
    factory.create_stack(filter_names)
}

/// 创建默认的 HTML 过滤器栈
pub fn create_default_html_stack() -> FilterStack {
    let factory = FILTER_FACTORY.lock().unwrap();
    factory.create_default_html_stack()
}

/// 创建默认的文本过滤器栈
pub fn create_default_text_stack() -> FilterStack {
    let factory = FILTER_FACTORY.lock().unwrap();
    factory.create_default_text_stack()
}
