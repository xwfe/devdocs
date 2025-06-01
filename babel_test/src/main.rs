use std::error::Error;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use nipper::Document;
use reqwest;
use std::io::Write;

/// 抓取器特征，定义了文档抓取器的基本接口
pub trait ScraperTrait {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>>;
    fn name(&self) -> &str;
    fn version(&self) -> Option<&str>;
    fn base_url(&self) -> &str;
    fn output_path(&self) -> &str;
    fn doc_type(&self) -> Option<&str>;
    fn release(&self) -> Option<&str>;
    fn get_latest_version(&self) -> Result<String, Box<dyn std::error::Error>>;
    fn clone_box(&self) -> Box<dyn ScraperTrait>;
}

/// Babel 文档抓取器
#[derive(Debug, Clone)]
pub struct BabelScraper {
    name: String,
    version: String,
    base_url: String,
    output_path: String,
    doc_type: Option<String>,
    release: Option<String>,
    skip_patterns: Vec<String>,
}

impl BabelScraper {
    /// 创建新的 Babel 抓取器
    pub fn new(version: &str) -> Self {
        Self {
            name: "Babel".to_string(),
            version: version.to_string(),
            base_url: "https://babeljs.io/docs".to_string(),
            output_path: "output".to_string(),
            doc_type: Some("simple".to_string()),
            release: Some("7.21.4".to_string()),
            skip_patterns: vec![
                "/usage/".to_string(),
                "/configuration/".to_string(),
                "/learn/".to_string(),
                "/v7-migration/".to_string(),
                "/v7-migration-api/".to_string(),
                "/editors/".to_string(),
                "/presets/".to_string(),
                "/caveats/".to_string(),
                "/faq/".to_string(),
                "/roadmap/".to_string(),
            ],
        }
    }

    /// 运行抓取器
    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("正在抓取 {} {} 文档...", self.name, self.version);
        println!("基础 URL: {}", self.base_url);
        println!("输出路径: {}", self.output_path);
        
        // 1. 创建输出目录
        let output_dir = Path::new(&self.output_path);
        if !output_dir.exists() {
            fs::create_dir_all(output_dir)?;
        }
        
        // 2. 下载文档
        // 确保正确处理 URL，如果版本是 latest，则直接使用 base_url
        let base_url = if self.version == "latest" {
            self.base_url.to_string()
        } else if self.base_url.ends_with('/') {
            format!("{}{}", self.base_url, self.version)
        } else {
            format!("{}/{}", self.base_url, self.version)
        };
        println!("下载文档从: {}", base_url);
        
