//! 内部 URL 过滤器
//!
//! 提取和处理 HTML 中的内部链接

use crate::core::error::Result;
use crate::core::filter_base::{Filter, FilterContext};
use scraper::{Html, Selector};
use std::any::Any;
use std::collections::HashSet;

/// 内部 URL 过滤器
///
/// 提取和处理 HTML 中的内部链接
#[derive(Debug, Clone)]
pub struct InternalUrlsFilter;

impl Filter for InternalUrlsFilter {
    fn call(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        // 解析 HTML
        let document = Html::parse_document(html);
        let mut internal_urls = HashSet::new();
        
        // 提取内部链接
        if let Ok(selector) = Selector::parse("a[href]") {
            for element in document.select(&selector) {
                if let Some(href) = element.value().attr("href") {
                    // 如果是相对 URL 或者以基础 URL 开头，则是内部链接
                    if self.is_relative_url(href) || (href.starts_with(&context.base_url) && !href.starts_with("javascript:")) {
                        // 处理相对 URL
                        let url = if self.is_relative_url(href) {
                            if href.starts_with('/') {
                                format!("{}{}", context.base_url.trim_end_matches('/'), href)
                            } else {
                                format!("{}/{}", context.base_url.trim_end_matches('/'), href)
                            }
                        } else {
                            href.to_string()
                        };
                        
                        // 添加到内部链接集合
                        internal_urls.insert(url);
                    }
                }
            }
        }
        
        // 在实际实现中，我们需要将内部链接添加到上下文中
        // 这里只是一个示例
        
        // 返回原始 HTML
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
