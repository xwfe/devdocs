//! 自动加载辅助模块
//!
//! 提供自动加载文档过滤器的功能

use crate::core::error::{Error, Result};
use crate::core::filter_base::Filter;
use crate::core::filters::factory::register_filter;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// 过滤器加载器
///
/// 用于加载和管理文档过滤器
#[derive(Default)]
pub struct FilterLoader {
    /// 已加载的过滤器
    loaded_filters: HashMap<String, Box<dyn Filter>>,
    /// 过滤器路径
    filter_paths: HashMap<String, PathBuf>,
}

/// 全局过滤器加载器
static FILTER_LOADER: Lazy<Arc<Mutex<FilterLoader>>> = Lazy::new(|| {
    Arc::new(Mutex::new(FilterLoader::default()))
});

impl FilterLoader {
    /// 创建新的过滤器加载器
    pub fn new() -> Self {
        Self {
            loaded_filters: HashMap::new(),
            filter_paths: HashMap::new(),
        }
    }
    
    /// 注册过滤器路径
    pub fn register_path(&mut self, name: &str, path: &Path) {
        self.filter_paths.insert(name.to_string(), path.to_path_buf());
    }
    
    /// 加载过滤器
    pub fn load_filter(&mut self, name: &str) -> Result<Box<dyn Filter>> {
        // 如果过滤器已经加载，直接返回
        if let Some(filter) = self.loaded_filters.get(name) {
            return Ok(filter.box_clone());
        }
        
        // 检查过滤器路径是否存在
        if let Some(path) = self.filter_paths.get(name) {
            // 在实际实现中，我们需要根据路径加载过滤器
            // 由于 Rust 不支持像 Ruby 那样的动态加载，这里只是一个占位符
            
            // 返回错误，表示无法加载过滤器
            return Err(Error::Message(format!("无法加载过滤器: {}", name)).into());
        }
        
        // 如果过滤器不存在，返回错误
        Err(Error::Message(format!("未找到过滤器: {}", name)).into())
    }
    
    /// 注册过滤器
    pub fn register_filter(&mut self, name: &str, filter: Box<dyn Filter>) {
        self.loaded_filters.insert(name.to_string(), filter);
    }
    
    /// 获取过滤器
    pub fn get_filter(&self, name: &str) -> Option<Box<dyn Filter>> {
        self.loaded_filters.get(name).map(|f| f.box_clone())
    }
    
    /// 自动加载所有过滤器
    pub fn autoload_all(&mut self) -> Result<()> {
        // 在实际实现中，我们需要扫描所有过滤器路径，加载所有过滤器
        // 由于 Rust 不支持像 Ruby 那样的动态加载，这里只是一个占位符
        
        Ok(())
    }
}

/// 自动加载辅助特征
///
/// 提供自动加载文档过滤器的辅助方法
pub trait AutoloadHelper {
    /// 自动加载所有过滤器
    fn autoload_filters(&mut self, filter_path: &str);
    
    /// 自动加载特定过滤器
    fn autoload_filter(&mut self, filter_name: &str, filter_path: &str);
}

/// 获取全局过滤器加载器
pub fn get_filter_loader() -> Arc<Mutex<FilterLoader>> {
    FILTER_LOADER.clone()
}

/// 注册过滤器路径
pub fn register_filter_path(name: &str, path: &Path) {
    let mut loader = FILTER_LOADER.lock().unwrap();
    loader.register_path(name, path);
}

/// 加载过滤器
pub fn load_filter(name: &str) -> Result<Box<dyn Filter>> {
    let mut loader = FILTER_LOADER.lock().unwrap();
    loader.load_filter(name)
}

/// 注册过滤器
pub fn register_filter_to_loader(name: &str, filter: Box<dyn Filter>) {
    let mut loader = FILTER_LOADER.lock().unwrap();
    loader.register_filter(name, filter);
    
    // 同时注册到全局过滤器工厂
    register_filter(name, filter.box_clone());
}

/// 获取过滤器
pub fn get_filter_from_loader(name: &str) -> Option<Box<dyn Filter>> {
    let loader = FILTER_LOADER.lock().unwrap();
    loader.get_filter(name)
}

/// 自动加载所有过滤器
pub fn autoload_all_filters() -> Result<()> {
    let mut loader = FILTER_LOADER.lock().unwrap();
    loader.autoload_all()
}
