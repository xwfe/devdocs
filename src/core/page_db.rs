//! PageDb 模块
//!
//! 严格对齐原版 Ruby 项目中的 core/page_db.rb 实现

use std::collections::HashMap;

/// 页面数据库
/// 
/// 对应原版 Ruby 的 PageDb 类
#[derive(Debug, Clone)]
pub struct PageDb {
    pages: HashMap<String, String>,
}

impl PageDb {
    /// 创建新的页面数据库
    /// 
    /// 对应原版 Ruby 的 initialize 方法
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }

    /// 添加页面
    /// 
    /// 对应原版 Ruby 的 add 方法
    pub fn add(&mut self, path: String, content: String) {
        self.pages.insert(path, content);
    }

    /// 检查是否为空
    /// 
    /// 对应原版 Ruby 的 empty? 方法
    pub fn empty(&self) -> bool {
        self.pages.is_empty()
    }

    /// 检查是否为空（别名）
    /// 
    /// 对应原版 Ruby 的 blank? 方法
    pub fn blank(&self) -> bool {
        self.empty()
    }

    /// 转换为 JSON 对象
    /// 
    /// 对应原版 Ruby 的 as_json 方法
    pub fn as_json(&self) -> &HashMap<String, String> {
        &self.pages
    }

    /// 转换为 JSON 字符串
    /// 
    /// 对应原版 Ruby 的 to_json 方法
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.pages).unwrap_or_default()
    }
}

impl Default for PageDb {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let db = PageDb::new();
        assert!(db.empty());
    }

    #[test]
    fn test_add() {
        let mut db = PageDb::new();
        db.add("test/path".to_string(), "<html>content</html>".to_string());
        
        assert!(!db.empty());
        assert_eq!(db.pages.len(), 1);
        assert_eq!(db.pages.get("test/path"), Some(&"<html>content</html>".to_string()));
    }

    #[test]
    fn test_empty_and_blank() {
        let mut db = PageDb::new();
        assert!(db.empty());
        assert!(db.blank());

        db.add("test".to_string(), "content".to_string());
        assert!(!db.empty());
        assert!(!db.blank());
    }

    #[test]
    fn test_as_json() {
        let mut db = PageDb::new();
        db.add("test1".to_string(), "content1".to_string());
        db.add("test2".to_string(), "content2".to_string());

        let json = db.as_json();
        assert_eq!(json.len(), 2);
        assert_eq!(json.get("test1"), Some(&"content1".to_string()));
        assert_eq!(json.get("test2"), Some(&"content2".to_string()));
    }

    #[test]
    fn test_to_json() {
        let mut db = PageDb::new();
        db.add("test".to_string(), "content".to_string());

        let json_str = db.to_json();
        assert!(json_str.contains("test"));
        assert!(json_str.contains("content"));
    }
}