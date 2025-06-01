//! Scraper Base 模块
//!
//! 严格对齐原版 Ruby 项目中的 scraper 基础功能

use crate::core::error::{Error, Result};
use crate::core::scraper::filter::{Filter, FilterContext};
use crate::core::{
    entry_index::EntryIndex, 
    page_db::PageDb
};
use crate::storage::{file_store::FileStore, store::Store};
use async_trait::async_trait;
use nipper::Document;
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use url::Url;

/// 爬虫基类
///
/// 对应原版 Ruby 的 Scraper 基类
#[async_trait]
pub trait Scraper {
    /// 获取爬虫名称
    fn name(&self) -> &str;

    /// 获取版本
    fn version(&self) -> &str;

    /// 运行爬虫
    async fn run(&mut self) -> Result<()>;

    /// 获取基础 URL
    fn base_url(&self) -> &str {
        ""
    }

    /// 获取初始路径
    fn initial_paths(&self) -> &[String] {
        &[]
    }
}

/// URL 爬虫基类
///
/// 对应原版 Ruby 的 UrlScraper 类
pub struct UrlScraper {
    pub name: String,
    pub version: String,
    pub base_url: String,
    pub output_path: String,
    pub root_title: Option<String>,
    pub attribution: Option<String>,
    pub links: HashMap<String, String>,
    pub trailing_slash: bool,
    pub skip_patterns: Vec<String>,
    pub skip_link_fn: Option<Box<dyn Fn(&str) -> bool + Send + Sync>>,
    pub filters: Vec<Box<dyn Filter>>,
    pub initial_paths: Vec<String>,
}

impl UrlScraper {
    /// 创建新的 URL 爬虫
    pub fn new(name: &str, version: &str, base_url: &str, output_path: &str) -> Self {
        Self {
            name: name.to_string(),
            version: version.to_string(),
            base_url: base_url.to_string(),
            output_path: output_path.to_string(),
            root_title: None,
            attribution: None,
            links: HashMap::new(),
            trailing_slash: false,
            skip_patterns: Vec::new(),
            skip_link_fn: None,
            filters: Vec::new(),
            initial_paths: vec!["/".to_string()],
        }
    }

    /// 设置根标题
    pub fn with_root_title(mut self, title: &str) -> Self {
        self.root_title = Some(title.to_string());
        self
    }

    /// 设置归属信息
    pub fn with_attribution(mut self, attribution: &str) -> Self {
        self.attribution = Some(attribution.to_string());
        self
    }

    /// 设置字符串链接
    pub fn with_string_links(mut self, links: Vec<(String, String)>) -> Self {
        for (key, value) in links {
            self.links.insert(key, value);
        }
        self
    }

    /// 设置末尾斜杠
    pub fn with_trailing_slash(mut self, trailing_slash: bool) -> Self {
        self.trailing_slash = trailing_slash;
        self
    }

    /// 设置跳过模式
    pub fn with_skip_patterns(mut self, patterns: Vec<&str>) -> Self {
        self.skip_patterns = patterns.into_iter().map(|s| s.to_string()).collect();
        self
    }

    /// 设置跳过链接函数
    pub fn with_skip_link<F>(mut self, skip_fn: F) -> Self
    where
        F: Fn(&str) -> bool + Send + Sync + 'static,
    {
        self.skip_link_fn = Some(Box::new(skip_fn));
        self
    }

    /// 添加过滤器
    pub fn with_filter(mut self, filter: Box<dyn Filter>) -> Self {
        self.filters.push(filter);
        self
    }

    /// 设置初始路径
    pub fn with_initial_paths(mut self, paths: Vec<String>) -> Self {
        self.initial_paths = paths;
        self
    }

    /// 应用所有过滤器
    pub fn apply_filters(&self, html: &str, context: &mut FilterContext) -> Result<String> {
        let mut result = html.to_string();
        for filter in &self.filters {
            result = filter.apply(&result, context)?;
        }
        Ok(result)
    }

