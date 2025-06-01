//! URL 模块
//!
//! 严格对齐原版 Ruby 项目中的 core/url.rb 实现

use crate::core::error::{Error, Result};
use std::collections::HashMap;
use std::fmt;

/// DocUrl 类型别名，对应原版中的 URL 类
pub type DocUrl = URL;

/// URL 解析器
#[derive(Debug, Clone, PartialEq)]
pub struct URLParser;

impl URLParser {
    /// 分割 URL
    ///
    /// 对应原版 Ruby 的 PARSER.split 方法
    pub fn split(&self, url: &str) -> Vec<Option<String>> {
        // 简化的 URL 分割实现
        // 返回 [scheme, userinfo, host, port, registry, path, opaque, query, fragment]
        let mut parts = vec![None; 9];

        if let Ok(parsed) = url::Url::parse(url) {
            parts[0] = Some(parsed.scheme().to_string());
            parts[2] = parsed.host_str().map(|h| h.to_string());
            if let Some(port) = parsed.port() {
                parts[3] = Some(port.to_string());
            }
            parts[5] = Some(parsed.path().to_string());
            parts[7] = parsed.query().map(|q| q.to_string());
            parts[8] = parsed.fragment().map(|f| f.to_string());
        }

        parts
    }

    /// 连接 URL
    ///
    /// 对应原版 Ruby 的 PARSER.join 方法
    pub fn join(&self, base: &str, relative: &str) -> String {
        if let Ok(base_url) = url::Url::parse(base) {
            if let Ok(joined) = base_url.join(relative) {
                return joined.to_string();
            }
        }
        relative.to_string()
    }
}

/// URL 类
///
/// 对应原版 Ruby 的 URL 类
#[derive(Debug, Clone, PartialEq)]
pub struct URL {
    scheme: Option<String>,
    userinfo: Option<String>,
    host: Option<String>,
    port: Option<u16>,
    registry: Option<String>,
    path: String,
    opaque: Option<String>,
    query: Option<String>,
    fragment: Option<String>,
    parser: URLParser,
}

impl URL {
    /// 创建新的 URL
    ///
    /// 对应原版 Ruby 的 initialize 方法
    pub fn new(
        scheme: Option<String>,
        userinfo: Option<String>,
        host: Option<String>,
        port: Option<u16>,
        registry: Option<String>,
        path: String,
        opaque: Option<String>,
        query: Option<String>,
        fragment: Option<String>,
    ) -> Self {
        Self {
            scheme,
            userinfo,
            host,
            port,
            registry,
            path,
            opaque,
            query,
            fragment,
            parser: URLParser,
        }
    }

    /// 从哈希创建 URL
    pub fn from_hash(hash: HashMap<String, String>) -> Self {
        Self {
            scheme: hash.get("scheme").cloned(),
            userinfo: hash.get("userinfo").cloned(),
            host: hash.get("host").cloned(),
            port: hash.get("port").and_then(|p| p.parse().ok()),
            registry: hash.get("registry").cloned(),
            path: hash.get("path").cloned().unwrap_or_default(),
            opaque: hash.get("opaque").cloned(),
            query: hash.get("query").cloned(),
            fragment: hash.get("fragment").cloned(),
            parser: URLParser,
        }
    }

    /// 解析 URL 字符串
    ///
    /// 对应原版 Ruby 的 parse 方法
    pub fn parse(url: &str) -> Result<Self> {
        if let Ok(parsed) = url::Url::parse(url) {
            Ok(Self {
                scheme: Some(parsed.scheme().to_string()),
                userinfo: None,
                host: parsed.host_str().map(|h| h.to_string()),
                port: parsed.port(),
                registry: None,
                path: parsed.path().to_string(),
                opaque: None,
                query: parsed.query().map(|q| q.to_string()),
                fragment: parsed.fragment().map(|f| f.to_string()),
                parser: URLParser,
            })
        } else {
            Err(Error::InvalidUrl(url.to_string()))
        }
    }

    /// 连接 URL
    ///
    /// 对应原版 Ruby 的 join 方法
    pub fn join(&self, relative: &str) -> Self {
        let base = self.to_string();
        let joined = self.parser.join(&base, relative);
        Self::parse(&joined).unwrap_or_else(|_| self.clone())
    }

