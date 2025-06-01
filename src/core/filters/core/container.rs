//! 容器过滤器
//!
//! 提取 HTML 中的主要内容容器

use crate::core::error::Result;
use crate::core::filter_base::{Filter, FilterContext};
use scraper::{Html, Selector};
use std::any::Any;

/// 容器过滤器
///
/// 提取 HTML 中的主要内容容器
#[derive(Debug, Clone)]
pub struct ContainerFilter {
    /// 容器选择器
    selector: String,
}

impl ContainerFilter {
    /// 创建新的容器过滤器
    pub fn new(selector: &str) -> Self {
        Self {
            selector: selector.to_string(),
        }
    }
}

impl Default for ContainerFilter {
    fn default() -> Self {
        Self {
            selector: "body".to_string(),
        }
    }
}

impl Filter for ContainerFilter {
    fn call(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // 解析 HTML
        let document = Html::parse_document(html);
        
        // 提取容器
        if let Ok(selector) = Selector::parse(&self.selector) {
            if let Some(container) = document.select(&selector).next() {
                // 在实际实现中，我们需要返回容器的 HTML
                // 由于 scraper 库的限制，这里只是一个示例
                let container_html = container.html();
                return Ok(container_html);
            }
        }
        
        // 如果没有找到容器，返回原始 HTML
        Ok(html.to_string())
    }
    
    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
