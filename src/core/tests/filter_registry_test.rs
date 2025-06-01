//! FilterRegistry 模块测试
//!
//! 为过滤器注册表提供单元测试

use crate::core::filter_registry::FilterRegistry;
use crate::core::scraper::filter::{Filter, FilterContext, FilterOutput};
use scraper::Html;
use std::sync::{Arc, Mutex};

// 测试用过滤器
struct TestFilter {
    name: String,
}

impl TestFilter {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
        }
    }
}

impl Filter for TestFilter {
    fn call(&self, html: &Html, _context: &mut FilterContext) -> FilterOutput {
        FilterOutput::Html(html.clone())
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_register_and_create() {
        let registry = Arc::new(Mutex::new(FilterRegistry::new()));

        // 注册过滤器
        {
            let mut reg = registry.lock().unwrap();
            reg.register("test_filter", || TestFilter::new("test_filter"));
        }

        // 创建过滤器实例
        {
            let reg = registry.lock().unwrap();
            let filter = reg.create("test_filter");
            assert!(filter.is_some(), "过滤器应该被成功创建");

            let filter = filter.unwrap();
            let any_filter = filter.as_any();
            let downcast_result = any_filter.downcast_ref::<TestFilter>();
            assert!(downcast_result.is_some(), "应该能够向下转型为 TestFilter");

            if let Some(test_filter) = downcast_result {
                assert_eq!(test_filter.name, "test_filter");
            }
        }
    }

    #[test]
    fn test_contains() {
        let registry = Arc::new(Mutex::new(FilterRegistry::new()));

        {
            let mut reg = registry.lock().unwrap();
            reg.register("test_filter", || TestFilter::new("test_filter"));
        }

        {
            let reg = registry.lock().unwrap();
            assert!(reg.contains("test_filter"), "应该包含已注册的过滤器");
            assert!(!reg.contains("non_existent"), "不应该包含未注册的过滤器");
        }
    }

    #[test]
    fn test_filter_names() {
        let registry = Arc::new(Mutex::new(FilterRegistry::new()));

        {
            let mut reg = registry.lock().unwrap();
            reg.register("filter1", || TestFilter::new("filter1"));
            reg.register("filter2", || TestFilter::new("filter2"));
            reg.register("filter3", || TestFilter::new("filter3"));
        }

        {
            let reg = registry.lock().unwrap();
            let names = reg.filter_names();
            assert_eq!(names.len(), 3);
            assert!(names.contains(&"filter1".to_string()));
            assert!(names.contains(&"filter2".to_string()));
            assert!(names.contains(&"filter3".to_string()));
        }
    }

    #[test]
    fn test_global_registry() {
        let registry = FilterRegistry::global();

        // 测试全局注册表是否可用
        {
            let mut reg = registry.lock().unwrap();
            reg.register("global_filter", || TestFilter::new("global_filter"));
        }

        {
            let reg = registry.lock().unwrap();
            assert!(reg.contains("global_filter"));
        }

        // 再次获取全局注册表，应该是同一个实例
        let registry2 = FilterRegistry::global();
        {
            let reg = registry2.lock().unwrap();
            assert!(reg.contains("global_filter"));
        }
    }
}
