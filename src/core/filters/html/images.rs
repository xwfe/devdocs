//! 图片过滤器，用于处理 HTML 中的图片标签

use crate::core::error::Result;
use crate::core::filter_base::{Filter, FilterContext};
use base64::encode;
use nipper::Document;
use reqwest::blocking::Client;
use std::any::Any;
use mime_guess::from_path;

const MAX_IMAGE_SIZE: u64 = 2 * 1024 * 1024; // 2MB

/// 图片过滤器
#[derive(Default, Clone)]
pub struct ImagesFilter {
    client: Client,
}

impl ImagesFilter {
    pub fn new() -> Self {
        ImagesFilter {
            client: Client::new(),
        }
    }

    fn process_images(&self, document: &mut Document, base_url: &str) -> Result<()> {
        let mut images_to_update: Vec<(String, String)> = Vec::new();

        document.select("img").iter().for_each(|img_selection| {
            if let Some(src_attr) = img_selection.attr("src") {
                let src = src_attr.to_string();
                if src.starts_with("data:") || src.is_empty() {
                    return; // Skip data URIs or empty src
                }

                // 处理相对 URL
                let image_url = if self.relative_url_string(&src) {
                    // Ensure base_url ends with a slash if src doesn't start with one, and vice-versa
                    let mut full_url = base_url.trim_end_matches('/').to_string();
                    if !src.starts_with('/') {
                        full_url.push('/');
                    }
                    full_url.push_str(src.trim_start_matches('/'));
                    full_url
                } else {
                    src.clone()
                };

                // 下载或处理图片
                match self.fetch_and_encode(&image_url) {
                    Ok(Some(data_url)) => {
                        // Store for batch update, as modifying selection during iteration is tricky
                        images_to_update.push((src.clone(), data_url));
                    }
                    Ok(None) => {
                        // Image fetch failed or status not success, keep original src or log
                        eprintln!("Failed to fetch or received non-success status for image: {}", image_url);
                    }
                    Err(e) => {
                        // Error during fetch/encode, keep original src or log
                        eprintln!("Error processing image {}: {}", image_url, e);
                    }
                }
            }
        });

        // Batch update image sources
        for (old_src, new_data_url) in images_to_update {
            // Find the image again to update it. This is not ideal for performance.
            // A better way would be to mark elements or use a more direct update mechanism if nipper supports it.
            document.select(&format!("img[src='{}']", old_src)).set_attr("src", &new_data_url);
        }

        Ok(())
    }

    fn fetch_and_encode(&self, img_url: &str) -> Result<Option<String>> {
        let response = self.client.get(img_url).send()?;
        if response.status().is_success() {
            let bytes = response.bytes()?;
            if bytes.len() as u64 > MAX_IMAGE_SIZE {
                return Err(Result::Message(format!("Image {} exceeds maximum size of {}", img_url, MAX_IMAGE_SIZE)));
            }
            let guessed_mime = from_path(img_url).first_or_octet_stream();
            let base64_encoded = encode(&bytes);
            Ok(Some(format!("data:{};base64,{}", guessed_mime.essence_str(), base64_encoded)))
        } else {
            Ok(None)
        }
    }

    // Helper to check if URL is relative (simplified)
    fn relative_url_string(&self, url: &str) -> bool {
        !url.starts_with("http://") && !url.starts_with("https://") && !url.starts_with("//")
    }
}

impl Filter for ImagesFilter {
    fn apply(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        let base_url_from_context = &context.base_url;
        let mut document = Document::from(html);
        self.process_images(&mut document, base_url_from_context)?;
        Ok(document.html().to_string_lossy().into_owned())
    }

    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }

    fn as_any(&self) -> &dyn Any {
        self as &dyn Any
    }
}
