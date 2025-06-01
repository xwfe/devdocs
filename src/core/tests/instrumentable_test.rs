//! Instrumentable 模块测试
//!
//! 参考原始 Ruby 项目中的 instrumentable_test.rb 实现
//! 为性能监控功能提供单元测试

use crate::core::instrumentable::{instrument, subscribe, InstrumentInfo};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_instrument_basic() {
        // 测试基本的计时功能
        let result = instrument("test_event", HashMap::new(), || {
            // 模拟一些工作
            thread::sleep(Duration::from_millis(10));
            "test_result"
        });
        
        assert_eq!(result, "test_result");
    }
    
    #[test]
    fn test_instrument_with_payload() {
        // 测试带有负载的计时
        let mut payload = HashMap::new();
        payload.insert("test_key".to_string(), "test_value".to_string());
        
        let result = instrument("test_event_with_payload", payload, || {
            "payload_result"
        });
        
        assert_eq!(result, "payload_result");
    }
    
    #[test]
    fn test_subscribe_and_notify() {
        // 使用 Arc<Mutex<>> 在测试中捕获回调中的信息
        let test_called = Arc::new(Mutex::new(false));
        let test_name = Arc::new(Mutex::new(String::new()));
        let test_payload = Arc::new(Mutex::new(HashMap::new()));
        
        let test_called_clone = test_called.clone();
        let test_name_clone = test_name.clone();
        let test_payload_clone = test_payload.clone();
        
        // 订阅事件
        subscribe("test_notification", move |info: &InstrumentInfo| {
            let mut called = test_called_clone.lock().unwrap();
            *called = true;
            
            let mut name = test_name_clone.lock().unwrap();
            *name = info.name.clone();
            
            let mut payload = test_payload_clone.lock().unwrap();
            *payload = info.payload.clone();
        });
        
        // 产生事件
        let mut event_payload = HashMap::new();
        event_payload.insert("key".to_string(), "value".to_string());
        
        instrument("test_notification", event_payload, || {
            // 等待一会儿确保回调已处理
            thread::sleep(Duration::from_millis(10));
        });
        
        // 验证回调是否被调用
        let called = test_called.lock().unwrap();
        assert!(*called, "Callback should have been called");
        
        let name = test_name.lock().unwrap();
        assert_eq!(*name, "test_notification");
        
        let payload = test_payload.lock().unwrap();
        assert_eq!(payload.get("key"), Some(&"value".to_string()));
    }
    
    #[test]
    fn test_wildcard_subscription() {
        // 测试通配符订阅
        let test_called = Arc::new(Mutex::new(false));
        let test_called_clone = test_called.clone();
        
        // 使用通配符订阅所有事件
        subscribe("*", move |_info: &InstrumentInfo| {
            let mut called = test_called_clone.lock().unwrap();
            *called = true;
        });
        
        // 产生一个随机命名的事件
        instrument("random_event_name", HashMap::new(), || {
            // 等待一会儿确保回调已处理
            thread::sleep(Duration::from_millis(10));
        });
        
        // 验证通配符回调是否被调用
        let called = test_called.lock().unwrap();
        assert!(*called, "Wildcard callback should have been called");
    }
    
    #[test]
    fn test_multiple_subscribers() {
        // 测试多个订阅者
        let counter1 = Arc::new(Mutex::new(0));
        let counter2 = Arc::new(Mutex::new(0));
        
        let counter1_clone = counter1.clone();
        let counter2_clone = counter2.clone();
        
        // 添加两个订阅者到同一事件
        subscribe("multi_test", move |_info: &InstrumentInfo| {
            let mut count = counter1_clone.lock().unwrap();
            *count += 1;
        });
        
        subscribe("multi_test", move |_info: &InstrumentInfo| {
            let mut count = counter2_clone.lock().unwrap();
            *count += 1;
        });
        
        // 产生事件
        instrument("multi_test", HashMap::new(), || {
            // 等待一会儿确保回调已处理
            thread::sleep(Duration::from_millis(10));
        });
        
        // 验证两个回调是否都被调用
        let count1 = counter1.lock().unwrap();
        let count2 = counter2.lock().unwrap();
        assert_eq!(*count1, 1);
        assert_eq!(*count2, 1);
    }
    
    #[test]
    fn test_performance_measure() {
        // 测试性能测量功能
        let duration_captured = Arc::new(Mutex::new(false));
        let duration_captured_clone = duration_captured.clone();
        
        subscribe("perf_test", move |info: &InstrumentInfo| {
            let mut captured = duration_captured_clone.lock().unwrap();
            // 确认事件有持续时间
            *captured = info.duration.is_some();
        });
        
        instrument("perf_test", HashMap::new(), || {
            // 执行一个可测量的操作
            thread::sleep(Duration::from_millis(20));
        });
        
        // 验证持续时间被捕获
        let captured = duration_captured.lock().unwrap();
        assert!(*captured, "Duration should have been captured");
    }
}