    /// 获取所有条目
    pub fn get_all_entries(
        &self,
        html: &str,
        context: &FilterContext,
    ) -> Vec<(String, String, String)> {
        let mut all_entries = Vec::new();
        for filter in &self.filters {
            let entries = filter.get_entries(html, context);
            all_entries.extend(entries);
        }
        all_entries
    }

    fn should_skip_link(&self, href: &str) -> bool {
        // 首先检查自定义跳过函数
        if let Some(skip_fn) = &self.skip_link_fn {
            if skip_fn(href) {
                println!("跳过链接 (自定义函数): {}", href);
                return true;
            }
        }
        
        // 然后检查跳过模式
        for pattern in &self.skip_patterns {
            if href.contains(pattern) {
                println!("跳过链接 (模式匹配): {} 包含 {}", href, pattern);
                return true;
            }
        }
        
        false
    }

    /// 检查是否应该跳过路径
    pub fn should_skip_path(&self, path: &str) -> bool {
        // 尝试将完整URL转换为路径部分
        let path_to_check = if path.starts_with(&self.base_url) {
            // 如果是完整URL，提取路径部分
            match Url::parse(path) {
                Ok(url) => url.path().to_string(),
                Err(_) => path.to_string(),
            }
        } else {
            // 已经是路径
            path.to_string()
        };
        
        // 将路径分割为部分，检查每个部分是否匹配跳过模式
        let path_segments: Vec<&str> = path_to_check.split('/').filter(|s| !s.is_empty()).collect();
        
        self.skip_patterns.iter().any(|pattern| {
            // 检查每个路径部分是否精确匹配模式
            path_segments.iter().any(|segment| *segment == *pattern)
        })
    }

    /// 抓取单个页面
    async fn fetch_page(&self, url: &str) -> Result<String> {
        // URL 已经是完整的 URL，不需要再拼接
        let fetch_url = if url.starts_with("http") {
            url.to_string()
        } else {
            format!("{}{}", self.base_url.trim_end_matches('/'), url)
        };

        // 使用 tokio 的异步 HTTP 客户端
        let client = reqwest::Client::new();
        let response = client
            .get(&fetch_url)
            .header("User-Agent", "DevDocs Rust")
            .send()
            .await
            .map_err(|e| Error::Http(e))?;

        // 检查响应状态
        if !response.status().is_success() {
            eprintln!("Failed to fetch {}: {}", fetch_url, response.status());
            return Err(Error::HttpError(response.status().as_u16()));
        }

        let body = response.text().await.map_err(|e| Error::Http(e))?;

        Ok(body)
    }

