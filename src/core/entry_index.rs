//! EntryIndex 模块
//!
//! 严格对齐原版 Ruby 项目中的 core/entry_index.rb 实现

use crate::core::models::{Entry, Type};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

/// 条目索引
/// 
/// 对应原版 Ruby 的 EntryIndex 类
#[derive(Debug, Clone)]
pub struct EntryIndex {
    entries: Vec<Entry>,
    index: HashSet<String>,
    types: HashMap<String, Type>,
}

impl EntryIndex {
    /// 创建新的条目索引
    /// 
    /// 对应原版 Ruby 的 initialize 方法
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            index: HashSet::new(),
            types: HashMap::new(),
        }
    }

    /// 添加条目
    /// 
    /// 对应原版 Ruby 的 add 方法
    pub fn add(&mut self, entry: Entry) {
        if entry.is_root() {
            return;
        }

        // 检查是否已存在
        let json_string = serde_json::to_string(&entry.as_json()).unwrap_or_default();
        if self.index.insert(json_string) {
            // 更新类型计数
            if let Some(entry_type) = entry.entry_type() {
                self.types
                    .entry(entry_type.clone())
                    .or_insert_with(|| Type::new(entry_type.clone()))
                    .count += 1;
            }
            
            self.entries.push(entry);
        }
    }

    /// 批量添加条目
    /// 
    /// 对应原版 Ruby 的 add 方法处理数组的情况
    pub fn add_entries(&mut self, entries: Vec<Entry>) {
        for entry in entries {
            self.add(entry);
        }
    }

    /// 检查是否为空
    /// 
    /// 对应原版 Ruby 的 empty? 方法
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    /// 检查是否为空（别名）
    /// 
    /// 对应原版 Ruby 的 blank? 方法
    pub fn is_blank(&self) -> bool {
        self.is_empty()
    }

    /// 获取条目数量
    /// 
    /// 对应原版 Ruby 的 length 方法
    pub fn length(&self) -> usize {
        self.entries.len()
    }

    /// 获取条目数量（别名）
    pub fn len(&self) -> usize {
        self.length()
    }

    /// 转换为 JSON 对象
    /// 
    /// 对应原版 Ruby 的 as_json 方法
    pub fn as_json(&self) -> IndexJson {
        IndexJson {
            entries: self.entries_as_json(),
            types: self.types_as_json(),
        }
    }

    /// 转换为 JSON 字符串
    /// 
    /// 对应原版 Ruby 的 to_json 方法
    pub fn to_json(&self) -> String {
        serde_json::to_string(&self.as_json()).unwrap_or_default()
    }

    /// 检查是否存在条目
    pub fn present(&self) -> bool {
        !self.is_empty()
    }

    /// 获取条目列表的排序后的 JSON 表示
    /// 
    /// 对应原版 Ruby 的 entries_as_json 方法
    fn entries_as_json(&self) -> Vec<serde_json::Value> {
        let mut entries: Vec<_> = self.entries.iter().collect();
        entries.sort_by(|a, b| self.sort_fn(a.name().unwrap_or(&String::new()), b.name().unwrap_or(&String::new())));
        
        entries.into_iter()
            .map(|entry| serde_json::to_value(entry.as_json()).unwrap_or_default())
            .collect()
    }

    /// 获取类型列表的排序后的 JSON 表示
    /// 
    /// 对应原版 Ruby 的 types_as_json 方法
    fn types_as_json(&self) -> Vec<serde_json::Value> {
        let mut types: Vec<_> = self.types.values().collect();
        types.sort_by(|a, b| self.sort_fn(&a.name, &b.name));
        
        types.into_iter()
            .map(|type_obj| serde_json::to_value(type_obj.as_json()).unwrap_or_default())
            .collect()
    }

    /// 排序函数
    /// 
    /// 对应原版 Ruby 的 sort_fn 方法
    /// 处理数字开头的字符串的特殊排序逻辑
    fn sort_fn(&self, a: &str, b: &str) -> std::cmp::Ordering {
        let a_starts_with_digit = a.chars().next().map_or(false, |c| c.is_ascii_digit());
        let b_starts_with_digit = b.chars().next().map_or(false, |c| c.is_ascii_digit());

        if a_starts_with_digit || b_starts_with_digit {
            let a_split = self.split_with_ints(a);
            let b_split = self.split_with_ints(b);

            if a_split.len() == 1 && b_split.len() == 1 {
                return a.to_lowercase().cmp(&b.to_lowercase());
            }
            if a_split.len() == 1 {
                return std::cmp::Ordering::Greater;
            }
            if b_split.len() == 1 {
                return std::cmp::Ordering::Less;
            }

            let mut a_parts = self.convert_to_comparable(&a_split);
            let mut b_parts = self.convert_to_comparable(&b_split);

            // 填充较短的数组
            while a_parts.len() < b_parts.len() {
                a_parts.insert(a_parts.len() - 1, ComparablePart::Number(0));
            }
            while b_parts.len() < a_parts.len() {
                b_parts.insert(b_parts.len() - 1, ComparablePart::Number(0));
            }

            a_parts.cmp(&b_parts)
        } else {
            a.to_lowercase().cmp(&b.to_lowercase())
        }
    }

    /// 按整数分割字符串
    /// 
    /// 对应原版 Ruby 的 SPLIT_INTS 正则表达式逻辑
    fn split_with_ints<'a>(&self, s: &'a str) -> Vec<&'a str> {
        // 简化实现，按点分割但保留数字逻辑
        s.split('.').collect()
    }

    /// 转换为可比较的部分
    fn convert_to_comparable(&self, parts: &[&str]) -> Vec<ComparablePart> {
        parts.iter().enumerate().map(|(i, part)| {
            if i == parts.len() - 1 {
                // 最后一部分保持为字符串
                ComparablePart::String(part.to_string())
            } else if let Ok(num) = part.parse::<i32>() {
                ComparablePart::Number(num)
            } else {
                ComparablePart::String(part.to_string())
            }
        }).collect()
    }
}

