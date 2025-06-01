//! 测试 Babel 抓取器
//!
//! 这是一个独立的测试文件，用于测试简化版 Babel 抓取器

use std::collections::HashMap;
use std::fmt::Debug;

/// 定义文档抓取器宏
///
/// 用于简化文档抓取器的创建，减少模板代码
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
        $(skip_patterns: [$($pattern:expr),*],)?
        $(run_impl: |$self:ident| $run_body:block)?
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
            pub links: std::collections::HashMap<String, String>,
            /// 选项
            pub options: std::collections::HashMap<String, String>,
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
                    links: std::collections::HashMap::new(),
                    options: std::collections::HashMap::new(),
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
                $(
                    let $self = self;
                    $run_body
                )?                
                $(({
                    // 默认实现
                    println!("正在抓取 {} {} 文档...", self.name, self.version);
                    println!("基础 URL: {}", self.base_url);
                    println!("输出路径: {}", self.output_path);
                    Ok(())
                }))?
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
    name: SimplifiedBabelScraper,
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
    ],
    run_impl: |self| {
        println!("正在抓取 Babel {} 文档...", self.version);
        println!("基础 URL: {}", self.base_url);
        println!("输出路径: {}", self.output_path);
        
        // 实现抓取逻辑，参照原始 Ruby 项目的实现
        // 1. 下载文档
        let base_url = format!("{}{}/", self.base_url, self.version);
        println!("下载文档从: {}", base_url);
        
        // 2. 应用过滤器
        // 在实际实现中，我们需要下载 HTML 内容并应用过滤器
        // 但目前我们只是模拟这个过程
        println!("应用过滤器...");
        
        // 3. 保存结果
        println!("保存结果到: {}", self.output_path);
        
        Ok(())
    }
);

#[test]
fn test_simplified_babel_scraper() {
    println!("测试运行简化版 Babel 抓取器...");
    
    // 创建抓取器
    let mut scraper = SimplifiedBabelScraper::new("7");
    
    // 检查属性
    assert_eq!(scraper.name, "Babel");
    assert_eq!(scraper.version, "7");
    assert_eq!(scraper.base_url, "https://babeljs.io/docs/");
    assert_eq!(scraper.output_path, "output");
    assert_eq!(scraper.doc_type, Some("simple".to_string()));
    assert_eq!(scraper.release, Some("7.21.4".to_string()));
    
    // 检查链接
    assert_eq!(scraper.links.get("home"), Some(&"https://babeljs.io/".to_string()));
    assert_eq!(scraper.links.get("code"), Some(&"https://github.com/babel/babel".to_string()));
    
    // 检查选项
    assert_eq!(scraper.options.get("trailing_slash"), Some(&"true".to_string()));
    
    // 检查跳过模式
    assert!(scraper.skip_patterns.contains(&"/usage/".to_string()));
    assert!(scraper.skip_patterns.contains(&"/configuration/".to_string()));
    
    // 运行抓取器
    let result = scraper.run();
    assert!(result.is_ok());
    
    println!("测试完成!");
}

fn main() {
    // 运行测试
    test_simplified_babel_scraper();
}
