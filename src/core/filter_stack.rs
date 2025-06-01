//! FilterStack 模块
//!
//! 严格对齐原版 Ruby 项目中的 core/filter_stack.rb 实现

use std::fmt;

/// 过滤器栈
/// 
/// 对应原版 Ruby 的 FilterStack 类
#[derive(Debug, Clone, PartialEq)]
pub struct FilterStack {
    filters: Vec<String>,
}

impl FilterStack {
    /// 创建新的过滤器栈
    /// 
    /// 对应原版 Ruby 的 initialize 方法
    pub fn new(filters: Option<Vec<String>>) -> Self {
        Self {
            filters: filters.unwrap_or_default(),
        }
    }

    /// 获取过滤器数量
    /// 
    /// 对应原版 Ruby 的 length 方法
    pub fn length(&self) -> usize {
        self.filters.len()
    }

    /// 获取过滤器列表的引用
    pub fn filters(&self) -> &Vec<String> {
        &self.filters
    }

    /// 推入过滤器
    /// 
    /// 对应原版 Ruby 的 push 方法
    pub fn push(&mut self, names: &[&str]) {
        for name in names {
            self.filters.push(self.filter_const(name));
        }
    }

    /// 在指定位置插入过滤器
    /// 
    /// 对应原版 Ruby 的 insert 方法
    pub fn insert(&mut self, index: FilterIndex, names: &[&str]) -> Result<(), String> {
        let idx = self.assert_index(index)?;
        for (i, name) in names.iter().enumerate() {
            self.filters.insert(idx + i, self.filter_const(name));
        }
        Ok(())
    }

    /// 在指定位置之前插入过滤器
    /// 
    /// 对应原版 Ruby 的 insert_before 方法
    pub fn insert_before(&mut self, index: FilterIndex, names: &[&str]) -> Result<(), String> {
        self.insert(index, names)
    }

    /// 在指定位置之后插入过滤器
    /// 
    /// 对应原版 Ruby 的 insert_after 方法
    pub fn insert_after(&mut self, index: FilterIndex, names: &[&str]) -> Result<(), String> {
        let idx = self.assert_index(index)?;
        let after_idx = FilterIndex::Position(idx + 1);
        self.insert(after_idx, names)
    }

    /// 替换指定位置的过滤器
    /// 
    /// 对应原版 Ruby 的 replace 方法
    pub fn replace(&mut self, index: FilterIndex, name: &str) -> Result<(), String> {
        let idx = self.assert_index(index)?;
        self.filters[idx] = self.filter_const(name);
        Ok(())
    }

    /// 转换为数组
    /// 
    /// 对应原版 Ruby 的 to_a 方法
    pub fn to_a(&self) -> Vec<String> {
        self.filters.clone()
    }

    /// 创建可继承的副本
    /// 
    /// 对应原版 Ruby 的 inheritable_copy 方法
    pub fn inheritable_copy(&self) -> Self {
        Self::new(Some(self.filters.clone()))
    }

    /// 过滤器常量转换
    /// 
    /// 对应原版 Ruby 的 filter_const 方法
    fn filter_const(&self, name: &str) -> String {
        format!("{}Filter", self.camelize(&format!("{}_filter", name)))
    }

    /// 驼峰命名转换
    fn camelize(&self, s: &str) -> String {
        s.split('_')
            .map(|word| {
                let mut chars = word.chars();
                match chars.next() {
                    None => String::new(),
                    Some(first) => first.to_uppercase().collect::<String>() + &chars.as_str().to_lowercase(),
                }
            })
            .collect()
    }

    /// 验证索引
    /// 
    /// 对应原版 Ruby 的 assert_index 方法
    fn assert_index(&self, index: FilterIndex) -> Result<usize, String> {
        match index {
            FilterIndex::Position(i) => {
                if i <= self.filters.len() {
                    Ok(i)
                } else {
                    Err(format!("Index {} out of bounds", i))
                }
            }
            FilterIndex::Name(name) => {
                let filter_name = self.filter_const(name);
                self.filters
                    .iter()
                    .position(|f| f == &filter_name)
                    .ok_or_else(|| format!("No such filter to insert: {}", name))
            }
        }
    }
}

