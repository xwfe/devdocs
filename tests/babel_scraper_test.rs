//! Babel 文档抓取器测试
//!
//! 使用 define_scraper 宏实现的 Babel 文档抓取器测试

use std::collections::HashMap;
use std::error::Error;

// 从 crate 根目录导入宏
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
            pub fn get_latest_version(&self) -> Result<String, Box<dyn Error>> {
                // 这里应该实现实际的版本检测逻辑
                // 为了简单起见，这里直接返回一个固定版本
                Ok(self.version.clone())
            }
            
            /// 运行抓取器
            pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
                println!("正在抓取 {} {} 文档...", self.name, self.version);
                println!("基础 URL: {}", self.base_url);
                println!("输出路径: {}", self.output_path);
                
                // 实现抓取逻辑，参照原始 Ruby 项目的实现
                // 1. 下载文档
                let base_url = format!("{}{}/", self.base_url, self.version);
                println!("下载文档从: {}", base_url);
                
                // 2. 应用过滤器
                // 在实际实现中，我们需要使用 nipper 库下载 HTML 内容并应用过滤器
                // 但目前我们只是模拟这个过程
                println!("应用过滤器...");
                
                // 3. 保存结果
                println!("保存结果到: {}", self.output_path);
                
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
    };
}

// 使用宏定义 Babel 抓取器
define_scraper!(
    name: BabelScraper,
    doc_name: "Babel",
    version: "7",
    base_url: "https://babeljs.io/docs/",
    output_path: "output",
    doc_type: "simple",
    release: "7.21.4",
    links: {
        "home" => "https://babeljs.io/",
        "code" => "https://github.com/babel/babel"
    },
    options: {
        "trailing_slash" => "true",
        "attribution" => "&copy; 2014-present Sebastian McKenzie<br>Licensed under the MIT License."
    },
    skip_patterns: [
        "/usage/",
        "/configuration/",
        "/learn/",
        "/v7-migration/",
        "/v7-migration-api/",
        "/editors/",
        "/presets/",
        "/caveats/",
        "/faq/",
        "/roadmap/"
    ]
);

fn main() {
    println!("开始测试 Babel 文档抓取器...");
    
    // 创建抓取器
    let mut scraper = BabelScraper::new("7");
    
    // 打印抓取器信息
    println!("抓取器名称: {}", scraper.name);
    println!("抓取器版本: {}", scraper.version);
    println!("基础 URL: {}", scraper.base_url);
    println!("输出路径: {}", scraper.output_path);
    
    if let Some(doc_type) = &scraper.doc_type {
        println!("文档类型: {}", doc_type);
    }
    
    if let Some(release) = &scraper.release {
        println!("发布版本: {}", release);
    }
    
    println!("链接数量: {}", scraper.links.len());
    for (key, value) in &scraper.links {
        println!("  - {}: {}", key, value);
    }
    
    println!("选项数量: {}", scraper.options.len());
    for (key, value) in &scraper.options {
        println!("  - {}: {}", key, value);
    }
    
    println!("跳过模式数量: {}", scraper.skip_patterns.len());
    for pattern in &scraper.skip_patterns {
        println!("  - {}", pattern);
    }
    
    // 运行抓取器
    println!("\n开始运行抓取器...");
    match scraper.run() {
        Ok(_) => println!("抓取成功!"),
        Err(e) => println!("抓取失败: {}", e),
    }
    
    println!("测试完成!");
}
