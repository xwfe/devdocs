//! 应用基础 URL 过滤器
//!
//! 提供将相对 URL 转换为绝对 URL 的功能

use crate::core::error::{Error, Result};
use crate::core::filter_base::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;
use url::Url;

/// 应用基础 URL 过滤器
///
/// 用于将 HTML 中的相对 URL 转换为绝对 URL
#[derive(Clone, Default)]
pub struct ApplyBaseUrlFilter;

impl ApplyBaseUrlFilter {
    /// 创建新的应用基础 URL 过滤器
    pub fn new() -> Self {
        Self::default()
    }
    
    /// 检查 URL 是否为相对 URL
    fn is_relative_url(&self, url: &str) -> bool {
        !url.contains(':') && !url.starts_with('#') && !url.starts_with("data:")
    }
    
    /// 将相对 URL 转换为绝对 URL
    fn to_absolute_url(&self, url: &str, base_url: &str) -> Result<String> {
        if self.is_relative_url(url) {
            match Url::parse(base_url) {
                Ok(base) => {
                    match base.join(url) {
                        Ok(absolute) => Ok(absolute.to_string()),
                        Err(e) => Err(Error::Message(format!("无法将 URL 转换为绝对 URL: {}", e)).into())
                    }
                },
                Err(e) => Err(Error::Message(format!("无效的基础 URL: {}", e)).into())
            }
        } else {
            Ok(url.to_string())
        }
    }
}

impl Filter for ApplyBaseUrlFilter {
    /// 应用过滤器
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        // 获取基础 URL
        let base_url = if let Some(base_url) = context.base_url() {
            base_url
        } else {
            return Err(Error::Message("未设置基础 URL".to_string()).into());
        };
        
        let mut document = Document::from(html);
        
        // 处理 <a> 标签的 href 属性
        document.select("a[href]").iter().for_each(|node| {
            if let Some(href) = node.attr("href") {
                if let Ok(absolute_url) = self.to_absolute_url(&href, base_url) {
                    node.set_attr("href", &absolute_url);
                }
            }
        });
        
        // 处理 <img> 标签的 src 属性
        document.select("img[src]").iter().for_each(|node| {
            if let Some(src) = node.attr("src") {
                if let Ok(absolute_url) = self.to_absolute_url(&src, base_url) {
                    node.set_attr("src", &absolute_url);
                }
            }
        });
        
        // 处理 <link> 标签的 href 属性
        document.select("link[href]").iter().for_each(|node| {
            if let Some(href) = node.attr("href") {
                if let Ok(absolute_url) = self.to_absolute_url(&href, base_url) {
                    node.set_attr("href", &absolute_url);
                }
            }
        });
        
        // 处理 <script> 标签的 src 属性
        document.select("script[src]").iter().for_each(|node| {
            if let Some(src) = node.attr("src") {
                if let Ok(absolute_url) = self.to_absolute_url(&src, base_url) {
                    node.set_attr("src", &absolute_url);
                }
            }
        });
        
        // 返回处理后的 HTML
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
    fn test_apply_base_url_filter() {
        let filter = ApplyBaseUrlFilter::new();
        let mut context = FilterContext::new();
        context.set_base_url("https://example.com/docs/");
        
        let html = r#"
        <html>
            <body>
                <a href="page.html">Link</a>
                <img src="image.png" alt="Image">
                <link rel="stylesheet" href="style.css">
                <script src="script.js"></script>
                <a href="https://other.com/page">External Link</a>
                <a href="#section">Fragment Link</a>
            </body>
        </html>
        "#;
        
        let result = filter.apply(html, &mut context).unwrap();
        
        // 检查相对 URL 已转换为绝对 URL
        assert!(result.contains(r#"href="https://example.com/docs/page.html"#));
        assert!(result.contains(r#"src="https://example.com/docs/image.png"#));
        assert!(result.contains(r#"href="https://example.com/docs/style.css"#));
        assert!(result.contains(r#"src="https://example.com/docs/script.js"#));
        
        // 检查绝对 URL 和片段 URL 保持不变
        assert!(result.contains(r#"href="https://other.com/page"#));
        assert!(result.contains(r#"href="#section"#));
    }
}