/// 可比较的部分枚举
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum ComparablePart {
    Number(i32),
    String(String),
}

/// JSON 序列化结构
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IndexJson {
    pub entries: Vec<serde_json::Value>,
    pub types: Vec<serde_json::Value>,
}

impl Default for EntryIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::models::Entry;

    #[test]
    fn test_new() {
        let index = EntryIndex::new();
        assert!(index.is_empty());
        assert_eq!(index.length(), 0);
    }

    #[test]
    fn test_add_entry() {
        let mut index = EntryIndex::new();
        let entry = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        index.add(entry);
        assert_eq!(index.length(), 1);
        assert!(!index.is_empty());
    }

    #[test]
    fn test_add_root_entry() {
        let mut index = EntryIndex::new();
        let root_entry = Entry::new(None, Some("index".to_string()), None).unwrap();

        index.add(root_entry);
        assert_eq!(index.length(), 0); // 根条目不应该被添加
        assert!(index.is_empty());
    }

    #[test]
    fn test_add_duplicate_entry() {
        let mut index = EntryIndex::new();
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

        index.add(entry1);
        index.add(entry2);
        assert_eq!(index.length(), 1); // 重复条目不应该被添加
    }

    #[test]
    fn test_type_counting() {
        let mut index = EntryIndex::new();
        let entry1 = Entry::new(
            Some("test1".to_string()),
            Some("test1/path".to_string()),
            Some("function".to_string()),
        ).unwrap();
        let entry2 = Entry::new(
            Some("test2".to_string()),
            Some("test2/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        index.add(entry1);
        index.add(entry2);

        assert_eq!(index.types.get("function").unwrap().count, 2);
    }

    #[test]
    fn test_as_json() {
        let mut index = EntryIndex::new();
        let entry = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        index.add(entry);
        let json = index.as_json();

        assert_eq!(json.entries.len(), 1);
        assert_eq!(json.types.len(), 1);
    }

    #[test]
    fn test_to_json() {
        let mut index = EntryIndex::new();
        let entry = Entry::new(
            Some("test".to_string()),
            Some("test/path".to_string()),
            Some("function".to_string()),
        ).unwrap();

        index.add(entry);
        let json_str = index.to_json();

        assert!(json_str.contains("entries"));
        assert!(json_str.contains("types"));
    }
}