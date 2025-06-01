//! css documentation scraper
//!
//! 严格按照原版 Ruby 实现转换的 css 文档抓取器
//! 参考文件: lib/docs/scrapers/mdn/css.rb

use crate::core::error::Result;
use crate::core::scraper::base::{Scraper, UrlScraper};
use async_trait::async_trait;

/// CSS文档爬虫
pub struct CssScraper {
    /// 基础爬虫
    scraper: UrlScraper,
}

impl CssScraper {
    /// 创建新的CSS文档爬虫
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://developer.mozilla.org/en-US/docs/Web/CSS";
        let mut scraper = UrlScraper::new("CSS", version, base_url, output_path);

        // 添加初始路径
        let initial_paths = vec![
            "/".to_string(),
            "/Reference".to_string(),
            "/Selectors".to_string(),
        ];

        // 添加初始路径
        scraper = scraper.with_initial_paths(initial_paths);

        Self { scraper }
    }
}

#[async_trait]
impl Scraper for CssScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取CSS文档...");
        self.scraper.run().await
    }
}
