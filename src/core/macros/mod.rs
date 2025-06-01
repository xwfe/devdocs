//! 核心宏定义模块
//!
//! 这个模块包含项目中使用的各种宏定义

// 重新导出宏定义
pub mod scraper_macros;
pub mod doc_macros;

// 导出所有宏
pub use scraper_macros::*;
pub use doc_macros::*;
