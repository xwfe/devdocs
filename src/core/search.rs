//! 文档搜索引擎模块
//!
//! 提供搜索文档的功能

use crate::core::error::Result;
use crate::core::indexer::{Entry, Index};
use std::collections::HashMap;

/// 搜索结果项
#[derive(Debug, Clone)]
pub struct SearchResult {
    /// 条目
    pub entry: Entry,
    /// 相关度分数
    pub score: f64,
    /// 匹配的文本
    pub matched_text: String,
}

/// 搜索引擎
pub struct SearchEngine {
    /// 文档索引
    index: Index,
    /// 倒排索引
    inverted_index: HashMap<String, Vec<usize>>,
}

impl SearchEngine {
    /// 创建新的搜索引擎
    pub fn new(index: Index) -> Self {
        let mut engine = Self {
            index,
            inverted_index: HashMap::new(),
        };
        
        // 构建倒排索引
        engine.build_inverted_index();
        
        engine
    }
    
    /// 构建倒排索引
    fn build_inverted_index(&mut self) {
        for (i, entry) in self.index.entries.iter().enumerate() {
            // 对名称进行分词
            let tokens = self.tokenize(&entry.name);
            
            // 添加到倒排索引
            for token in tokens {
                self.inverted_index.entry(token).or_insert_with(Vec::new).push(i);
            }
        }
    }
    
    /// 分词
    fn tokenize(&self, text: &str) -> Vec<String> {
        let text = text.to_lowercase();
        
        // 分词
        let mut tokens = Vec::new();
        let mut token = String::new();
        
        for c in text.chars() {
            if c.is_alphanumeric() {
                token.push(c);
            } else if !token.is_empty() {
                tokens.push(token.clone());
                token.clear();
            }
        }
        
        if !token.is_empty() {
            tokens.push(token);
        }
        
        tokens
    }
    
    /// 搜索
    pub fn search(&self, query: &str, limit: usize) -> Result<Vec<SearchResult>> {
        // 对查询进行分词
        let tokens = self.tokenize(query);
        
        // 如果没有有效的查询词，返回空结果
        if tokens.is_empty() {
            return Ok(Vec::new());
        }
        
        // 计算每个条目的分数
        let mut scores = HashMap::new();
        
        for token in &tokens {
            if let Some(entry_indices) = self.inverted_index.get(token) {
                for &index in entry_indices {
                    let entry = &self.index.entries[index];
                    let score = self.calculate_score(entry, token);
                    *scores.entry(index).or_insert(0.0) += score;
                }
            }
        }
        
        // 将分数转换为结果
        let mut results = Vec::new();
        
        for (index, score) in scores {
            let entry = self.index.entries[index].clone();
            let matched_text = self.get_matched_text(&entry, &tokens);
            
            results.push(SearchResult {
                entry,
                score,
                matched_text,
            });
        }
        
        // 按分数排序
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        
        // 限制结果数量
        if results.len() > limit {
            results.truncate(limit);
        }
        
        Ok(results)
    }
    
    /// 计算分数
    fn calculate_score(&self, entry: &Entry, token: &str) -> f64 {
        let name = entry.name.to_lowercase();
        
        // 如果名称包含完整的查询词，给予更高的分数
        if name.contains(token) {
            // 如果名称以查询词开头，给予最高分数
            if name.starts_with(token) {
                return 2.0;
            }
            
            // 如果名称中的单词以查询词开头，给予较高分数
            let words: Vec<&str> = name.split_whitespace().collect();
            for word in words {
                if word.starts_with(token) {
                    return 1.5;
                }
            }
            
            // 如果名称包含查询词，给予中等分数
            return 1.0;
        }
        
        // 如果名称不包含查询词，给予较低分数
        0.5
    }
    
    /// 获取匹配的文本
    fn get_matched_text(&self, entry: &Entry, tokens: &[String]) -> String {
        let name = entry.name.to_lowercase();
        
        // 查找最长的匹配词
        let mut longest_match = String::new();
        
        for token in tokens {
            if name.contains(token) && token.len() > longest_match.len() {
                longest_match = token.clone();
            }
        }
        
        // 如果没有匹配，返回名称
        if longest_match.is_empty() {
            return entry.name.clone();
        }
        
        // 查找匹配词在名称中的位置
        if let Some(pos) = name.find(&longest_match) {
            // 提取匹配词前后的上下文
            let start = if pos > 10 { pos - 10 } else { 0 };
            let end = if pos + longest_match.len() + 10 < name.len() {
                pos + longest_match.len() + 10
            } else {
                name.len()
            };
            
            // 提取上下文
            let mut context = String::new();
            
            if start > 0 {
                context.push_str("...");
            }
            
            context.push_str(&name[start..pos]);
            context.push_str("<b>");
            context.push_str(&name[pos..pos + longest_match.len()]);
            context.push_str("</b>");
            context.push_str(&name[pos + longest_match.len()..end]);
            
            if end < name.len() {
                context.push_str("...");
            }
            
            return context;
        }
        
        // 如果无法找到匹配词的位置，返回名称
        entry.name.clone()
    }
    
    /// 获取条目
    pub fn get_entry(&self, path: &str) -> Option<&Entry> {
        self.index.get_entry(path)
    }
    
    /// 获取所有条目
    pub fn get_all_entries(&self) -> &[Entry] {
        &self.index.entries
    }
    
    /// 获取条目数量
    pub fn len(&self) -> usize {
        self.index.len()
    }
    
    /// 检查索引是否为空
    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }
}

/// 创建搜索引擎
pub fn create_search_engine(index: Index) -> SearchEngine {
    SearchEngine::new(index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::indexer::Index;
    
    #[test]
    fn test_search() {
        let mut index = Index::new();
        
        index.add_entry("Home", "index.html", "Page", None);
        index.add_entry("Introduction", "index.html#intro", "Section", Some("index.html"));
        index.add_entry("Getting Started", "index.html#getting-started", "Section", Some("index.html"));
        index.add_entry("API Reference", "api.html", "Page", None);
        index.add_entry("Methods", "api.html#methods", "Section", Some("api.html"));
        index.add_entry("Method 1", "api.html#method1", "Method", Some("api.html"));
        index.add_entry("Method 2", "api.html#method2", "Method", Some("api.html"));
        
        let engine = create_search_engine(index);
        
        // 测试搜索
        let results = engine.search("method", 10).unwrap();
        assert_eq!(results.len(), 3);
        assert_eq!(results[0].entry.name, "Methods");
        assert_eq!(results[1].entry.name, "Method 1");
        assert_eq!(results[2].entry.name, "Method 2");
        
        let results = engine.search("intro", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entry.name, "Introduction");
        
        let results = engine.search("get", 10).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].entry.name, "Getting Started");
        
        // 测试空查询
        let results = engine.search("", 10).unwrap();
        assert_eq!(results.len(), 0);
        
        // 测试无匹配
        let results = engine.search("nonexistent", 10).unwrap();
        assert_eq!(results.len(), 0);
    }
}
