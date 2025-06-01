//! xwdoc - API 文档浏览器的主入口点

use std::error::Error;
use xwdoc::cli::handle_cli;

fn main() -> Result<(), Box<dyn Error>> {
    // 初始化日志
    env_logger::init();
    
    // 显示版本信息
    println!("xwdoc {}", env!("CARGO_PKG_VERSION"));
    
    // 创建异步运行时并处理命令行
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(handle_cli())
}
