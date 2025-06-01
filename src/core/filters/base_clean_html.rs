use crate::core::error::Result;
use crate::core::scraper::filter::{Filter, FilterContext};
use nipper::Document;
use std::any::Any;
use url::Url;

pub trait FilterBase {
    fn normalize_href(&self, href: &str, context: &FilterContext) -> String {
        if href.starts_with('#') || href.starts_with("mailto:") || href.starts_with("tel:") || href.starts_with("javascript:") {
            return href.to_string();
        }

        let current_as_url = Url::parse(&context.current_url);
        if let Ok(base_url_for_href) = current_as_url {
            if let Ok(parsed_href) = base_url_for_href.join(href) {
                if parsed_href.as_str().starts_with(&context.root_url) {
                    let relative_path = parsed_href.as_str().trim_start_matches(&context.root_url);
                    return relative_path.trim_start_matches('/').to_string();
                }
                return parsed_href.as_ref().to_string();
            }
        }
        href.trim_start_matches('/').to_string()
    }

    fn normalize_hrefs(&self, document: &mut Document, base_url: &Url, selector_str: &str, attr_name: &str) {
        document.select(selector_str).iter().for_each(|mut el_sel| {
            if let Some(attr_val_cow) = el_sel.attr(attr_name) {
                let attr_val = attr_val_cow.to_string();
                if attr_val.starts_with("//") {
                    let new_url_str = format!("{}:{}", base_url.scheme(), attr_val);
                    el_sel.set_attr(attr_name, new_url_str.as_str());
                } else if attr_val.starts_with("/") {
                    if let Ok(joined_url) = base_url.join(&attr_val) {
                        el_sel.set_attr(attr_name, joined_url.as_str());
                    }
                } else if !attr_val.starts_with("http") && !attr_val.starts_with("#") && !attr_val.starts_with("mailto:") && !attr_val.is_empty() {
                    if let Ok(joined_url) = base_url.join(&attr_val) {
                        el_sel.set_attr(attr_name, joined_url.as_str());
                    }
                }
            }
        });
    }

    fn remove_elements(&self, document: &mut Document, selectors: &[&str]) {
        for s in selectors {
            document.select(s).remove();
        }
    }

    fn remove_attributes(&self, document: &mut Document, selector_attrs: &[(&str, &str)]) {
        for (selector, attr_name) in selector_attrs {
            document.select(selector).iter().for_each(|mut el_sel| {
                el_sel.remove_attr(attr_name);
            });
        }
    }
}

pub struct DefaultCleanHtmlFilter;

impl DefaultCleanHtmlFilter {
    pub fn name(&self) -> &str {
        "default_clean_html"
    }

    fn remove_elements_custom(&self, document: &mut Document, tags_to_remove: &[&str]) {
        self.remove_elements(document, tags_to_remove);
    }

    fn normalize_links_custom(&self, document: &mut Document, context: &FilterContext) {
        match Url::parse(&context.current_url) {
            Ok(base_url) => {
                self.normalize_hrefs(document, &base_url, "a", "href");
                self.normalize_hrefs(document, &base_url, "link[rel=canonical]", "href");
            }
            Err(e) => {
                // Log or handle the error appropriately
                // For now, just print a warning or skip normalization for this context
                eprintln!("Warning: Could not parse current_url '{}': {}. Skipping link normalization.", context.current_url, e);
            }
        }
    }
}

impl Filter for DefaultCleanHtmlFilter {
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        let mut document = Document::from(html);

        let tags_to_remove = [
            "script", "style", "iframe", "noscript", "applet", "embed", "object",
        ];
        self.remove_elements_custom(&mut document, &tags_to_remove);
        self.normalize_links_custom(&mut document, context);

        Ok(document.html().to_string())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(DefaultCleanHtmlFilter) 
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any { 
        self
    }
}

impl FilterBase for DefaultCleanHtmlFilter {}
