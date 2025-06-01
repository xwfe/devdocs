//! 抓取器宏定义
//!
//! 这个模块包含用于简化抓取器创建的宏

use std::collections::HashMap;
use crate::core::scraper::ScraperTrait;
use crate::core::error::Error;

/// 定义文档抓取器宏
///
/// 用于简化文档抓取器的创建，减少模板代码
#[macro_export]
macro_rules! define_scraper {
    (
        name: $name:ident,
        doc_name: $doc_name:expr,
        version: $version:expr,
        base_url: $base_url:expr,
        output_path: $output_path:expr,
        $(doc_type: $doc_type:expr,)?
        $(release: $release:expr,)?
        $(links: {
            $($link_key:expr => $link_value:expr),*
        },)?
        $(options: {
            $($option_key:expr => $option_value:expr),*
        },)?
        $(skip_patterns: [$($pattern:expr),*])?
    ) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            /// 文档名称
            pub name: String,
            /// 文档版本
            pub version: String,
            /// 基础 URL
            pub base_url: String,
            /// 输出路径
            pub output_path: String,
            /// 文档类型
            pub doc_type: Option<String>,
            /// 发布版本
            pub release: Option<String>,
            /// 链接
            pub links: HashMap<String, String>,
            /// 选项
            pub options: HashMap<String, String>,
            /// 跳过模式
            pub skip_patterns: Vec<String>,
        }
        
        impl $name {
            /// 创建新的文档抓取器
            pub fn new(version: &str) -> Self {
                let mut scraper = Self {
                    name: $doc_name.to_string(),
                    version: version.to_string(),
                    base_url: $base_url.to_string(),
                    output_path: $output_path.to_string(),
                    doc_type: None,
                    release: None,
                    links: HashMap::new(),
                    options: HashMap::new(),
                    skip_patterns: Vec::new(),
                };
                
                // 设置文档类型
                $(scraper.doc_type = Some($doc_type.to_string());)?
                
                // 设置发布版本
                $(scraper.release = Some($release.to_string());)?
                
                // 添加链接
                $(
                    $(
                        scraper.links.insert($link_key.to_string(), $link_value.to_string());
                    )*
                )?
                
                // 设置选项
                $(
                    $(
                        scraper.options.insert($option_key.to_string(), $option_value.to_string());
                    )*
                )?
                
                // 添加跳过模式
                $(
                    $(
                        scraper.skip_patterns.push($pattern.to_string());
                    )*
                )?
                
                scraper
            }
            
            /// 获取最新版本
            pub fn get_latest_version(&self) -> Result<String, Box<dyn std::error::Error>> {
                // 这里应该实现实际的版本检测逻辑
                // 为了简单起见，这里直接返回一个固定版本
                Ok(self.version.clone())
            }
            
            /// 运行抓取器
            pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                use nipper::Document;
                
                println!("正在抓取 {} {} 文档...", self.name, self.version);
                println!("基础 URL: {}", self.base_url);
                println!("输出路径: {}", self.output_path);
                
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
                                
                                // 收集链接信息
                                entries.push((href.to_string(), text));
                            }
                        }
                    }
                });
                
                println!("找到 {} 个有效链接", links.len());
                
                // 6. 收集页面内容
                let mut pages = Vec::new();
                
                for link in &links {
                    
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
                                    
                                    // 收集页面信息
                                    pages.push((link.clone(), page_title.to_string(), content));
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
                
                // 7. 返回抓取结果
                println!("抓取完成，共抓取了 {} 个页面", pages.len());
                
                // 返回成功结果
                Ok(())
            }
            
            /// 获取名称
            pub fn name(&self) -> &str {
                &self.name
            }
            
            /// 获取版本
            pub fn version(&self) -> Option<&str> {
                Some(&self.version)
            }
        }
        
        // 实现 ScraperTrait 特征，使其可以被注册到文档注册表中
        impl crate::core::scraper::ScraperTrait for $name {
            fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
                // 调用实例方法
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
                // 调用实例方法
                self.get_latest_version()
            }
            
            fn clone_box(&self) -> Box<dyn crate::core::scraper::ScraperTrait> {
                Box::new(self.clone())
            }
        }
    };
}