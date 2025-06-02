use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;

#[derive(Clone, Default)]
pub struct AngularCleanHtmlFilter;

impl AngularCleanHtmlFilter {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Filter for AngularCleanHtmlFilter {
    fn apply(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        let mut document = Document::from(html);
        document.select("header").remove();
        document.select("footer").remove();
        Ok(document.html().to_string_lossy().into_owned())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}
