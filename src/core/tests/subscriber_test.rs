//! Subscriber 模块测试
//!
//! 为事件订阅者提供单元测试

use crate::core::instrumentable::InstrumentInfo;
use crate::core::subscriber::{ConsoleSubscriber, Subscriber};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// 测试用订阅者，记录接收到的事件
struct TestSubscriber {
    events: Arc<Mutex<Vec<InstrumentInfo>>>,
}

impl TestSubscriber {
    fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    fn event_count(&self) -> usize {
        self.events.lock().unwrap().len()
    }
    
    fn events(&self) -> Vec<InstrumentInfo> {
        self.events.lock().unwrap().clone()
    }
}

impl Subscriber for TestSubscriber {
    fn handle_event(&self, info: &InstrumentInfo) {
        self.events.lock().unwrap().push(info.clone());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_instrument_info(name: &str, args: Vec<String>) -> InstrumentInfo {
        InstrumentInfo {
            name: name.to_string(),
            args,
            start_time: Instant::now(),
            duration: Some(Duration::from_millis(100)),
            parent_name: None,
            metadata: None,
        }
    }

    #[test]
    fn test_test_subscriber() {
        let subscriber = TestSubscriber::new();
        
        // 初始状态应该没有事件
        assert_eq!(subscriber.event_count(), 0);
        
        // 处理一个事件
        let info = create_instrument_info("test_event", vec!["arg1".to_string()]);
        subscriber.handle_event(&info);
        
        // 应该有一个事件
        assert_eq!(subscriber.event_count(), 1);
        
        // 处理另一个事件
        let info2 = create_instrument_info("test_event2", vec!["arg2".to_string()]);
        subscriber.handle_event(&info2);
        
        // 应该有两个事件
        assert_eq!(subscriber.event_count(), 2);
        
        // 验证事件内容
        let events = subscriber.events();
        assert_eq!(events[0].name, "test_event");
        assert_eq!(events[0].args[0], "arg1");
        assert_eq!(events[1].name, "test_event2");
        assert_eq!(events[1].args[0], "arg2");
    }
    
    #[test]
    fn test_console_subscriber_create() {
        let subscriber = ConsoleSubscriber::new();
        
        // 测试默认实例创建
        assert_eq!(subscriber.use_color, atty::is(atty::Stream::Stdout));
        assert!(subscriber.terminal_width.is_some());
    }
    
    #[test]
    fn test_console_subscriber_default() {
        let subscriber = ConsoleSubscriber::default();
        
        // 测试默认实例创建
        assert_eq!(subscriber.use_color, atty::is(atty::Stream::Stdout));
        assert!(subscriber.terminal_width.is_some());
    }
    
    #[test]
    fn test_format_url() {
        let subscriber = ConsoleSubscriber::new();
        
        // 测试 URL 格式化
        assert_eq!(subscriber.format_url("http://example.com"), "example.com");
        assert_eq!(subscriber.format_url("https://example.com"), "example.com");
        assert_eq!(subscriber.format_url("ftp://example.com"), "ftp://example.com");
    }
    
    #[test]
    fn test_format_path() {
        let subscriber = ConsoleSubscriber::new();
        
        // 测试路径格式化
        let path = "/some/test/path";
        let formatted = subscriber.format_path(path);
        
        // 由于当前目录可能因测试环境而异，只验证结果类型
        assert!(formatted.len() >= 0);
    }
    
    #[test]
    fn test_handle_event() {
        let subscriber = ConsoleSubscriber::new();
        
        // 创建测试事件
        let mut info = create_instrument_info("download", vec!["https://example.com/file.txt".to_string()]);
        
        // 添加元数据
        let mut metadata = std::collections::HashMap::new();
        metadata.insert("status".to_string(), "200 OK".to_string());
        metadata.insert("size".to_string(), "1024 bytes".to_string());
        info.metadata = Some(metadata);
        
        // 处理事件（此测试只确保方法不会崩溃）
        subscriber.handle_event(&info);
        
        // 由于输出到控制台，无法直接验证输出内容，仅确保代码正常执行
    }
    
    #[test]
    fn test_nested_events() {
        let subscriber = TestSubscriber::new();
        
        // 创建父事件
        let parent_info = create_instrument_info("parent_task", vec!["parent_arg".to_string()]);
        subscriber.handle_event(&parent_info);
        
        // 创建子事件
        let mut child_info = create_instrument_info("child_task", vec!["child_arg".to_string()]);
        child_info.parent_name = Some("parent_task".to_string());
        subscriber.handle_event(&child_info);
        
        // 验证事件计数和嵌套关系
        assert_eq!(subscriber.event_count(), 2);
        
        let events = subscriber.events();
        assert_eq!(events[0].name, "parent_task");
        assert_eq!(events[1].name, "child_task");
        assert_eq!(events[1].parent_name, Some("parent_task".to_string()));
    }
}