        // 使用 reqwest 下载首页
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30)) // 增加超时时间
            .build()?;
            
        // 添加重试机制
        let mut retry_count = 0;
        let max_retries = 3;
        let mut html = String::new();
        
        while retry_count < max_retries {
            match client.get(&base_url).send() {
                Ok(response) => {
                    match response.text() {
                        Ok(text) => {
                            html = text;
                            break;
                        }
                        Err(e) => {
                            println!("获取响应内容失败: {}", e);
                            retry_count += 1;
                            if retry_count >= max_retries {
                                return Err(Box::new(e));
                            }
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        }
                    }
                }
                Err(e) => {
                    println!("请求失败: {}", e);
                    retry_count += 1;
                    if retry_count >= max_retries {
                        return Err(Box::new(e));
                    }
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }
        
        // 3. 使用 nipper 解析 HTML
        println!("使用 nipper 解析 HTML...");
        let document = Document::from(&html);
        
        // 4. 提取文档标题
        let title = document.select("title").text();
        println!("文档标题: {}", title);
        
        // 5. 提取所有链接
        let mut links = Vec::new();
        let mut entries = Vec::new();
        
        document.select("a[href]").iter().for_each(|link| {
            if let Some(href) = link.attr("href") {
                // 过滤跳过模式
                let skip = self.skip_patterns.iter().any(|pattern| href.contains(pattern));
                if !skip && href.starts_with('/') {
                    // 提取链接文本
                    let text = link.text().trim().to_string();
                    if !text.is_empty() {
                        links.push(href.to_string());
                        
                        // 创建条目对象
                        let entry = format!("{{
  \"path\": \"{}\",
  \"title\": \"{}\",
  \"type\": \"doc\"  
}}", href, text.replace("\"", "\\\""));
                        entries.push(entry);
                    }
                }
            }
        });
        
        println!("找到 {} 个有效链接", links.len());
        
        // 6. 递归抓取链接页面
        let max_pages = 5; // 限制页面数量，避免抓取过多
        let mut processed_links = 0;
        
        for link in &links {
            if processed_links >= max_pages {
                println!("达到最大页面限制，停止递归抓取");
                break;
            }
            
            // 构建完整 URL
            let page_url = if self.base_url.ends_with('/') {
                format!("{}{}", self.base_url, link.trim_start_matches('/'))
            } else {
                format!("{}{}", self.base_url, link)
            };
            
            println!("抓取页面: {}", page_url);
            
            // 尝试下载页面
            match client.get(&page_url).send() {
                Ok(response) => {
                    match response.text() {
                        Ok(page_html) => {
                            // 解析页面内容
                            let page_doc = Document::from(&page_html);
                            let page_title = page_doc.select("title").text();
                            
                            // 提取页面内容
                            let content = page_doc.select("main").html();
                            
                            // 保存页面内容
                            let page_path = link.trim_start_matches('/');
                            let file_path = if page_path.is_empty() || page_path.ends_with('/') {
                                format!("{}/index.html", page_path)
                            } else {
                                format!("{}.html", page_path)
                            };
                            
                            // 创建目录
                            if let Some(parent) = Path::new(&file_path).parent() {
                                let full_dir = output_dir.join(parent);
                                if !full_dir.exists() {
                                    fs::create_dir_all(&full_dir)?;
                                }
                            }
                            
                            // 写入文件
                            let full_path = output_dir.join(&file_path);
                            let mut file = fs::File::create(&full_path)?;
                            file.write_all(content.as_bytes())?;
                            
                            println!("保存页面: {}", full_path.display());
                            processed_links += 1;
                        }
                        Err(e) => {
                            println!("获取页面内容失败: {} - {}", page_url, e);
                        }
                    }
                }
                Err(e) => {
                    println!("请求页面失败: {} - {}", page_url, e);
                }
            }
            
            // 添加延时，避免请求过快
            std::thread::sleep(std::time::Duration::from_millis(500));
        }
        
        // 7. 保存索引文件
        let index_path = output_dir.join("index.json");
        let index_path_display = format!("{}", index_path.display()); // 先保存显示路径
        let mut index_file = fs::File::create(&index_path)?;
        
        // 创建更详细的 JSON 结构
        let json = format!("{{
  \"name\": \"{}\",
  \"version\": \"{}\",
  \"release\": \"{}\",
  \"type\": \"{}\",
  \"links\": {{
    \"home\": \"{}\",
    \"code\": \"{}\"
  }},
  \"entries\": [
    {}
  ]
}}",
            self.name,
            self.version,
            self.release.as_deref().unwrap_or(""),
            self.doc_type.as_deref().unwrap_or("simple"),
            self.base_url,
            self.base_url,
            entries.join(",\n    ")
        );
        
        index_file.write_all(json.as_bytes())?;
        println!("保存结果到: {}", index_path_display);
        
        Ok(())
    }

    /// 获取最新版本
    pub fn get_latest_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        // 在实际实现中，我们应该从网站获取最新版本
        // 但为了简单起见，这里直接返回一个硬编码的版本
        Ok("7.21.4".to_string())
    }
}

impl ScraperTrait for BabelScraper {
    fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.run()
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn version(&self) -> Option<&str> {
        Some(&self.version)
    }
    
    fn base_url(&self) -> &str {
        &self.base_url
    }
    
    fn output_path(&self) -> &str {
        &self.output_path
    }
    
    fn doc_type(&self) -> Option<&str> {
        self.doc_type.as_deref()
    }
    
    fn release(&self) -> Option<&str> {
        self.release.as_deref()
    }
    
    fn get_latest_version(&self) -> Result<String, Box<dyn std::error::Error>> {
        self.get_latest_version()
    }
    
    fn clone_box(&self) -> Box<dyn ScraperTrait> {
        Box::new(self.clone())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Babel 文档抓取器测试程序");
    
    // 创建 Babel 抓取器
    let mut scraper = BabelScraper::new("latest");
    
    // 打印抓取器信息
    println!("Babel 抓取器信息:");
    println!("  名称: {}", scraper.name());
    println!("  版本: {:?}", scraper.version());
    println!("  基础 URL: {}", scraper.base_url());
    println!("  输出路径: {}", scraper.output_path());
    println!("  文档类型: {:?}", scraper.doc_type());
    println!("  发布版本: {:?}", scraper.release());
    
    // 运行抓取器
    scraper.run()
}