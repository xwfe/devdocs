//! URL 辅助工具模块
//!
//! 提供处理 URL 的辅助函数

use crate::core::error::{Error, Result};
use url::Url;

/// 检查 URL 是否为相对 URL
pub fn is_relative_url(url: &str) -> bool {
    !url.contains(':') && !url.starts_with('#') && !url.starts_with("data:")
}

/// 检查 URL 是否为绝对 URL
pub fn is_absolute_url(url: &str) -> bool {
    url.contains(':')
}

/// 检查 URL 是否为片段 URL
pub fn is_fragment_url(url: &str) -> bool {
    url.starts_with('#')
}

/// 检查 URL 是否为数据 URL
pub fn is_data_url(url: &str) -> bool {
    url.starts_with("data:")
}

/// 将相对 URL 转换为绝对 URL
pub fn to_absolute_url(url: &str, base_url: &str) -> Result<String> {
    if is_relative_url(url) {
        match Url::parse(base_url) {
            Ok(base) => {
                match base.join(url) {
                    Ok(absolute) => Ok(absolute.to_string()),
                    Err(e) => Err(Error::Message(format!("无法将 URL 转换为绝对 URL: {}", e)).into())
                }
            },
            Err(e) => Err(Error::Message(format!("无效的基础 URL: {}", e)).into())
        }
    } else {
        Ok(url.to_string())
    }
}

/// 规范化 URL 路径
pub fn normalize_path(path: &str) -> String {
    // 移除 URL 中的特殊字符
    path.replace(['!', ';', ':'], "-")
        .replace('+', "_plus_")
        .replace(['?', '&'], "_")
}

/// 规范化 URL
pub fn normalize_url(url: &str) -> Result<String> {
    match Url::parse(url) {
        Ok(mut parsed_url) => {
            // 移除查询参数和片段
            parsed_url.set_query(None);
            parsed_url.set_fragment(None);
            
            // 规范化路径
            let path = parsed_url.path();
            let normalized_path = normalize_path(path);
            
            // 如果路径被修改，更新 URL
            if normalized_path != path {
                parsed_url.set_path(&normalized_path);
            }
            
            Ok(parsed_url.to_string())
        },
        Err(e) => Err(Error::Message(format!("无法解析 URL: {}", e)).into())
    }
}

/// 获取 URL 的域名
pub fn get_domain(url: &str) -> Result<String> {
    match Url::parse(url) {
        Ok(parsed_url) => {
            if let Some(domain) = parsed_url.host_str() {
                Ok(domain.to_string())
            } else {
                Err(Error::Message("URL 没有域名".to_string()).into())
            }
        },
        Err(e) => Err(Error::Message(format!("无法解析 URL: {}", e)).into())
    }
}

/// 检查 URL 是否为内部 URL
pub fn is_internal_url(url: &str, base_domain: &str) -> Result<bool> {
    let domain = get_domain(url)?;
    Ok(domain == base_domain || domain.ends_with(&format!(".{}", base_domain)))
}

/// 从 URL 中提取路径
pub fn extract_path(url: &str) -> Result<String> {
    match Url::parse(url) {
        Ok(parsed_url) => {
            let path = parsed_url.path();
            Ok(path.to_string())
        },
        Err(e) => Err(Error::Message(format!("无法解析 URL: {}", e)).into())
    }
}

/// 从 URL 中提取文件名
pub fn extract_filename(url: &str) -> Result<String> {
    let path = extract_path(url)?;
    let segments: Vec<&str> = path.split('/').collect();
    if let Some(last) = segments.last() {
        if !last.is_empty() {
            return Ok(last.to_string());
        }
    }
    Err(Error::Message("URL 没有文件名".to_string()).into())
}

/// 从 URL 中提取扩展名
pub fn extract_extension(url: &str) -> Result<String> {
    let filename = extract_filename(url)?;
    let segments: Vec<&str> = filename.split('.').collect();
    if let Some(last) = segments.last() {
        if segments.len() > 1 {
            return Ok(last.to_string());
        }
    }
    Err(Error::Message("URL 没有扩展名".to_string()).into())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_is_relative_url() {
        assert!(is_relative_url("page.html"));
        assert!(is_relative_url("/page.html"));
        assert!(!is_relative_url("http://example.com/page.html"));
        assert!(!is_relative_url("#section"));
        assert!(!is_relative_url("data:image/png;base64,iVBORw0KGg"));
    }
    
    #[test]
    fn test_to_absolute_url() {
        let base_url = "https://example.com/docs/";
        
        assert_eq!(
            to_absolute_url("page.html", base_url).unwrap(),
            "https://example.com/docs/page.html"
        );
        
        assert_eq!(
            to_absolute_url("/page.html", base_url).unwrap(),
            "https://example.com/page.html"
        );
        
        assert_eq!(
            to_absolute_url("http://other.com/page.html", base_url).unwrap(),
            "http://other.com/page.html"
        );
    }
    
    #[test]
    fn test_normalize_path() {
        assert_eq!(normalize_path("path/to/file!with:special;chars"), "path/to/file-with-special-chars");
        assert_eq!(normalize_path("path/with+plus"), "path/with_plus_plus");
        assert_eq!(normalize_path("path/with?query&params"), "path/with_query_params");
    }
    
    #[test]
    fn test_normalize_url() {
        assert_eq!(
            normalize_url("https://example.com/path/to/file?query=1#section").unwrap(),
            "https://example.com/path/to/file"
        );
        
        assert_eq!(
            normalize_url("https://example.com/path/with+plus").unwrap(),
            "https://example.com/path/with_plus_plus"
        );
    }
}
