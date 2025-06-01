//! HTML文档条目过滤器
//! 严格按照原版Ruby实现

use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use nipper::Document;
use regex::Regex;
use std::any::Any;

const ADDITIONAL_ENTRIES: &[(&str, &[&str])] = &[
    ("Element/Heading_Elements", &["h1", "h2", "h3", "h4", "h5", "h6"])
];

/// HTML文档条目过滤器
pub struct HtmlEntriesFilter;

impl HtmlEntriesFilter {
    /// 创建新的条目过滤器
    pub fn new() -> Self {
        HtmlEntriesFilter
    }

    fn get_name(&self, doc: &Document, slug: &str) -> String {
        let mut name = slug.replace('_', " ")
            .replace('/', ".")
            .trim()
            .to_string();

        name = name.replace("Element.", "").to_lowercase();
        if name.starts_with("Global attributes.") {
            name = name.replace("Global attributes.", "");
            name.push_str(" (attribute)");
        }
        if let Some(captures) = Regex::new(r"input\.([-\w]+)").unwrap().captures(&name) {
            if let Some(input_type) = captures.get(1) {
                name = format!("input type=\"{}\"", input_type.as_str());
            }
        }
        name
    }

    fn get_type(&self, doc: &Document, slug: &str) -> Option<String> {
        if slug.contains("CORS") || slug.contains("Using") {
            return Some("Miscellaneous".to_string());
        }

        // 使用 nipper 选择器检查是否有过时或非标准元素
        if doc.select(".deprecated, .non-standard, .obsolete").iter().next().is_some() {
            return Some("Obsolete".to_string());
        }

        if slug.starts_with("Global_attr") {
            Some("Attributes".to_string())
        } else if slug.starts_with("Element/") {
            Some("Elements".to_string())
        } else {
            Some("Miscellaneous".to_string())
        }
    }

    fn include_default_entry(&self, slug: &str, doc: &Document) -> bool {
        if slug == "Element/Heading_Elements" {
            return false;
        }

        // 使用 nipper 选择器检查是否有非标准轨道的指示器
        if let Some(node) = doc.select(".overheadIndicator, .blockIndicator").iter().next() {
            let content = node.text().to_string();
            if content.contains("not on a standards track") {
                return false;
            }
        }
        true
    }

    fn additional_entries(&self, doc: &Document, slug: &str) -> Vec<(String, String, String)> {
        // 检查预定义的额外条目
        for (entry_slug, elements) in ADDITIONAL_ENTRIES {
            if *entry_slug == slug {
                return elements.iter()
                    .map(|&tag| (tag.to_string(), tag.to_string(), "Elements".to_string()))
                    .collect();
            }
        }

        if slug == "Attributes" {
            let mut entries = Vec::new();
            // 使用 nipper 选择器查找表格单元格
            for node_selection in doc.select(".standard-table td:first-child").iter() {
                // 获取下一个兄弟元素的文本
                let next_sibling = node_selection.next_sibling();
                // nipper 的 next_sibling() 返回一个新的 Selection
                // 我们可以检查它是否有内容，方法是看是否有节点
                let next_content = if next_sibling.iter().next().is_some() {
                    next_sibling.text().to_string()
                } else {
                    "".to_string()
                };
                
                if next_content.contains("Global attribute") {
                    continue;
                }

                // 尝试从 code 元素获取名称，如果没有则使用节点本身的文本
                let mut name = if let Some(code) = node_selection.select("code").iter().next() {
                    code.text().to_string().trim().to_string()
                } else {
                    node_selection.text().to_string().trim().to_string()
                };
                
                name.push_str(" (attribute)");
                let id = name.to_lowercase().replace(' ', "-");
                entries.push((name, id, "Attributes".to_string()));
            }
            entries
        } else if slug == "Link_types" {
            let mut entries = Vec::new();
            // 使用 nipper 选择器查找 code 元素
            for node_selection in doc.select(".standard-table td:first-child > code").iter() {
                let name = format!("rel: {}", node_selection.text().to_string().trim());
                let id = name.to_lowercase().replace(' ', "-");
                entries.push((name, id, "Attributes".to_string()));
            }
            entries
        } else {
            Vec::new()
        }
    }

    fn build_entry(&self, name: String, fragment: Option<String>, entry_type: Option<String>, context: &FilterContext) -> (String, String, String) {
        let path = if let Some(frag) = fragment {
            if frag.contains('#') {
                frag
            } else {
                format!("{}#{}", context.current_path, frag)
            }
        } else {
            context.current_path.clone()
        };

        (name, path, entry_type.unwrap_or_else(|| "Element".to_string()))
    }

    fn is_api_page(&self, slug: &str) -> bool {
        slug.starts_with("api/")
    }

    fn is_root_page(&self, slug: &str, context: &FilterContext) -> bool {
        let is_root_path_match = match &context.root_path {
            Some(rp) => slug == rp.as_str(),
            None => false,
        };
        slug.is_empty() || slug == "/" || is_root_path_match
    }

    fn css_filter(&self) -> &str {
        ""
    }
}

impl Filter for HtmlEntriesFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(HtmlEntriesFilter::new())
    }

    fn get_entries(&self, html: &str, context: &FilterContext) -> Vec<(String, String, String)> {
        let slug = &context.current_path;
        let is_root = self.is_root_page(slug, context);

        let mut entries = Vec::new();
        let doc = Document::from(html);

        if self.include_default_entry(slug, &doc) {
            let name = self.get_name(&doc, slug);
            if let Some(entry_type) = self.get_type(&doc, slug) {
                entries.push((name, slug.to_string(), entry_type));
            }
        }

        entries.extend(self.additional_entries(&doc, slug));
        entries
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
