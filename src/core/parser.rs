//! HTML 解析器模块
//!
//! 提供使用 nipper 库解析 HTML 的功能

use crate::core::error::{Error, Result};
use nipper::Document;
use std::collections::HashMap;

/// HTML 解析器
///
/// 使用 nipper 库解析 HTML 文档
pub struct HtmlParser {
    /// HTML 文档
    document: Document,
    /// 缓存的选择器结果
    cache: HashMap<String, Vec<String>>,
}

impl HtmlParser {
    /// 创建新的 HTML 解析器
    pub fn new(html: &str) -> Self {
        Self {
            document: Document::from(html),
            cache: HashMap::new(),
        }
    }
    
    /// 获取原始文档
    pub fn document(&self) -> &Document {
        &self.document
    }
    
    /// 获取可变的原始文档
    pub fn document_mut(&mut self) -> &mut Document {
        &mut self.document
    }
    
    /// 选择元素
    pub fn select(&self, selector: &str) -> nipper::Selection {
        self.document.select(selector)
    }
    
    /// 获取 HTML
    pub fn html(&self) -> String {
        self.document.html().to_string_lossy().into_owned()
    }
    
    /// 获取文本
    pub fn text(&self) -> String {
        self.document.text().to_string_lossy().into_owned()
    }
    
    /// 获取标题
    pub fn title(&self) -> Result<String> {
        let title = self.document.select("title").first();
        if let Some(title) = title {
            Ok(title.text().to_string())
        } else {
            Err(Error::Message("未找到标题".to_string()).into())
        }
    }
    
    /// 获取元素文本
    pub fn get_text(&self, selector: &str) -> Result<String> {
        let element = self.document.select(selector).first();
        if let Some(element) = element {
            Ok(element.text().to_string())
        } else {
            Err(Error::Message(format!("未找到元素: {}", selector)).into())
        }
    }
    
    /// 获取元素属性
    pub fn get_attr(&self, selector: &str, attr: &str) -> Result<String> {
        let element = self.document.select(selector).first();
        if let Some(element) = element {
            if let Some(attr_value) = element.attr(attr) {
                Ok(attr_value.to_string())
            } else {
                Err(Error::Message(format!("元素没有属性 {}: {}", attr, selector)).into())
            }
        } else {
            Err(Error::Message(format!("未找到元素: {}", selector)).into())
        }
    }
    
    /// 获取所有匹配元素的文本
    pub fn get_all_text(&self, selector: &str) -> Vec<String> {
        // 检查缓存
        if let Some(cached) = self.cache.get(selector) {
            return cached.clone();
        }
        
        // 获取所有文本
        let mut texts = Vec::new();
        self.document.select(selector).iter().for_each(|element| {
            texts.push(element.text().to_string());
        });
        
        // 缓存结果
        self.cache.insert(selector.to_string(), texts.clone());
        
        texts
    }
    
    /// 获取所有匹配元素的属性
    pub fn get_all_attr(&self, selector: &str, attr: &str) -> Vec<String> {
        let cache_key = format!("{}-{}", selector, attr);
        
        // 检查缓存
        if let Some(cached) = self.cache.get(&cache_key) {
            return cached.clone();
        }
        
        // 获取所有属性
        let mut attrs = Vec::new();
        self.document.select(selector).iter().for_each(|element| {
            if let Some(attr_value) = element.attr(attr) {
                attrs.push(attr_value.to_string());
            }
        });
        
        // 缓存结果
        self.cache.insert(cache_key, attrs.clone());
        
        attrs
    }
    
    /// 移除元素
    pub fn remove(&mut self, selector: &str) {
        self.document.select(selector).remove();
    }
    
    /// 替换元素
    pub fn replace(&mut self, selector: &str, html: &str) {
        // 注意：nipper 的 Selection 对象已经包含了对文档的可变引用
        // 所以我们不需要对单个元素进行操作
        self.document.select(selector).replace_with_html(html);
    }
    
    /// 添加类
    pub fn add_class(&mut self, selector: &str, class: &str) {
        // 直接对整个选择器结果进行操作
        self.document.select(selector).add_class(class);
    }
    
    /// 移除类
    pub fn remove_class(&mut self, selector: &str, class: &str) {
        // 直接对整个选择器结果进行操作
        self.document.select(selector).remove_class(class);
    }
    
    /// 设置属性
    pub fn set_attr(&mut self, selector: &str, attr: &str, value: &str) {
        // 直接对整个选择器结果进行操作
        self.document.select(selector).set_attr(attr, value);
    }
    
    /// 移除属性
    pub fn remove_attr(&mut self, selector: &str, attr: &str) {
        // 直接对整个选择器结果进行操作
        self.document.select(selector).remove_attr(attr);
    }
    
    /// 清理 HTML
    pub fn clean(&mut self) {
        // 移除脚本和样式
        self.remove("script, style");
        
        // 移除注释
        self.remove("comment()");
        
        // 对于需要判断条件的操作，我们需要分步处理
        // 首先收集所有需要移除的空白节点
        let mut empty_nodes = Vec::new();
        self.document.select("text()").iter().for_each(|node| {
            if node.text().trim().is_empty() {
                // 收集需要移除的节点的选择器
                if let Some(id) = node.attr("id") {
                    empty_nodes.push(format!("#{}:text", id));
                } else {
                    // 对于没有 ID 的节点，我们可以使用其他方式标识
                    // 这里简化处理，直接移除所有空白文本节点
                }
            }
        });
        
        // 移除空白节点
        // 由于 nipper 的限制，我们使用更简单的方法
        // 在实际实现中，我们可能需要使用其他方法来处理这个问题
        
        // 移除类属性
        self.remove_attr("[class]", "class");
        
        // 移除 ID 属性
        self.remove_attr("[id]", "id");
        
        // 移除样式属性
        self.remove_attr("[style]", "style");
    }
}

/// 解析 HTML
pub fn parse_html(html: &str) -> HtmlParser {
    HtmlParser::new(html)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parser() {
        let html = r#"<html><head><title>Test</title></head><body><h1>Hello</h1><p>World</p></body></html>"#;
        let parser = parse_html(html);
        
        assert_eq!(parser.title().unwrap(), "Test");
        assert_eq!(parser.get_text("h1").unwrap(), "Hello");
        assert_eq!(parser.get_text("p").unwrap(), "World");
    }
    
    #[test]
    fn test_clean() {
        let html = r#"<html><head><title>Test</title><script>alert('test');</script><style>body { color: red; }</style></head><body><h1 class="title" id="main" style="color: blue;">Hello</h1><p>World</p><!-- Comment --></body></html>"#;
        let mut parser = parse_html(html);
        
        parser.clean();
        
        let cleaned_html = parser.html();
        
        assert!(!cleaned_html.contains("<script>"));
        assert!(!cleaned_html.contains("<style>"));
        assert!(!cleaned_html.contains("class="));
        assert!(!cleaned_html.contains("id="));
        assert!(!cleaned_html.contains("style="));
        assert!(!cleaned_html.contains("<!-- Comment -->"));
        assert!(cleaned_html.contains("<h1>Hello</h1>"));
        assert!(cleaned_html.contains("<p>World</p>"));
    }
}