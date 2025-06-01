use devdocs_rust::docs::babel::test_simplified_scraper;
use std::process;

fn main() {
    println!("测试运行简化版 Babel 抓取器...");
    
    // 运行测试
    if let Err(e) = test_simplified_scraper() {
        eprintln!("错误: {}", e);
        process::exit(1);
    }
    
    println!("测试完成!");
}