    /// 合并哈希参数
    ///
    /// 对应原版 Ruby 的 merge! 方法
    pub fn merge(&mut self, hash: HashMap<String, String>) {
        if let Some(scheme) = hash.get("scheme") {
            self.scheme = Some(scheme.clone());
        }
        if let Some(userinfo) = hash.get("userinfo") {
            self.userinfo = Some(userinfo.clone());
        }
        if let Some(host) = hash.get("host") {
            self.host = Some(host.clone());
        }
        if let Some(port) = hash.get("port") {
            self.port = port.parse().ok();
        }
        if let Some(registry) = hash.get("registry") {
            self.registry = Some(registry.clone());
        }
        if let Some(path) = hash.get("path") {
            self.path = path.clone();
        }
        if let Some(opaque) = hash.get("opaque") {
            self.opaque = Some(opaque.clone());
        }
        if let Some(query) = hash.get("query") {
            self.query = Some(query.clone());
        }
        if let Some(fragment) = hash.get("fragment") {
            self.fragment = Some(fragment.clone());
        }
    }

    /// 获取 origin
    ///
    /// 对应原版 Ruby 的 origin 方法
    pub fn origin(&self) -> Option<String> {
        if let (Some(scheme), Some(host)) = (&self.scheme, &self.host) {
            let mut origin = format!("{}://{}", scheme.to_lowercase(), host);
            if let Some(port) = self.port {
                origin.push_str(&format!(":{}", port));
            }
            Some(origin)
        } else {
            None
        }
    }

    /// 获取规范化路径
    ///
    /// 对应原版 Ruby 的 normalized_path 方法
    pub fn normalized_path(&self) -> String {
        if self.path.is_empty() {
            "/".to_string()
        } else {
            self.path.clone()
        }
    }

    /// 获取到指定 URL 的子路径
    ///
    /// 对应原版 Ruby 的 subpath_to 方法
    pub fn subpath_to(&self, url: &URL, ignore_case: Option<bool>) -> Option<String> {
        if self.origin() != url.origin() {
            return None;
        }

        let mut base = self.path.clone();
        let mut dest = url.path.clone();

        if ignore_case.unwrap_or(false) {
            base = base.to_lowercase();
            dest = dest.to_lowercase();
        }

        if base == dest {
            Some(String::new())
        } else if dest.starts_with(&format!("{}/", base))
            || (base.ends_with('/') && dest.starts_with(&base))
        {
            let start_pos = if base.ends_with('/') {
                base.len()
            } else {
                base.len() + 1
            };
            Some(url.path[start_pos..].to_string())
        } else {
            None
        }
    }

    /// 获取从指定 URL 的子路径
    ///
    /// 对应原版 Ruby 的 subpath_from 方法
    pub fn subpath_from(&self, url: &URL, ignore_case: Option<bool>) -> Option<String> {
        url.subpath_to(self, ignore_case)
    }

    /// 检查是否包含指定 URL
    ///
    /// 对应原版 Ruby 的 contains? 方法
    pub fn contains(&self, url: &URL, ignore_case: Option<bool>) -> bool {
        self.subpath_to(url, ignore_case).is_some()
    }

    /// 获取到指定 URL 的相对路径
    ///
    /// 对应原版 Ruby 的 relative_path_to 方法
    pub fn relative_path_to(&self, url: &URL) -> Option<String> {
        if self.origin() != url.origin() {
            return None;
        }

        // 简化实现
        let base_path = &self.normalized_path();
        let dest_path = &url.normalized_path();

        if dest_path.ends_with('/') {
            // 目录路径
            Some(format!(
                "./{}",
                dest_path
                    .trim_start_matches(base_path)
                    .trim_start_matches('/')
            ))
        } else {
            // 文件路径
            Some(dest_path.split('/').last().unwrap_or("").to_string())
        }
    }

    /// 获取从指定 URL 的相对路径
    ///
    /// 对应原版 Ruby 的 relative_path_from 方法
    pub fn relative_path_from(&self, url: &URL) -> Option<String> {
        url.relative_path_to(self)
    }

