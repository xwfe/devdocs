//! 容器过滤器
//!
//! 提供提取 HTML 容器的功能

use crate::core::error::{Error, Result};
use crate::core::filter_base::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;

/// 容器过滤器
///
/// 用于提取 HTML 中的主要内容容器
#[derive(Clone, Default)]
pub struct ContainerFilter {
    /// 容器选择器
    container_selector: Option<String>,
}

impl ContainerFilter {
    /// 创建新的容器过滤器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 设置容器选择器
    pub fn with_selector(mut self, selector: &str) -> Self {
        self.container_selector = Some(selector.to_string());
        self
    }
}

impl Filter for ContainerFilter {
    /// 应用过滤器
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        let document = Document::from(html);
        
        // 获取容器选择器
        let selector = if let Some(selector) = &self.container_selector {
            selector
        } else if let Some(selector) = context.get_data("container_selector") {
            selector
        } else {
            // 默认选择器
            "main, .main, #main, .content, #content, .container, #container, article, .documentation, .doc-content"
        };
        
        // 查找容器
        let container = document.select(selector);
        
        // 如果找到容器，返回容器内容
        if !container.is_empty() {
            // 获取第一个匹配的容器
            let first_container = container.first();
            return Ok(first_container.html().to_string_lossy().into_owned());
        }
        
        // 如果没有找到容器，返回原始 HTML
        Ok(html.to_string())
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
    fn test_container_filter() {
        let filter = ContainerFilter::new().with_selector(".content");
        let mut context = FilterContext::new();
        
        let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <header>Header</header>
                <div class="content">
                    <p>This is the main content</p>
                </div>
                <footer>Footer</footer>
            </body>
        </html>
        "#;
        
        let result = filter.apply(html, &mut context).unwrap();
        
        // 检查结果包含内容
        assert!(result.contains("<p>This is the main content</p>"));
        
        // 检查结果不包含标题和页脚
        assert!(!result.contains("<header>Header</header>"));
        assert!(!result.contains("<footer>Footer</footer>"));
    }
    
    #[test]
    fn test_container_filter_with_context() {
        let filter = ContainerFilter::new();
        let mut context = FilterContext::new();
        context.set_data("container_selector", ".custom-container");
        
        let html = r#"
        <html>
            <head>
                <title>Test</title>
            </head>
            <body>
                <header>Header</header>
                <div class="custom-container">
                    <p>This is the custom content</p>
                </div>
                <footer>Footer</footer>
            </body>
        </html>
        "#;
        
        let result = filter.apply(html, &mut context).unwrap();
        
        // 检查结果包含内容
        assert!(result.contains("<p>This is the custom content</p>"));
        
        // 检查结果不包含标题和页脚
        assert!(!result.contains("<header>Header</header>"));
        assert!(!result.contains("<footer>Footer</footer>"));
    }
}
