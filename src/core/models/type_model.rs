//! Type 模型
//!
//! 严格对齐原版 Ruby 项目中的 models/type.rb 实现

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 文档类型
/// 
/// 对应原版 Ruby 的 Type = Struct.new :name, :count
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Type {
    pub name: String,
    pub count: usize,
}

impl Type {
    /// 创建新的类型
    /// 
    /// 对应原版 Ruby 的 initialize 方法
    pub fn new(name: String) -> Self {
        Self {
            name,
            count: 0,
        }
    }

    /// 获取 slug
    /// 
    /// 对应原版 Ruby 的 slug 方法
    /// name.parameterize
    pub fn slug(&self) -> String {
        self.name.to_lowercase()
            .chars()
            .map(|c| {
                if c.is_alphanumeric() {
                    c
                } else if c.is_whitespace() || c == '-' || c == '_' {
                    '-'
                } else {
                    '-'
                }
            })
            .collect::<String>()
            .split('-')
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join("-")
    }

    /// 转换为 JSON 对象
    /// 
    /// 对应原版 Ruby 的 as_json 方法
    /// to_h.merge! slug: slug
    pub fn as_json(&self) -> TypeJson {
        TypeJson {
            name: self.name.clone(),
            count: self.count,
            slug: self.slug(),
        }
    }

    /// 转换为哈希
    /// 
    /// 对应原版 Ruby Struct 的 to_h 方法
    pub fn to_h(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("name".to_string(), serde_json::Value::String(self.name.clone()));
        map.insert("count".to_string(), serde_json::Value::Number(serde_json::Number::from(self.count)));
        map
    }
}

/// JSON 序列化结构
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct TypeJson {
    pub name: String,
    pub count: usize,
    pub slug: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_type() {
        let type_obj = Type::new("Function".to_string());
        assert_eq!(type_obj.name, "Function");
        assert_eq!(type_obj.count, 0);
    }

    #[test]
    fn test_slug() {
        let type_obj = Type::new("Function".to_string());
        assert_eq!(type_obj.slug(), "function");
        
        let type_obj = Type::new("Class Method".to_string());
        assert_eq!(type_obj.slug(), "class-method");
    }

    #[test]
    fn test_as_json() {
        let mut type_obj = Type::new("Function".to_string());
        type_obj.count = 5;
        
        let json = type_obj.as_json();
        assert_eq!(json.name, "Function");
        assert_eq!(json.count, 5);
        assert_eq!(json.slug, "function");
    }

    #[test]
    fn test_to_h() {
        let mut type_obj = Type::new("Function".to_string());
        type_obj.count = 3;
        
        let hash = type_obj.to_h();
        assert_eq!(hash.get("name"), Some(&serde_json::Value::String("Function".to_string())));
        assert_eq!(hash.get("count"), Some(&serde_json::Value::Number(serde_json::Number::from(3))));
    }
}