    // Getter 方法
    pub fn scheme(&self) -> Option<&String> {
        self.scheme.as_ref()
    }

    pub fn host(&self) -> Option<&String> {
        self.host.as_ref()
    }

    pub fn port(&self) -> Option<u16> {
        self.port
    }

    pub fn path(&self) -> &String {
        &self.path
    }

    pub fn query(&self) -> Option<&String> {
        self.query.as_ref()
    }

    pub fn fragment(&self) -> Option<&String> {
        self.fragment.as_ref()
    }

    // Setter 方法
    pub fn set_scheme(&mut self, scheme: Option<String>) {
        self.scheme = scheme;
    }

    pub fn set_host(&mut self, host: Option<String>) {
        self.host = host;
    }

    pub fn set_port(&mut self, port: Option<u16>) {
        self.port = port;
    }

    pub fn set_path(&mut self, path: String) {
        self.path = path;
    }
}

impl fmt::Display for URL {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut url = String::new();

        if let Some(scheme) = &self.scheme {
            url.push_str(scheme);
            url.push_str("://");
        }

        if let Some(host) = &self.host {
            url.push_str(host);
            if let Some(port) = self.port {
                url.push_str(&format!(":{}", port));
            }
        }

        url.push_str(&self.path);

        if let Some(query) = &self.query {
            url.push('?');
            url.push_str(query);
        }

        if let Some(fragment) = &self.fragment {
            url.push('#');
            url.push_str(fragment);
        }

        write!(f, "{}", url)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let url = URL::parse("https://example.com:8080/path?query=value#fragment");
        assert_eq!(url.scheme(), Some(&"https".to_string()));
        assert_eq!(url.host(), Some(&"example.com".to_string()));
        assert_eq!(url.port(), Some(8080));
        assert_eq!(url.path(), "/path");
        assert_eq!(url.query(), Some(&"query=value".to_string()));
        assert_eq!(url.fragment(), Some(&"fragment".to_string()));
    }

    #[test]
    fn test_origin() {
        let url = URL::parse("https://example.com:8080/path");
        assert_eq!(url.origin(), Some("https://example.com:8080".to_string()));

        let url2 = URL::parse("https://example.com/path");
        assert_eq!(url2.origin(), Some("https://example.com".to_string()));
    }

    #[test]
    fn test_normalized_path() {
        let url = URL::parse("https://example.com");
        assert_eq!(url.normalized_path(), "/");

        let url2 = URL::parse("https://example.com/path");
        assert_eq!(url2.normalized_path(), "/path");
    }

    #[test]
    fn test_subpath_to() {
        let base = URL::parse("https://example.com/docs");
        let target = URL::parse("https://example.com/docs/api/reference");

        assert_eq!(
            base.subpath_to(&target, None),
            Some("/api/reference".to_string())
        );
    }

    #[test]
    fn test_contains() {
        let base = URL::parse("https://example.com/docs");
        let target = URL::parse("https://example.com/docs/api");

        assert!(base.contains(&target, None));
    }

    #[test]
    fn test_join() {
        let base = URL::parse("https://example.com/docs/");
        let joined = base.join("api/reference.html");

        assert_eq!(
            joined.to_string(),
            "https://example.com/docs/api/reference.html"
        );
    }

    #[test]
    fn test_merge() {
        let mut url = URL::parse("https://example.com/path");
        let mut hash = HashMap::new();
        hash.insert("host".to_string(), "newhost.com".to_string());
        hash.insert("path".to_string(), "/newpath".to_string());

        url.merge(hash);
        assert_eq!(url.host(), Some(&"newhost.com".to_string()));
        assert_eq!(url.path(), "/newpath");
    }

    #[test]
    fn test_to_string() {
        let url = URL::parse("https://example.com:8080/path?query=value#fragment");
        let url_string = url.to_string();

        assert!(url_string.contains("https://"));
        assert!(url_string.contains("example.com"));
        assert!(url_string.contains(":8080"));
        assert!(url_string.contains("/path"));
        assert!(url_string.contains("?query=value"));
        assert!(url_string.contains("#fragment"));
    }
}
