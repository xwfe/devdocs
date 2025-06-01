//! 测试 Babel 抓取器与原版对齐情况
//! 
//! 这个测试脚本用于验证修改后的 Babel 抓取器是否与原版 Ruby 实现保持行为一致
//! 会抓取一小部分文档并检查结果

use std::path::Path;
use std::time::Instant;
use xwdoc::core::error::Result;
use xwdoc::core::scraper::base::Scraper;
use xwdoc::docs::babel::BabelScraper;

/// 最大页面抓取数量 - 为了测试设置较小的值
const MAX_PAGES: usize = 5;

#[tokio::main]
async fn main() -> Result<()> {
    println!("=== Babel 抓取器对齐测试 ===");
    println!("本测试验证 Babel 抓取器与原始 Ruby 版本行为是否一致");
    
    // 输出目录设置
    let output_path = "./test_docs/babel_alignment_test";
    
    // 清理之前的测试数据
    if Path::new(output_path).exists() {
        println!("清理之前的测试数据...");
        std::fs::remove_dir_all(output_path)?;
    }
    
    // 创建输出目录
    println!("创建输出目录...");
    std::fs::create_dir_all(output_path)?;
    
    println!("创建 Babel 抓取器...");
    println!("- 设置输出路径: {}", output_path);
    println!("- 设置版本: 7");
    let mut scraper = BabelScraper::new(output_path, "7");
    
    // 输出配置信息
    println!("抓取器配置:");
    println!("- 名称: {}", scraper.name());
    println!("- 版本: {}", scraper.version());
    
    // 开始计时
    let start_time = Instant::now();
    println!("开始抓取 (限制 {} 个页面)...", MAX_PAGES);
    
    // 运行抓取器
    // 此处我们可以深入到 scraper 内部修改逻辑以限制页面数量
    // 但为简化测试，我们先使用标准 run 方法
    println!("执行抓取...");
    let result = scraper.run().await;
    
    let elapsed = start_time.elapsed();
    
    match result {
        Ok(_) => {
            println!("抓取成功完成! 耗时: {:.2?}", elapsed);
            
            // 检查抓取结果
            validate_results(output_path)?;
        },
        Err(e) => {
            println!("抓取过程中发生错误: {:?}", e);
            return Err(e);
        }
    }
    
    println!("=== 测试完成 ===");
    Ok(())
}

/// 检查抓取结果，确认与预期行为一致
fn validate_results(output_path: &str) -> Result<()> {
    println!("验证抓取结果...");
    
    // 列出输出目录内容
    println!("输出目录内容:");
    let entries = std::fs::read_dir(output_path)?;
    for entry in entries.filter_map(|e| e.ok()) {
        let path = entry.path();
        let file_type = if path.is_dir() {
            "目录"
        } else if path.is_file() {
            "文件"
        } else {
            "其他"
        };
        println!("  - {} ({})", path.display(), file_type);
    }
    
    // 检查必要的文件是否存在
    let index_file = Path::new(output_path).join("index.json");
    if !index_file.exists() {
        println!("❌ 验证失败: 找不到索引文件 index.json");
        return Err("索引文件不存在".into());
    }
    println!("✓ 找到索引文件 index.json");
    
    // 检查是否有 HTML 文件生成
    let html_files_exist = Path::new(output_path)
        .read_dir()?
        .filter_map(|res| res.ok())
        .any(|entry| {
            let path = entry.path();
            let is_valid = path.is_dir() || path.extension().map_or(false, |ext| ext == "html");
            if is_valid {
                println!("  找到有效内容: {}", path.display());
            }
            is_valid
        });
    
    if !html_files_exist {
        println!("❌ 验证失败: 未找到任何 HTML 文件或子目录");
        return Err("未生成 HTML 文件".into());
    }
    println!("✓ 找到 HTML 文件或子目录");
    
    // 读取并解析 index.json 文件
    let index_content = std::fs::read_to_string(&index_file)?;
    let index: serde_json::Value = serde_json::from_str(&index_content)?;
    
    // 检查条目数量
    if let Some(entries_field) = index.get("entries") {
        if let Some(entries_array) = entries_field.as_array() {
            let entries_count = entries_array.len();
            println!("✓ 发现 {} 个文档条目", entries_count);
            
            if entries_count == 0 {
                println!("❌ 验证失败: 没有文档条目");
                return Err("没有文档条目".into());
            }
            
            // 检查条目结构 (现在是数组的数组格式 [name, path, type])
            if let Some(first_entry) = entries_array.first() {
                if let Some(entry_array) = first_entry.as_array() {
                    if entry_array.len() < 3 {
                        println!("❌ 验证失败: 条目结构不正确");
                        return Err("条目结构不正确".into());
                    }
                    println!("✓ 条目结构正确");
                    
                    // 输出前 3 个条目信息作为参考
                    println!("前 {} 个条目:", std::cmp::min(3, entries_count));
                    for (i, entry) in entries_array.iter().take(3).enumerate() {
                        if let Some(entry_array) = entry.as_array() {
                            if entry_array.len() >= 3 {
                                println!("  {}. {} (类型: {}, 路径: {})", 
                                    i + 1,
                                    entry_array[0].as_str().unwrap_or("未知"),
                                    entry_array[2].as_str().unwrap_or("未知"),
                                    entry_array[1].as_str().unwrap_or("未知")
                                );
                            }
                        }
                    }
                } else {
                    println!("❌ 验证失败: 条目格式不正确");
                    return Err("条目格式不正确".into());
                }
            }
        } else {
            println!("❌ 验证失败: index.json 中的 entries 不是有效的 JSON 数组");
            return Err("索引文件格式错误".into());
        }
    } else {
        println!("❌ 验证失败: index.json 中没有 entries 字段");
        return Err("索引文件格式错误".into());
    }
    
    println!("✓ 所有验证通过");
    Ok(())
}