    /// 存储页面内容
    fn store_page(&self, path: &str, content: &str) -> Result<()> {
        let file_path = self.output_path.clone() + path;
        let parent = Path::new(&file_path).parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Failed to get parent directory",
            )
        })?;

        // 确保目录存在
        fs::create_dir_all(parent)?;

        // 写入文件
        fs::write(file_path, content)?;
        Ok(())
    }

    /// 从页面内容中发现内部链接
    /// 对应原版 Ruby 的 internal_urls 过滤器功能
    fn find_internal_links(&self, html: &str, current_url: &str) -> Vec<String> {
        let document = Document::from(html);
        let mut links = Vec::new();
        let base_url = match Url::parse(&self.base_url) {
            Ok(url) => url,
            Err(_) => return links, // 如果基础 URL 无效，则无法解析链接
        };

        for element_selection in document.select("a[href]").iter() {
            if let Some(href_attr_cow) = element_selection.attr("href") {
                let href_attr = href_attr_cow.as_ref();
                if href_attr.is_empty() || href_attr.starts_with('#') || href_attr.starts_with("mailto:") || href_attr.starts_with("javascript:") {
                    continue;
                }

                match base_url.join(href_attr) {
                    Ok(mut abs_url) => {
                        // 移除 fragment
                        abs_url.set_fragment(None);

                        // 检查是否是内部链接 (与 base_url 同源)
                        if abs_url.domain() == base_url.domain()
                            && abs_url.port_or_known_default() == base_url.port_or_known_default()
                            && abs_url.scheme() == base_url.scheme()
                            && abs_url.to_string().starts_with(&self.base_url) // 新增检查
                        {
                            let mut link_str = abs_url.to_string();

                            // 根据 trailing_slash 配置处理末尾斜杠
                            if self.trailing_slash && !link_str.ends_with('/') && !link_str.contains('.') {
                                link_str.push('/');
                            } else if !self.trailing_slash && link_str.ends_with('/') {
                                link_str.pop();
                            }
                            
                            // 避免重复添加，并确保不跳过此链接
                            // 同时检查 should_skip_link 和 should_skip_path
                            if !links.contains(&link_str) && 
                               !self.should_skip_link(&link_str) && 
                               !self.should_skip_path(&link_str) {
                                links.push(link_str);
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("解析链接失败 '{}' (base: '{}', current: '{}'): {}", href_attr, self.base_url, current_url, e);
                    }
                }
            }
        }
        links
    }

    /// 抓取所有页面的实际实现
    async fn scrape_all_pages(&mut self) -> Result<()> {
        println!("开始抓取所有页面...");

        // 创建数据存储
        let mut page_db = PageDb::new();
        let mut entry_index = EntryIndex::new();

        // 创建存储
        let store = FileStore::new(&self.output_path);

        // 创建初始 URL 队列 - 严格对齐Ruby版本的initial_urls方法
        // Ruby: [root_url.to_s].concat(initial_paths.map(&method(:url_for)))
        let mut url_queue: Vec<String> = Vec::new();

        // 首先添加root_url (等同于Ruby的root_url.to_s)
        url_queue.push(self.base_url.clone());

        // 然后为每个initial_path调用url_for逻辑
        for path in &self.initial_paths {
            let url = if path.is_empty() || path == "/" {
                // Ruby: root_url.to_s
                self.base_url.clone()
            } else if path.starts_with("http") {
                // 绝对URL直接使用
                path.clone()
            } else {
                // Ruby: File.join(base_url.to_s, path.to_s)
                let base_url = self.base_url.trim_end_matches('/');
                if path.starts_with('/') {
                    format!("{}{}", base_url, path)
                } else {
                    format!("{}/{}", base_url, path)
                }
            };
            if !url_queue.contains(&url) {
                url_queue.push(url);
            }
        }

        let mut processed_urls = HashSet::new();

        // 处理队列中的每个 URL
        while let Some(url) = url_queue.pop() {
            if processed_urls.contains(&url) {
                continue;
            }

            // 检查是否应该跳过这个路径
            if self.should_skip_path(&url) {
                continue;
            }

            println!("正在抓取: {}", url);
            processed_urls.insert(url.clone());

            // 发送 HTTP 请求
            match self.fetch_page(&url).await {
                Ok(content) => {
                    // 创建过滤器上下文
                    let raw_path = url.strip_prefix(&self.base_url).unwrap_or(&url);
                    // 将空路径或"/"转换为"index"，以匹配原版Ruby行为
                    let path = if raw_path.is_empty() || raw_path == "/" {
                        "index".to_string()
                    } else {
                        raw_path.trim_start_matches('/').to_string()
                    };
                    let mut context = FilterContext {
                        entries: Vec::new(), // 添加 entries 初始化
                        current_path: path.to_string(),
                        current_url: url.clone(),
                        base_url: self.base_url.clone(),
                        root_url: self.base_url.clone(),
                        root_path: None, // 使用 Option<String>
                        links: Vec::new(),
                        version: None,
                        release: None,
                        initial_paths: self.initial_paths.clone(),
                        options: HashMap::new(),
                    };

                    // 应用所有过滤器处理内容
                    let processed_content = self.apply_filters(&content, &mut context)?;

                    // 获取条目
                    let entries = self.get_all_entries(&content, &context);
                    for (name, entry_path, entry_type) in entries {
                        if let Ok(entry) = crate::core::models::Entry::new(
                            Some(name),
                            Some(entry_path),
                            Some(entry_type),
                        ) {
                            entry_index.add(entry);
                        }
                    }

                    // 将处理过的内容添加到页面数据库
                    page_db.add(path.to_string(), processed_content);

                    // 查找新的链接（这里可以添加链接发现逻辑）
                    let new_links = self.find_internal_links(&content, &url);
                    for link in new_links {
                        if !processed_urls.contains(&link) && !self.should_skip_link(&link) {
                            url_queue.push(link);
                        }
                    }
                }
                Err(e) => {
                    eprintln!("抓取失败 {}: {}", url, e);
                }
            }
        }

        // 保存结果
        self.save_results(&store, &page_db, &entry_index)?;

        println!("抓取完成！");
        Ok(())
    }

    /// 保存抓取结果
    /// 对应原版 Ruby 的 store_pages 方法逻辑
    fn save_results(
        &self,
        store: &FileStore,
        page_db: &PageDb,
        entry_index: &EntryIndex,
    ) -> Result<()> {
        // 保存页面数据库 (db.json)
        let db_content = page_db.to_json();
        store.write("db.json", &db_content)?;

        // 保存索引 (index.json)
        let index_content = entry_index.to_json();
        store.write("index.json", &index_content)?;

        // 保存元数据 (meta.json) - 对应原版 Ruby 的 store_meta 方法
        self.store_meta(store, &db_content)?;

        println!("已保存到: {}", self.output_path);
        Ok(())
    }

    /// 存储元数据
    /// 对应原版 Ruby 的 store_meta 方法
    fn store_meta(&self, store: &FileStore, db_content: &str) -> Result<()> {
        // 构建 slug - 对应原版 Ruby 的 slug 方法逻辑
        let slug = if !self.version.is_empty() {
            format!("{}~{}", self.name.to_lowercase(), self.version_slug())
        } else {
            self.name.to_lowercase()
        };

        // 获取当前时间戳
        let mtime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        // 构建 meta.json 内容 - 对应原版 Ruby 的 as_json 方法
        let mut meta = json!({
            "name": self.name,
            "slug": slug,
            "type": "simple"
        });

        // 添加链接信息（如果存在）
        if !self.links.is_empty() {
            meta["links"] = json!(self.links);
        } else if !self.base_url.is_empty() {
            meta["links"] = json!({
                "home": self.base_url
            });
        }

        // 添加版本信息（如果存在）
        if !self.version.is_empty() {
            meta["version"] = json!(self.version);
        }

        // 添加时间戳和数据库大小
        meta["mtime"] = json!(mtime);
        meta["db_size"] = json!(db_content.len());

        // 写入 meta.json 文件
        let meta_content = meta.to_string();
        store.write("meta.json", &meta_content)?;

        Ok(())
    }

    /// 版本 slug 生成
    /// 对应原版 Ruby 的 version_slug 方法
    fn version_slug(&self) -> String {
        if self.version.is_empty() {
            return String::new();
        }

        let mut slug = self.version.to_lowercase();
        slug = slug.replace('+', "p");
        slug = slug.replace('#', "s");

        // 替换非字母数字字符为下划线
        slug = slug
            .chars()
            .map(|c| {
                if c.is_alphanumeric() || c == '_' || c == '.' {
                    c
                } else {
                    '_'
                }
            })
            .collect();

        slug
    }
}

#[async_trait]
impl Scraper for UrlScraper {
    fn name(&self) -> &str {
        &self.name
    }

    fn version(&self) -> &str {
        &self.version
    }

    fn base_url(&self) -> &str {
        &self.base_url
    }

    fn initial_paths(&self) -> &[String] {
        &self.initial_paths
    }

    async fn run(&mut self) -> Result<()> {
        println!("Running URL scraper for {} v{}", self.name, self.version);
        println!("Base URL: {}", self.base_url);
        println!("Initial paths: {:?}", self.initial_paths);

        // 实现实际的抓取逻辑
        self.scrape_all_pages().await?;

        Ok(())
    }
}
