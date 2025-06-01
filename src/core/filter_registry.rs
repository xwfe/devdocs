//! Filter Registry 模块 - 严格对齐原版 Ruby 过滤器注册机制
//!
//! 提供过滤器的注册、查找和管理功能

use crate::core::scraper::filter::{Filter, FilterChain};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// 过滤器工厂函数类型
pub type FilterFactory = Box<dyn Fn() -> Box<dyn Filter> + Send + Sync>;

/// 过滤器注册表
/// 
/// 对应原版 Ruby 的过滤器管理机制，提供全局的过滤器注册和查找功能
#[derive(Clone)]
pub struct FilterRegistry {
    /// 过滤器工厂映射
    factories: Arc<Mutex<HashMap<String, FilterFactory>>>,
    /// 过滤器实例缓存
    instances: Arc<Mutex<HashMap<String, Box<dyn Filter>>>>,
    /// 过滤器链模板
    chains: Arc<Mutex<HashMap<String, Vec<String>>>>,
}

impl FilterRegistry {
    /// 创建新的过滤器注册表
    pub fn new() -> Self {
        Self {
            factories: Arc::new(Mutex::new(HashMap::new())),
            instances: Arc::new(Mutex::new(HashMap::new())),
            chains: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// 注册过滤器工厂
    /// 
    /// 对应原版 Ruby 的过滤器注册机制
    pub fn register<F, T>(&self, name: &str, factory: F) -> Result<(), String>
    where
        F: Fn() -> T + 'static + Send + Sync,
        T: Filter + 'static,
    {
        let boxed_factory: FilterFactory = Box::new(move || -> Box<dyn Filter> {
            Box::new(factory())
        });

        let mut factories = self.factories.lock().map_err(|e| e.to_string())?;
        factories.insert(name.to_string(), boxed_factory);
        Ok(())
    }

    /// 注册过滤器实例
    pub fn register_instance(&self, name: &str, filter: Box<dyn Filter>) -> Result<(), String> {
        let mut instances = self.instances.lock().map_err(|e| e.to_string())?;
        instances.insert(name.to_string(), filter);
        Ok(())
    }

    /// 创建过滤器实例
    /// 
    /// 对应原版 Ruby 的过滤器创建逻辑
    pub fn create(&self, name: &str) -> Result<Box<dyn Filter>, String> {
        // 首先检查实例缓存
        {
            let instances = self.instances.lock().map_err(|e| e.to_string())?;
            if let Some(filter) = instances.get(name) {
                return Ok(filter.box_clone());
            }
        }

        // 然后尝试使用工厂创建
        let factories = self.factories.lock().map_err(|e| e.to_string())?;
        if let Some(factory) = factories.get(name) {
            Ok(factory())
        } else {
            Err(format!("未找到过滤器: {}", name))
        }
    }

    /// 检查过滤器是否存在
    pub fn exists(&self, name: &str) -> bool {
        if let (Ok(factories), Ok(instances)) = (
            self.factories.lock(),
            self.instances.lock(),
        ) {
            factories.contains_key(name) || instances.contains_key(name)
        } else {
            false
        }
    }

    /// 获取所有已注册的过滤器名称
    pub fn list_filters(&self) -> Vec<String> {
        let mut names = Vec::new();

        if let Ok(factories) = self.factories.lock() {
            names.extend(factories.keys().cloned());
        }

        if let Ok(instances) = self.instances.lock() {
            for name in instances.keys() {
                if !names.contains(name) {
                    names.push(name.clone());
                }
            }
        }

        names.sort();
        names
    }

    /// 注册过滤器链模板
    /// 
    /// 对应原版 Ruby 的过滤器链配置
    pub fn register_chain(&self, name: &str, filter_names: Vec<String>) -> Result<(), String> {
        let mut chains = self.chains.lock().map_err(|e| e.to_string())?;
        chains.insert(name.to_string(), filter_names);
        Ok(())
    }

    /// 创建过滤器链
    /// 
    /// 根据注册的链模板创建完整的过滤器链
    pub fn create_chain(&self, name: &str) -> Result<FilterChain, String> {
        let chains = self.chains.lock().map_err(|e| e.to_string())?;
        
        if let Some(filter_names) = chains.get(name) {
            let mut chain = FilterChain::new();
            
            for filter_name in filter_names {
                let filter = self.create(filter_name)?;
                chain.add_filter(filter);
            }
            
            Ok(chain)
        } else {
            Err(format!("未找到过滤器链: {}", name))
        }
    }

    /// 获取所有已注册的过滤器链名称
    pub fn list_chains(&self) -> Vec<String> {
        if let Ok(chains) = self.chains.lock() {
            let mut names: Vec<String> = chains.keys().cloned().collect();
            names.sort();
            names
        } else {
            Vec::new()
        }
    }

    /// 移除过滤器
    pub fn unregister(&self, name: &str) -> Result<(), String> {
        let mut factories = self.factories.lock().map_err(|e| e.to_string())?;
        let mut instances = self.instances.lock().map_err(|e| e.to_string())?;
        
        factories.remove(name);
        instances.remove(name);
        
        Ok(())
    }

    /// 移除过滤器链
    pub fn unregister_chain(&self, name: &str) -> Result<(), String> {
        let mut chains = self.chains.lock().map_err(|e| e.to_string())?;
        chains.remove(name);
        Ok(())
    }

    /// 清空所有注册
    pub fn clear(&self) -> Result<(), String> {
        let mut factories = self.factories.lock().map_err(|e| e.to_string())?;
        let mut instances = self.instances.lock().map_err(|e| e.to_string())?;
        let mut chains = self.chains.lock().map_err(|e| e.to_string())?;
        
        factories.clear();
        instances.clear();
        chains.clear();
        
        Ok(())
    }

    /// 获取过滤器统计信息
    pub fn stats(&self) -> FilterRegistryStats {
        let factory_count = self.factories.lock()
            .map(|f| f.len())
            .unwrap_or(0);
        
        let instance_count = self.instances.lock()
            .map(|i| i.len())
            .unwrap_or(0);
        
        let chain_count = self.chains.lock()
            .map(|c| c.len())
            .unwrap_or(0);

        FilterRegistryStats {
            factory_count,
            instance_count,
            chain_count,
            total_filters: factory_count + instance_count,
        }
    }
}

impl Default for FilterRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 过滤器注册表统计信息
#[derive(Debug, Clone)]
pub struct FilterRegistryStats {
    pub factory_count: usize,
    pub instance_count: usize,
    pub chain_count: usize,
    pub total_filters: usize,
}

/// 全局过滤器注册表实例
static mut GLOBAL_REGISTRY: Option<FilterRegistry> = None;
static REGISTRY_INIT: std::sync::Once = std::sync::Once::new();

/// 获取全局过滤器注册表
/// 
/// 对应原版 Ruby 的全局过滤器管理
pub fn global_registry() -> &'static FilterRegistry {
    unsafe {
        REGISTRY_INIT.call_once(|| {
            GLOBAL_REGISTRY = Some(FilterRegistry::new());
        });
        GLOBAL_REGISTRY.as_ref().unwrap()
    }
}

/// 注册全局过滤器
pub fn register_global_filter<F, T>(name: &str, factory: F) -> Result<(), String>
where
    F: Fn() -> T + 'static + Send + Sync,
    T: Filter + 'static,
{
    global_registry().register(name, factory)
}

/// 创建全局过滤器实例
pub fn create_global_filter(name: &str) -> Result<Box<dyn Filter>, String> {
    global_registry().create(name)
}

/// 注册全局过滤器链
pub fn register_global_chain(name: &str, filter_names: Vec<String>) -> Result<(), String> {
    global_registry().register_chain(name, filter_names)
}

/// 创建全局过滤器链
pub fn create_global_chain(name: &str) -> Result<FilterChain, String> {
    global_registry().create_chain(name)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::scraper::filter::{BaseFilter, FilterContext};
    use std::any::Any;

    #[derive(Clone)]
    struct TestFilter {
        name: String,
    }

    impl TestFilter {
        fn new(name: String) -> Self {
            Self { name }
        }
    }

    impl Filter for TestFilter {
        fn apply(&self, html: &str, _context: &mut FilterContext) -> crate::core::error::Result<String> {
            Ok(format!("{}:{}", self.name, html))
        }

        fn box_clone(&self) -> Box<dyn Filter> {
            Box::new(self.clone())
        }

        fn as_any(&self) -> &dyn Any {
            self
        }

        fn name(&self) -> &str {
            &self.name
        }
    }

    #[test]
    fn test_registry_register_and_create() {
        let registry = FilterRegistry::new();
        
        // 注册过滤器工厂
        registry.register("test_filter", || TestFilter::new("Test".to_string())).unwrap();
        
        // 检查过滤器是否存在
        assert!(registry.exists("test_filter"));
        
        // 创建过滤器实例
        let filter = registry.create("test_filter").unwrap();
        assert_eq!(filter.name(), "Test");
    }

    #[test]
    fn test_registry_instance() {
        let registry = FilterRegistry::new();
        
        // 注册过滤器实例
        let filter = Box::new(TestFilter::new("Instance".to_string()));
        registry.register_instance("test_instance", filter).unwrap();
        
        // 检查并创建
        assert!(registry.exists("test_instance"));
        let created = registry.create("test_instance").unwrap();
        assert_eq!(created.name(), "Instance");
    }

    #[test]
    fn test_registry_chain() {
        let registry = FilterRegistry::new();
        
        // 注册过滤器
        registry.register("filter1", || TestFilter::new("F1".to_string())).unwrap();
        registry.register("filter2", || TestFilter::new("F2".to_string())).unwrap();
        
        // 注册过滤器链
        registry.register_chain("test_chain", vec!["filter1".to_string(), "filter2".to_string()]).unwrap();
        
        // 创建过滤器链
        let chain = registry.create_chain("test_chain").unwrap();
        assert_eq!(chain.len(), 2);
        
        let names = chain.filter_names();
        assert_eq!(names, vec!["F1", "F2"]);
    }

    #[test]
    fn test_registry_list_operations() {
        let registry = FilterRegistry::new();
        
        registry.register("filter1", || TestFilter::new("F1".to_string())).unwrap();
        registry.register("filter2", || TestFilter::new("F2".to_string())).unwrap();
        registry.register_chain("chain1", vec!["filter1".to_string()]).unwrap();
        
        let filters = registry.list_filters();
        assert_eq!(filters.len(), 2);
        assert!(filters.contains(&"filter1".to_string()));
        assert!(filters.contains(&"filter2".to_string()));
        
        let chains = registry.list_chains();
        assert_eq!(chains.len(), 1);
        assert!(chains.contains(&"chain1".to_string()));
    }

    #[test]
    fn test_registry_stats() {
        let registry = FilterRegistry::new();
        
        registry.register("factory1", || TestFilter::new("F1".to_string())).unwrap();
        registry.register_instance("instance1", Box::new(TestFilter::new("I1".to_string()))).unwrap();
        registry.register_chain("chain1", vec!["factory1".to_string()]).unwrap();
        
        let stats = registry.stats();
        assert_eq!(stats.factory_count, 1);
        assert_eq!(stats.instance_count, 1);
        assert_eq!(stats.chain_count, 1);
        assert_eq!(stats.total_filters, 2);
    }

    #[test]
    fn test_registry_unregister() {
        let registry = FilterRegistry::new();
        
        registry.register("test_filter", || TestFilter::new("Test".to_string())).unwrap();
        assert!(registry.exists("test_filter"));
        
        registry.unregister("test_filter").unwrap();
        assert!(!registry.exists("test_filter"));
    }

    #[test]
    fn test_registry_clear() {
        let registry = FilterRegistry::new();
        
        registry.register("filter1", || TestFilter::new("F1".to_string())).unwrap();
        registry.register_chain("chain1", vec!["filter1".to_string()]).unwrap();
        
        let stats_before = registry.stats();
        assert!(stats_before.total_filters > 0);
        assert!(stats_before.chain_count > 0);
        
        registry.clear().unwrap();
        
        let stats_after = registry.stats();
        assert_eq!(stats_after.total_filters, 0);
        assert_eq!(stats_after.chain_count, 0);
    }

    #[test]
    fn test_global_registry() {
        // 测试全局注册表功能
        register_global_filter("global_test", || TestFilter::new("Global".to_string())).unwrap();
        
        let filter = create_global_filter("global_test").unwrap();
        assert_eq!(filter.name(), "Global");
        
        register_global_chain("global_chain", vec!["global_test".to_string()]).unwrap();
        let chain = create_global_chain("global_chain").unwrap();
        assert_eq!(chain.len(), 1);
    }
}