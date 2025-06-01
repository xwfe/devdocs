//! JavaScript HTML清理过滤器
//! 严格按照原版Ruby实现

use nipper::Document;
use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use std::any::Any;

/// JavaScript HTML清理过滤器
#[derive(Default, Clone)]
pub struct JavaScriptCleanHtmlFilter;

impl JavaScriptCleanHtmlFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self
    }
}

impl Filter for JavaScriptCleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        let mut document = Document::from(html);

        self.clean_html_content(&mut document)?;

        Ok(document.html().to_string())
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

impl JavaScriptCleanHtmlFilter {
    fn clean_html_content(&self, document: &mut Document) -> Result<()> {
        // 选择器1: 移除包装器元素，保留其内容
        let selector1 = "section, div.section, div.row, div.notice, div.deprecated, div.obsolete";
        document.select(selector1).iter().for_each(|mut selection| {
            let mut inner_html_content = String::new();
            selection.children().iter().for_each(|child_selection| {
                // html() 方法直接返回 Tendril<UTF8>，可以直接转为字符串
                let html_content = child_selection.html().to_string();
                inner_html_content.push_str(&html_content);
            });
            if !inner_html_content.is_empty() { 
                selection.replace_with_html(inner_html_content);
            } else {
                selection.remove(); 
            }
        });

        // 选择器2: 移除包含特定 href 的链接
        let selector2 = "a[href*='additional_examples']";
        document.select(selector2).remove();

        // 更多来自原始 Ruby Filter 的清理逻辑可以逐步添加:
        // document.select("dt > tt > a[name], dt > code > a[name]").remove();
        // document.select("dt > a[name]").each(|node| node.parent().unwrap().replace_with(node.inner_html()));
        // document.select("p > code:only-child").each(|node| node.parent().unwrap().replace_with(node.inner_html()));
        // document.select("tt, var").each(|node| node.replace_with(node.inner_html()));
        // document.select("span.skip").remove();

        // 移除所有元素的 style 属性 (如果 FilterBase 提供了此功能，则可以调用)
        // self.remove_attributes(document, &[("*", "style")]);

        // 移除空属性 (nipper 可能没有直接的方法，需要手动迭代和检查)

        Ok(())
    }
}
