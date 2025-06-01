//! FilterStack 模块测试
//!
//! 为过滤器栈提供单元测试

use crate::core::filter_stack::FilterStack;
use crate::core::scraper::filter::{Filter, FilterContext, FilterOutput};
use scraper::Html;
use std::cell::RefCell;
use std::rc::Rc;

// 测试用过滤器
struct TestFilter {
    name: String,
    call_count: Rc<RefCell<usize>>,
}

impl TestFilter {
    fn new(name: &str, call_count: Rc<RefCell<usize>>) -> Self {
        Self {
            name: name.to_string(),
            call_count,
        }
    }
}

impl Filter for TestFilter {
    fn call(&self, html: &Html, _context: &mut FilterContext) -> FilterOutput {
        *self.call_count.borrow_mut() += 1;
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
    fn test_push_filter() {
        let mut stack = FilterStack::new();
        let call_count = Rc::new(RefCell::new(0));

        // 注册过滤器
        stack.register("test_filter", || {
            TestFilter::new("test_filter", call_count.clone())
        });

        // 添加过滤器到栈中
        let result = stack.push("test_filter");
        assert!(result.is_ok());

        // 验证过滤器栈大小
        assert_eq!(stack.len(), 1);

        // 测试不存在的过滤器
        let result = stack.push("non_existent");
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_before() {
        let mut stack = FilterStack::new();
        let count1 = Rc::new(RefCell::new(0));
        let count2 = Rc::new(RefCell::new(0));

        // 注册过滤器
        stack.register("filter1", || TestFilter::new("filter1", count1.clone()));
        stack.register("filter2", || TestFilter::new("filter2", count2.clone()));

        // 添加第一个过滤器
        stack.push("filter1").unwrap();

        // 在第一个过滤器之前插入第二个过滤器
        let result = stack.insert_before("filter1", "filter2");
        assert!(result.is_ok());

        // 验证过滤器顺序
        let filters = stack.filter_names();
        assert_eq!(filters, vec!["filter2", "filter1"]);

        // 测试在不存在的过滤器之前插入
        let result = stack.insert_before("non_existent", "filter1");
        assert!(result.is_err());
    }

    #[test]
    fn test_insert_after() {
        let mut stack = FilterStack::new();
        let count1 = Rc::new(RefCell::new(0));
        let count2 = Rc::new(RefCell::new(0));

        // 注册过滤器
        stack.register("filter1", || TestFilter::new("filter1", count1.clone()));
        stack.register("filter2", || TestFilter::new("filter2", count2.clone()));

        // 添加第一个过滤器
        stack.push("filter1").unwrap();

        // 在第一个过滤器之后插入第二个过滤器
        let result = stack.insert_after("filter1", "filter2");
        assert!(result.is_ok());

        // 验证过滤器顺序
        let filters = stack.filter_names();
        assert_eq!(filters, vec!["filter1", "filter2"]);

        // 测试在不存在的过滤器之后插入
        let result = stack.insert_after("non_existent", "filter1");
        assert!(result.is_err());
    }

    #[test]
    fn test_replace() {
        let mut stack = FilterStack::new();
        let count1 = Rc::new(RefCell::new(0));
        let count2 = Rc::new(RefCell::new(0));

        // 注册过滤器
        stack.register("filter1", || TestFilter::new("filter1", count1.clone()));
        stack.register("filter2", || TestFilter::new("filter2", count2.clone()));

        // 添加第一个过滤器
        stack.push("filter1").unwrap();

        // 替换过滤器
        let result = stack.replace("filter1", "filter2");
        assert!(result.is_ok());

        // 验证过滤器栈大小和名称
        assert_eq!(stack.len(), 1);
        let filters = stack.filter_names();
        assert_eq!(filters, vec!["filter2"]);

        // 测试替换不存在的过滤器
        let result = stack.replace("non_existent", "filter1");
        assert!(result.is_err());
    }

    #[test]
    fn test_remove() {
        let mut stack = FilterStack::new();
        let count = Rc::new(RefCell::new(0));

        // 注册过滤器
        stack.register("filter", || TestFilter::new("filter", count.clone()));

        // 添加过滤器
        stack.push("filter").unwrap();
        assert_eq!(stack.len(), 1);

        // 移除过滤器
        let result = stack.remove("filter");
        assert!(result.is_ok());
        assert_eq!(stack.len(), 0);

        // 测试移除不存在的过滤器
        let result = stack.remove("non_existent");
        assert!(result.is_err());
    }

    #[test]
    fn test_clear() {
        let mut stack = FilterStack::new();
        let count1 = Rc::new(RefCell::new(0));
        let count2 = Rc::new(RefCell::new(0));

        // 注册过滤器
        stack.register("filter1", || TestFilter::new("filter1", count1.clone()));
        stack.register("filter2", || TestFilter::new("filter2", count2.clone()));

        // 添加过滤器
        stack.push("filter1").unwrap();
        stack.push("filter2").unwrap();
        assert_eq!(stack.len(), 2);

        // 清空过滤器栈
        stack.clear();
        assert_eq!(stack.len(), 0);
    }

    #[test]
    fn test_process() {
        let mut stack = FilterStack::new();
        let count1 = Rc::new(RefCell::new(0));
        let count2 = Rc::new(RefCell::new(0));

        // 注册过滤器
        stack.register("filter1", || TestFilter::new("filter1", count1.clone()));
        stack.register("filter2", || TestFilter::new("filter2", count2.clone()));

        // 添加过滤器
        stack.push("filter1").unwrap();
        stack.push("filter2").unwrap();

        // 处理HTML
        let html = Html::parse_document("<html><body><p>Test</p></body></html>");
        let mut context = FilterContext::default();

        let result = stack.process(&html, &mut context);

        // 验证每个过滤器都被调用了
        assert_eq!(*count1.borrow(), 1);
        assert_eq!(*count2.borrow(), 1);

        // 验证结果是HTML
        match result {
            FilterOutput::Html(_) => assert!(true),
            _ => assert!(false, "应该返回HTML输出"),
        }
    }
}
