//! Angular 文档抓取器

use crate::core::error::Result;
use crate::core::scraper::base::{Scraper, UrlScraper};
use async_trait::async_trait;
use crate::docs::angular::{AngularCleanHtmlFilter, AngularEntriesFilter};

/// Angular 文档爬虫
pub struct AngularScraper {
    scraper: UrlScraper,
}

impl AngularScraper {
    /// 创建新的 Angular 文档爬虫
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://angular.io/docs";
        let mut scraper = UrlScraper::new("Angular", version, base_url, output_path);

        let initial_paths = vec![
            "/".to_string(),
            "/guide/quickstart".to_string(),
            "/tutorial".to_string(),
            "/api".to_string(),
        ];

        scraper = scraper
            .with_initial_paths(initial_paths)
            .with_filter(Box::new(AngularCleanHtmlFilter::new()))
            .with_filter(Box::new(AngularEntriesFilter::new()));

        Self { scraper }
    }
}

#[async_trait]
impl Scraper for AngularScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取 Angular 文档...");
        self.scraper.run().await
    }
}
