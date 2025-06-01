//! 核心模型模块
//!
//! 严格对齐原版 Ruby 项目中的模型定义

pub mod entry;
pub mod type_model;

pub use entry::{Entry, EntryJson, Invalid as EntryInvalid};
pub use type_model::{Type, TypeJson};