/// 过滤器索引枚举
/// 
/// 支持按位置或名称索引
#[derive(Debug, Clone)]
pub enum FilterIndex {
    Position(usize),
    Name(&'static str),
}

impl fmt::Display for FilterStack {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.filters)
    }
}

impl Default for FilterStack {
    fn default() -> Self {
        Self::new(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let stack = FilterStack::new(None);
        assert_eq!(stack.length(), 0);

        let stack = FilterStack::new(Some(vec!["TestFilter".to_string()]));
        assert_eq!(stack.length(), 1);
    }

    #[test]
    fn test_push() {
        let mut stack = FilterStack::new(None);
        stack.push(&["clean_html", "normalize_urls"]);
        
        assert_eq!(stack.length(), 2);
        assert!(stack.filters().contains(&"CleanHtmlFilter".to_string()));
        assert!(stack.filters().contains(&"NormalizeUrlsFilter".to_string()));
    }

    #[test]
    fn test_insert() {
        let mut stack = FilterStack::new(None);
        stack.push(&["clean_html"]);
        
        stack.insert(FilterIndex::Position(0), &["normalize_urls"]).unwrap();
        assert_eq!(stack.filters()[0], "NormalizeUrlsFilter");
        assert_eq!(stack.filters()[1], "CleanHtmlFilter");
    }

    #[test]
    fn test_insert_by_name() {
        let mut stack = FilterStack::new(None);
        stack.push(&["clean_html"]);
        
        stack.insert(FilterIndex::Name("clean_html"), &["normalize_urls"]).unwrap();
        assert_eq!(stack.filters()[0], "NormalizeUrlsFilter");
        assert_eq!(stack.filters()[1], "CleanHtmlFilter");
    }

    #[test]
    fn test_insert_after() {
        let mut stack = FilterStack::new(None);
        stack.push(&["clean_html"]);
        
        stack.insert_after(FilterIndex::Position(0), &["normalize_urls"]).unwrap();
        assert_eq!(stack.filters()[0], "CleanHtmlFilter");
        assert_eq!(stack.filters()[1], "NormalizeUrlsFilter");
    }

    #[test]
    fn test_replace() {
        let mut stack = FilterStack::new(None);
        stack.push(&["clean_html"]);
        
        stack.replace(FilterIndex::Position(0), "normalize_urls").unwrap();
        assert_eq!(stack.filters()[0], "NormalizeUrlsFilter");
        assert_eq!(stack.length(), 1);
    }

    #[test]
    fn test_to_a() {
        let mut stack = FilterStack::new(None);
        stack.push(&["clean_html", "normalize_urls"]);
        
        let array = stack.to_a();
        assert_eq!(array.len(), 2);
        assert!(array.contains(&"CleanHtmlFilter".to_string()));
        assert!(array.contains(&"NormalizeUrlsFilter".to_string()));
    }

    #[test]
    fn test_inheritable_copy() {
        let mut original = FilterStack::new(None);
        original.push(&["clean_html"]);
        
        let copy = original.inheritable_copy();
        assert_eq!(original.filters(), copy.filters());
        
        // 修改原始不应该影响副本
        // 由于是不可变操作，这里主要测试结构相等性
        assert_eq!(original, copy);
    }

    #[test]
    fn test_equality() {
        let mut stack1 = FilterStack::new(None);
        stack1.push(&["clean_html"]);
        
        let mut stack2 = FilterStack::new(None);
        stack2.push(&["clean_html"]);
        
        assert_eq!(stack1, stack2);
        
        stack2.push(&["normalize_urls"]);
        assert_ne!(stack1, stack2);
    }

    #[test]
    fn test_filter_const() {
        let stack = FilterStack::new(None);
        assert_eq!(stack.filter_const("clean_html"), "CleanHtmlFilterFilter");
        assert_eq!(stack.filter_const("normalize_urls"), "NormalizeUrlsFilterFilter");
    }

    #[test]
    fn test_assert_index_errors() {
        let stack = FilterStack::new(None);
        
        // 超出范围的位置索引
        let result = stack.assert_index(FilterIndex::Position(10));
        assert!(result.is_err());
        
        // 不存在的过滤器名称
        let result = stack.assert_index(FilterIndex::Name("nonexistent"));
        assert!(result.is_err());
    }
}