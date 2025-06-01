//! Rust 文档抓取器

use crate::core::error::Result;
use crate::core::scraper::base::{Scraper, UrlScraper};
use async_trait::async_trait;

/// Rust文档抓取器
pub struct RustScraper {
    /// 基础抓取器
    scraper: UrlScraper,
}

impl RustScraper {
    /// 创建新的Rust文档抓取器
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://doc.rust-lang.org/";
        let mut scraper = UrlScraper::new("Rust", version, base_url, output_path);

        // 添加初始路径
        let initial_paths = vec![
            "std/index.html".to_string(),
            "book/index.html".to_string(),
            "reference/index.html".to_string(),
            "cargo/index.html".to_string(),
            "rustc/index.html".to_string(),
        ];

        // 添加初始路径
        scraper = scraper.with_initial_paths(initial_paths);

        Self { scraper }
    }
}

#[async_trait]
impl Scraper for RustScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取Rust文档...");
        self.scraper.run().await
    }
}
