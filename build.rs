//! 构建脚本 - 简化版本
//!
//! 提供编译时配置和版本信息

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // 获取基本构建信息
    let build_time = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let git_hash = get_git_hash().unwrap_or_else(|| "unknown".to_string());
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_string());
    let profile = env::var("PROFILE").unwrap_or_else(|_| "debug".to_string());
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    
    // 设置构建时常量
    println!("cargo:rustc-env=BUILD_TIME={}", build_time);
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);
    println!("cargo:rustc-env=BUILD_TARGET={}", target);
    println!("cargo:rustc-env=BUILD_PROFILE={}", profile);
    println!("cargo:rustc-env=BUILD_VERSION={}", version);
    
    // 检查特性配置
    check_features();
    
    // 生成构建信息模块
    generate_build_info();
    
    // 重新构建触发条件
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=Cargo.toml");
}

/// 获取 Git 提交哈希
fn get_git_hash() -> Option<String> {
    let output = Command::new("git")
        .args(&["rev-parse", "--short", "HEAD"])
        .output()
        .ok()?;
    
    if output.status.success() {
        Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        None
    }
}

/// 检查和配置特性
fn check_features() {
    if cfg!(feature = "simd") {
        println!("cargo:rustc-cfg=simd_enabled");
    }
    
    if cfg!(feature = "compression") {
        println!("cargo:rustc-cfg=compression_enabled");
    }
    
    if cfg!(feature = "templates") {
        println!("cargo:rustc-cfg=templates_enabled");
    }
    
    if cfg!(feature = "xpath") {
        println!("cargo:rustc-cfg=xpath_enabled");
    }
    
    if cfg!(feature = "cache") {
        println!("cargo:rustc-cfg=cache_enabled");
    }
}

/// 生成构建信息模块
fn generate_build_info() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let build_info_path = Path::new(&out_dir).join("build_info.rs");
    
    let build_time = env::var("BUILD_TIME").unwrap_or_else(|_| "0".to_string());
    let git_hash = env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string());
    let target = env::var("BUILD_TARGET").unwrap_or_else(|_| "unknown".to_string());
    let profile = env::var("BUILD_PROFILE").unwrap_or_else(|_| "debug".to_string());
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.1.0".to_string());
    
    let build_info = format!(
        r#"//! 构建时生成的信息模块

/// 构建时间戳
pub const BUILD_TIME: &str = "{}";

/// Git 提交哈希
pub const GIT_HASH: &str = "{}";

/// 构建目标
pub const BUILD_TARGET: &str = "{}";

/// 构建配置
pub const BUILD_PROFILE: &str = "{}";

/// 版本信息
pub const VERSION: &str = "{}";

/// 完整版本字符串
pub fn full_version() -> String {{
    format!("{{}} ({{}})", VERSION, GIT_HASH)
}}

/// 特性信息
pub fn feature_info() -> Vec<&'static str> {{
    let mut features = Vec::new();
    
    #[cfg(feature = "simd")]
    features.push("simd");
    
    #[cfg(feature = "compression")]
    features.push("compression");
    
    #[cfg(feature = "templates")]
    features.push("templates");
    
    #[cfg(feature = "xpath")]
    features.push("xpath");
    
    #[cfg(feature = "cache")]
    features.push("cache");
    
    features
}}

/// 运行时信息
pub fn runtime_info() -> String {{
    format!(
        "xwdoc {{}} built on {{}} for {{}} ({{}})\\nFeatures: {{}}",
        VERSION,
        BUILD_TIME,
        BUILD_TARGET,
        BUILD_PROFILE,
        feature_info().join(", ")
    )
}}
"#,
        build_time, git_hash, target, profile, version
    );
    
    fs::write(&build_info_path, build_info)
        .expect("Failed to write build info");
}