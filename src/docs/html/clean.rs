//! HTML文档清理过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;

/// 清理HTML的过滤器，移除不必要的元素和属性
pub struct CleanHtmlFilter;

impl CleanHtmlFilter {
    /// 创建新的清理过滤器
    pub fn new() -> Self {
        Self
    }
}

impl Filter for CleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // 使用 nipper 解析 HTML
        let document = Document::from(html);
        let mut output = String::new();

        // 使用 nipper 选择器查找和处理节点
        let selector = "section, div.section, div.row";
        let selections = document.select(selector);
        
        // 如果没有找到匹配的元素，尝试使用 body 选择器
        // nipper::Selection 没有 is_empty() 方法，我们使用 iter().next().is_none() 来检查
        let selections = if selections.iter().next().is_none() {
            document.select("body")
        } else {
            selections
        };

        for node_selection in selections.iter() {
            // 获取节点的外部 HTML
            let outer_html = node_selection.html().to_string();
            
            // 获取节点的内部 HTML
            let mut inner_html = String::new();
            for child in node_selection.children().iter() {
                inner_html.push_str(&child.html().to_string());
            }
            
            // 替换外部 HTML 为内部 HTML
            output = if output.is_empty() {
                html.replace(&outer_html, &inner_html)
            } else {
                output.replace(&outer_html, &inner_html)
            };
        }

        if output.is_empty() {
            output = html.to_string();
        }
        Ok(output)
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self::new())
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
