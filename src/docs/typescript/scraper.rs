//! TypeScript 文档爬虫

use crate::core::error::Result;
use crate::core::scraper::base::{Scraper, UrlScraper};
use async_trait::async_trait;

/// TypeScript 文档爬虫
pub struct TypeScriptScraper {
    /// 基础爬虫
    scraper: UrlScraper,
}

impl TypeScriptScraper {
    /// 创建新的 TypeScript 文档爬虫
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://www.typescriptlang.org/docs";
        let mut scraper = UrlScraper::new("TypeScript", version, base_url, output_path);

        // 添加初始路径
        let initial_paths = vec![
            "/".to_string(),
            "/handbook/intro.html".to_string(),
            "/handbook/typescript-in-5-minutes.html".to_string(),
            "/handbook/2/basic-types.html".to_string(),
            "/handbook/2/functions.html".to_string(),
            "/handbook/2/classes.html".to_string(),
        ];

        // 添加初始路径
        scraper = scraper.with_initial_paths(initial_paths);

        Self { scraper }
    }
}

#[async_trait]
impl Scraper for TypeScriptScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取TypeScript文档...");
        self.scraper.run().await
    }
}
