//! 性能监控模块
//!
//! 参考原始 Ruby 项目中的 instrumentable.rb 实现
//! 提供性能监控和事件通知功能

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// 性能指标信息
#[derive(Debug, Clone)]
pub struct InstrumentInfo {
    /// 事件名称
    pub name: String,
    /// 事件开始时间
    pub start_time: Instant,
    /// 事件持续时间
    pub duration: Option<Duration>,
    /// 附加信息
    pub payload: HashMap<String, String>,
}

/// 事件监听回调函数类型
type NotificationCallback = Box<dyn Fn(&InstrumentInfo) -> () + Send + Sync>;

/// 通知中心，管理所有事件订阅
pub struct NotificationCenter {
    subscribers: Mutex<HashMap<String, Vec<NotificationCallback>>>,
}

impl NotificationCenter {
    /// 创建新的通知中心
    pub fn new() -> Self {
        Self {
            subscribers: Mutex::new(HashMap::new()),
        }
    }

    /// 订阅事件
    pub fn subscribe<F>(&self, event: &str, callback: F)
    where
        F: Fn(&InstrumentInfo) -> () + 'static + Send + Sync,
    {
        let mut subscribers = self.subscribers.lock().unwrap();
        let callbacks = subscribers
            .entry(event.to_string())
            .or_insert_with(Vec::new);
        callbacks.push(Box::new(callback));
    }

    /// 发布事件
    pub fn publish(&self, info: &InstrumentInfo) {
        let subscribers = self.subscribers.lock().unwrap();

        // 按事件名称匹配的订阅
        if let Some(callbacks) = subscribers.get(&info.name) {
            for callback in callbacks {
                callback(info);
            }
        }

        // 通配符订阅
        if let Some(callbacks) = subscribers.get("*") {
            for callback in callbacks {
                callback(info);
            }
        }
    }
}

lazy_static! {
    static ref NOTIFICATION_CENTER: Arc<NotificationCenter> = Arc::new(NotificationCenter::new());
}

/// 订阅事件
pub fn subscribe<F>(event: &str, callback: F)
where
    F: Fn(&InstrumentInfo) -> () + 'static + Send + Sync,
{
    NOTIFICATION_CENTER.subscribe(event, callback);
}

/// 执行被监控的代码块，记录执行时间和结果
pub fn instrument<T, F>(name: &str, payload: HashMap<String, String>, action: F) -> T
where
    F: FnOnce() -> T,
{
    let start_time = Instant::now();

    let initial_info = InstrumentInfo {
        name: name.to_string(),
        start_time,
        duration: None,
        payload: payload.clone(),
    };

    let result = action();

    let duration = start_time.elapsed();
    let final_info = InstrumentInfo {
        name: name.to_string(),
        start_time,
        duration: Some(duration),
        payload,
    };
    
    NOTIFICATION_CENTER.publish(&final_info);

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_instrument() {
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = counter.clone();

        subscribe("test_event", move |info| {
            let mut count = counter_clone.lock().unwrap();
            *count += 1;
            assert_eq!(info.name, "test_event");
            assert!(info.duration.is_some());
        });

        let result = instrument(
            "test_event",
            [("key".to_string(), "value".to_string())]
                .iter()
                .cloned()
                .collect(),
            || {
                thread::sleep(Duration::from_millis(10));
                42
            },
        );

        assert_eq!(result, 42);
        assert_eq!(*counter.lock().unwrap(), 1);
    }
}
