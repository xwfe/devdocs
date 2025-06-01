use crate::core::error::Result;
use crate::core::models::Entry;
use crate::core::filter_base::FilterContext;
use nipper::Document;
use lazy_static::lazy_static;
use std::any::Any;
use std::collections::HashMap;

// 定义 EntryDefinitionProvider trait
pub trait EntryDefinitionProvider: Send + Sync {
    fn get_entry_name(&self, document: &Document, context: &FilterContext) -> Option<String>;
    fn get_entry_type(&self, name: &str, context: &FilterContext) -> Option<String>;
    fn box_clone(&self) -> Box<dyn EntryDefinitionProvider>;
}

lazy_static! {
    static ref ENTRIES: HashMap<&'static str, Vec<&'static str>> = {
        let mut map = HashMap::new();
        map.insert(
            "Usage",
            vec![
                "Options", "Plugins", "Config Files", "Compiler assumptions",
                "@babel/cli", "@babel/polyfill", "@babel/plugin-transform-runtime",
                "@babel/register",
            ],
        );
        map.insert("Presets", vec!["@babel/preset"]);
        map.insert(
            "Tooling",
            vec![
                "@babel/parser", "@babel/core", "@babel/generator", "@babel/code-frame",
                "@babel/helper", "@babel/runtime", "@babel/template", "@babel/traverse",
                "@babel/types", "@babel/standalone",
            ],
        );
        map
    };
}

#[derive(Clone)]
pub struct BabelEntriesFilter;

impl BabelEntriesFilter {
    pub fn new() -> Self {
        Self
    }

    fn get_name_impl(&self, document: &Document) -> String {
        document.select("h1").text().to_string()
    }

    fn get_type_impl(&self, name: &str, context: &FilterContext) -> String {
        for (key, value_list) in ENTRIES.iter() {
            if value_list.iter().any(|val| name.starts_with(val)) {
                return key.to_string();
            }
        }
        if context.current_path.contains("babel-plugin") {
            return "Other Plugins".to_string();
        }
        "Unknown".to_string()
    }
}

impl EntryDefinitionProvider for BabelEntriesFilter {
    fn get_entry_name(&self, document: &Document, _context: &FilterContext) -> Option<String> {
        let name = self.get_name_impl(document);
        if name.is_empty() {
            None
        } else {
            Some(name)
        }
    }

    fn get_entry_type(&self, name: &str, context: &FilterContext) -> Option<String> {
        let type_str = self.get_type_impl(name, context);
        if type_str == "Unknown" {
            None
        } else {
            Some(type_str)
        }
    }

    fn box_clone(&self) -> Box<dyn EntryDefinitionProvider> {
        Box::new(self.clone())
    }
}
