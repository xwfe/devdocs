use crate::core::error::Result;
use crate::core::filter_base::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;

#[derive(Clone, Default)] // Default for easy construction if no state
pub struct BabelCleanHtmlFilter;

impl BabelCleanHtmlFilter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Filter for BabelCleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        let mut document = Document::from(html);
        document.select("header").remove();
        document.select("aside").remove();
        Ok(document.html().to_string_lossy().into_owned())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}