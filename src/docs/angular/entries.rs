use crate::core::scraper::filter::{EntryDefinitionProvider, FilterContext};
use crate::core::error::Result;
use crate::core::scraper::filter::Filter;
use nipper::Document;
use std::any::Any;

#[derive(Clone, Default)]
pub struct AngularEntriesFilter;

impl AngularEntriesFilter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl EntryDefinitionProvider for AngularEntriesFilter {
    fn get_entry_name(&self, document: &Document, _context: &FilterContext) -> Option<String> {
        let title = document.select("h1").text();
        if title.is_empty() { None } else { Some(title) }
    }

    fn get_entry_type(&self, name: &str, context: &FilterContext) -> Option<String> {
        if context.current_path.contains("/api/") {
            Some("API".to_string())
        } else if context.current_path.contains("/guide/") || context.current_path.contains("/tutorial") {
            Some("Guide".to_string())
        } else {
            Some("Page".to_string())
        }
    }

    fn box_clone(&self) -> Box<dyn EntryDefinitionProvider> {
        Box::new(self.clone())
    }
}

impl Filter for AngularEntriesFilter {
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        let document = Document::from(html);
        if let Some(name) = self.get_entry_name(&document, context) {
            let type_str = self.get_entry_type(&name, context).unwrap_or_default();
            let entry = crate::core::models::Entry::new(
                Some(name),
                Some(context.current_path.clone()),
                Some(type_str),
            )?;
            context.entries.push(entry);
        }
        Ok(html.to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any { self }

    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
