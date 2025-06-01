//! JavaScript documentation scraper
//!
//! 严格按照原版 Ruby 实现转换的 JavaScript 文档抓取器
//! 参考文件: lib/docs/scrapers/mdn/javaScript.rb

use crate::core::error::Result;
use crate::core::scraper::base::{Scraper, UrlScraper};
use crate::docs::javascript::JavaScriptEntriesFilter;
use async_trait::async_trait;
use chrono::{Datelike, NaiveDateTime};
use reqwest::Client;

/// JavaScript文档爬虫
pub struct JavaScriptScraper {
    /// 基础爬虫
    scraper: UrlScraper,
}

const LINKS: [(&str, &str); 2] = [
    (
        "home",
        "https://developer.mozilla.org/en-US/docs/Web/JavaScript",
    ),
    (
        "code",
        "https://github.com/mdn/content/tree/main/files/en-us/web/javascript",
    ),
];

const ROOT_TITLE: &str = "JavaScript";
const CONTAINER: &str = "#content > .main-page-content";
const ATTRIBUTION: &str = "© 2005–2023 MDN contributors.\nLicensed under the Creative Commons Attribution-ShareAlike License v2.5 or later.";

impl JavaScriptScraper {
    /// 创建新的JavaScript文档爬虫
    pub fn new(version: &str, output_path: &str) -> Self {
        let base_url = "https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference";
        let mut scraper = UrlScraper::new("JavaScript", version, base_url, output_path)
            .with_root_title(ROOT_TITLE)
            .with_attribution(ATTRIBUTION)
            .with_string_links(
                LINKS
                    .iter()
                    .map(|&(k, v)| (k.to_string(), v.to_string()))
                    .collect(),
            );

        // 跳过的路径（重复内容）
        let skip_paths = vec![
            "/Global_Objects".to_string(),
            "/Operators".to_string(),
            "/Statements".to_string(),
        ];

        // 额外跳过的模式
        let skip_patterns = vec![
            "/additional_examples",
            "/noSuchMethod",
            "/Deprecated_and_obsolete_features",
        ];

        // 路径替换映射
        let replace_paths = vec![
            (
                "/template_strings".to_string(),
                "/Template_literals".to_string(),
            ),
            (
                "/default_parameters".to_string(),
                "/Default_parameters".to_string(),
            ),
            (
                "/rest_parameters".to_string(),
                "/Rest_parameters".to_string(),
            ),
            ("/spread_operator".to_string(), "/Spread_syntax".to_string()),
            (
                "/destructuring_assignment".to_string(),
                "/Destructuring_assignment".to_string(),
            ),
        ];

        // 过滤器
        let entries_filter = Box::new(JavaScriptEntriesFilter::new());

        // 配置抓取器
        scraper = scraper
            .with_skip_patterns(skip_patterns)
            .with_filter(entries_filter);

        Self { scraper }
    }

    /// 获取最新版本
    pub async fn get_latest_version(&self) -> Result<String> {
        // 获取MDN最新更新时间
        let client = Client::new();
        let response = client
            .get("https://developer.mozilla.org/en-US/docs/Web/JavaScript")
            .send()
            .await?;

        let html = response.text().await?;

        // 简单解析最后更新时间
        if let Some(pos) = html.find("\"dateModified\":") {
            if let Some(end_pos) = html[pos..].find(",") {
                let date_str = html[pos + 16..pos + end_pos - 1].trim_matches('"');
                if let Ok(date) = NaiveDateTime::parse_from_str(date_str, "%Y-%m-%dT%H:%M:%S%.fZ") {
                    return Ok(format!("ES{}", date.year() - 2015 + 6));
                }
            }
        }

        // 默认返回ES6
        Ok("ES6".to_string())
    }
}

#[async_trait]
impl Scraper for JavaScriptScraper {
    fn name(&self) -> &str {
        self.scraper.name()
    }

    fn version(&self) -> &str {
        self.scraper.version()
    }

    async fn run(&mut self) -> Result<()> {
        println!("开始抓取JavaScript文档...");
        self.scraper.run().await
    }
}
