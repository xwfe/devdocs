//! Entry 模型
//!
//! 严格对齐原版 Ruby 项目中的 models/entry.rb 实现

use serde::{Deserialize, Serialize};

/// 条目无效错误
#[derive(Debug, Clone)]
pub struct Invalid {
    pub message: String,
}

impl std::fmt::Display for Invalid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for Invalid {}

/// 文档条目
/// 
/// 对应原版 Ruby 的 Entry 类
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Entry {
    name: Option<String>,
    path: Option<String>,
    entry_type: Option<String>,
}

impl Entry {
    /// 创建新条目
    /// 
    /// 对应原版 Ruby 的 initialize 方法
    pub fn new(name: Option<String>, path: Option<String>, entry_type: Option<String>) -> Result<Self, Invalid> {
        let mut entry = Self {
            name: None,
            path,
            entry_type: None,
        };

        entry.set_name(name);
        entry.set_type(entry_type);

        // 验证非根条目
        if !entry.is_root() {
            if entry.name.is_none() || entry.name.as_ref().unwrap().is_empty() {
                return Err(Invalid { message: "missing name".to_string() });
            }
            if entry.path.is_none() || entry.path.as_ref().unwrap().is_empty() {
                return Err(Invalid { message: "missing path".to_string() });
            }
            if entry.entry_type.is_none() || entry.entry_type.as_ref().unwrap().is_empty() {
                return Err(Invalid { message: "missing type".to_string() });
            }
        }

        Ok(entry)
    }

    /// 获取名称
    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    /// 获取路径
    pub fn path(&self) -> Option<&String> {
        self.path.as_ref()
    }

    /// 获取类型
    pub fn entry_type(&self) -> Option<&String> {
        self.entry_type.as_ref()
    }

    /// 设置名称
    /// 
    /// 对应原版 Ruby 的 name= 方法，会自动 strip
    pub fn set_name(&mut self, name: Option<String>) {
        self.name = name.and_then(|n| {
            let trimmed = n.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        });
    }

    /// 设置路径
    pub fn set_path(&mut self, path: Option<String>) {
        self.path = path;
    }

    /// 设置类型
    /// 
    /// 对应原版 Ruby 的 type= 方法，会自动 strip
    pub fn set_type(&mut self, entry_type: Option<String>) {
        self.entry_type = entry_type.and_then(|t| {
            let trimmed = t.trim();
            if trimmed.is_empty() {
                None
            } else {
                Some(trimmed.to_string())
            }
        });
    }

    /// 检查是否为根条目
    /// 
    /// 对应原版 Ruby 的 root? 方法
    pub fn is_root(&self) -> bool {
        self.path.as_ref().map_or(false, |p| p == "index")
    }

    /// 转换为 JSON 格式
    /// 
    /// 对应原版 Ruby 的 as_json 方法
    pub fn as_json(&self) -> EntryJson {
        EntryJson {
            name: self.name.clone(),
            path: self.path.clone(),
            entry_type: self.entry_type.clone(),
        }
    }
}

/// JSON 序列化结构
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct EntryJson {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid_entry() {
        let entry = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        );
        
        assert!(entry.is_ok());
        let entry = entry.unwrap();
        assert_eq!(entry.name(), Some(&"test".to_string()));
        assert_eq!(entry.path(), Some(&"test/path".to_string()));
        assert_eq!(entry.entry_type(), Some(&"function".to_string()));
    }

    #[test]
    fn test_new_missing_name() {
        let result = Entry::new(
            None,
            Some("test/path".to_string()),
            Some("function".to_string()),
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "missing name");
    }

    #[test]
    fn test_new_missing_path() {
        let result = Entry::new(
            Some("test".to_string()),
            None,
            Some("function".to_string()),
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "missing path");
    }

    #[test]
    fn test_new_missing_type() {
        let result = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            None,
        );
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "missing type");
    }

    #[test]
    fn test_root_entry() {
        let entry = Entry::new(None, Some("index".to_string()), None).unwrap();
        assert!(entry.is_root());
        assert_eq!(entry.path(), Some(&"index".to_string()));
        assert_eq!(entry.name(), None);
        assert_eq!(entry.entry_type(), None);
    }

    #[test]
    fn test_is_root() {
        let root_entry = Entry::new(None, Some("index".to_string()), None).unwrap();
        assert!(root_entry.is_root());

        let normal_entry = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();
        assert!(!normal_entry.is_root());
    }

    #[test]
    fn test_set_name_with_whitespace() {
        let mut entry = Entry::new(None, Some("index".to_string()), None).unwrap();
        entry.set_name(Some("  test  ".to_string()));
        assert_eq!(entry.name(), Some(&"test".to_string()));
    }

    #[test]
    fn test_set_type_with_whitespace() {
        let mut entry = Entry::new(None, Some("index".to_string()), None).unwrap();
        entry.set_type(Some("  function  ".to_string()));
        assert_eq!(entry.entry_type(), Some(&"function".to_string()));
    }

    #[test]
    fn test_equality() {
        let entry1 = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        let entry2 = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        let entry3 = Entry::new(
            Some("different".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        assert_eq!(entry1, entry2);
        assert_ne!(entry1, entry3);
    }

    #[test]
    fn test_as_json() {
        let entry = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        let json = entry.as_json();
        assert_eq!(json.name, Some("test".to_string()));
        assert_eq!(json.path, Some("test/path".to_string()));
        assert_eq!(json.entry_type, Some("function".to_string()));
    }
}