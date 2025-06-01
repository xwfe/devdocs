//! HTML documentation scraper
//!
//! 严格按照原版 Ruby 实现转换的 HTML 文档抓取器
//! 参考文件: lib/docs/scrapers/mdn/html.rb

use crate::core::error::Result;
use crate::core::scraper::base::{Scraper, UrlScraper};
use crate::docs::html::HtmlEntriesFilter;
use async_trait::async_trait;

/// HTML文档爬虫
pub struct HtmlScraper {
    /// 基础爬虫
    scraper: UrlScraper,
}

impl HtmlScraper {
    /// 创建新的HTML文档爬虫（仅抓取页面和图片，过滤无用资源）
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://developer.mozilla.org/en-US/docs/Web/HTML";
        let mut scraper = UrlScraper::new("HTML", version, base_url, output_path);

        // 只抓取首页、元素、全局属性等主要入口
        let initial_paths = vec![
            "/".to_string(),
            "/Element".to_string(),
            "/Global_attributes".to_string(),
        ];

        // 添加过滤器
        let html_entries = Box::new(HtmlEntriesFilter::new());

        // 组合过滤器和初始路径
        scraper = scraper
            .with_initial_paths(initial_paths)
            .with_filter(html_entries);

        Self { scraper }
    }
}

#[async_trait]
impl Scraper for HtmlScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取HTML文档...");
        self.scraper.run().await
    }
}
