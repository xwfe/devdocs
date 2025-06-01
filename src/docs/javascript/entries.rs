//! JavaScript 条目过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;

/// JavaScript条目过滤器
pub struct JavaScriptEntriesFilter {
    /// 输出路径前缀
    path_prefix: String,
}

impl JavaScriptEntriesFilter {
    /// 创建新的过滤器
    pub fn new() -> Self {
        Self {
            path_prefix: "/en-US/docs/Web/JavaScript/".to_string(),
        }
    }

    /// 创建带有路径前缀的过滤器
    pub fn with_path_prefix(path_prefix: String) -> Self {
        Self { path_prefix }
    }

    /// 获取条目名称
    fn get_name(&self, doc: &Document, slug: &str) -> String {
        let title_text_tendril = doc.select("h1").first().text();
        if !title_text_tendril.is_empty() {
            return title_text_tendril.to_string().trim().to_string();
        }
        slug.replace('_', " ").trim().to_string()
    }

    /// 获取条目类型
    fn get_type(&self, doc: &Document) -> String {
        let breadcrumb_text_tendril = doc.select(".breadcrumbs-container").first().text();
        if !breadcrumb_text_tendril.is_empty() {
            let text = breadcrumb_text_tendril.to_string();
            if text.contains("Statements") {
                return "Statements".to_string();
            } else if text.contains("Operators") {
                return "Operators".to_string();
            } else if text.contains("Functions") {
                return "Functions".to_string();
            } else if text.contains("Global Objects") || text.contains("Classes") {
                return "Objects".to_string();
            }
        }
        "Others".to_string()
    }
}

impl Filter for JavaScriptEntriesFilter {
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(Self::new())
    }

    fn get_entries(&self, html: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let doc = Document::from(html);
        let name = self.get_name(&doc, &context.current_path);
        let entry_type = self.get_type(&doc);

        vec![(name, context.current_path.clone(), entry_type)]
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
