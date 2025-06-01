//! 清理 HTML 过滤器
//!
//! 提供基本的 HTML 清理功能

use crate::core::error::Result;
use crate::core::filter_base::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;

/// 清理 HTML 过滤器
///
/// 用于清理 HTML 内容，删除不需要的元素和属性
#[derive(Clone, Default)]
pub struct CleanHtmlFilter;

impl CleanHtmlFilter {
    /// 创建新的清理 HTML 过滤器
    pub fn new() -> Self {
        Self::default()
    }
}

impl Filter for CleanHtmlFilter {
    /// 应用过滤器
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        let mut document = Document::from(html);
        
        // 移除脚本和样式
        document.select("script, style").remove();
        
        // 移除注释
        document.select("comment()").remove();
        
        // 移除空白节点
        document.select("text()").iter().for_each(|node| {
            if node.text().trim().is_empty() {
                node.remove();
            }
        });
        
        // 移除不需要的属性
        document.select("[class], [id], [style]").iter().for_each(|node| {
            node.remove_attr("class");
            node.remove_attr("id");
            node.remove_attr("style");
        });
        
        // 返回清理后的 HTML
        Ok(document.html().to_string_lossy().into_owned())
    }
    
    /// 克隆过滤器
    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_clean_html_filter() {
        let filter = CleanHtmlFilter::new();
        let mut context = FilterContext::new();
        
        let html = r#"
        <html>
            <head>
                <script>alert('test');</script>
                <style>body { color: red; }</style>
            </head>
            <body>
                <div id="main" class="container" style="color: blue;">
                    <p>Hello, world!</p>
                    <!-- This is a comment -->
                </div>
            </body>
        </html>
        "#;
        
        let result = filter.apply(html, &mut context).unwrap();
        
        // 检查结果不包含脚本和样式
        assert!(!result.contains("<script>"));
        assert!(!result.contains("<style>"));
        
        // 检查结果不包含注释
        assert!(!result.contains("<!-- This is a comment -->"));
        
        // 检查结果不包含 id、class 和 style 属性
        assert!(!result.contains("id="));
        assert!(!result.contains("class="));
        assert!(!result.contains("style="));
        
        // 检查结果包含内容
        assert!(result.contains("<p>Hello, world!</p>"));
    }
}
