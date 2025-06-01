//! HTTP 请求模块
//!
//! 参考原始 Ruby 项目中的 request.rb 实现
//! 提供发送 HTTP 请求的功能

use crate::core::error::{Error, Result};
use crate::core::instrumentable;
use crate::core::response::Response;
use crate::core::url::DocUrl;
use reqwest::{blocking, header};
use std::collections::HashMap;
use std::time::Duration;

/// 默认的用户代理
const DEFAULT_USER_AGENT: &str = "DevDocs Rust";

/// 默认的连接超时时间（秒）
const DEFAULT_CONNECT_TIMEOUT: u64 = 15;

/// HTTP 请求选项
#[derive(Debug, Clone)]
pub struct RequestOptions {
    /// 是否跟随重定向
    pub follow_redirects: bool,
    /// 头部信息
    pub headers: HashMap<String, String>,
    /// 连接超时时间（秒）
    pub connect_timeout: u64,
    /// 请求超时时间（秒）
    pub timeout: Option<u64>,
}

impl Default for RequestOptions {
    fn default() -> Self {
        let mut headers = HashMap::new();
        headers.insert("User-Agent".to_string(), DEFAULT_USER_AGENT.to_string());

        Self {
            follow_redirects: true,
            headers,
            connect_timeout: DEFAULT_CONNECT_TIMEOUT,
            timeout: None,
        }
    }
}

/// HTTP 请求结构体
#[derive(Debug)]
pub struct Request {
    /// 请求的 URL
    url: DocUrl,
    /// 请求选项
    options: RequestOptions,
}

impl Request {
    /// 创建新的 HTTP 请求
    pub fn new(url: &str, options: Option<RequestOptions>) -> Result<Self> {
        let url = DocUrl::parse(url)?;
        let options = options.unwrap_or_default();

        Ok(Self { url, options })
    }

    /// 执行请求并返回响应
    pub fn run(&self) -> Result<Response> {
        let payload = HashMap::from([("url".to_string(), self.url.to_string())]);

        instrumentable::instrument("response.request", payload, || self.execute())
    }

    /// 静态方法，创建并执行请求
    pub fn run_once(url: &str, options: Option<RequestOptions>) -> Result<Response> {
        let request = Self::new(url, options)?;
        request.run()
    }

    /// 执行请求
    fn execute(&self) -> Result<Response> {
        let client = self.build_client()?;
        let response = client
            .get(self.url.to_string())
            .send()
            .map_err(Error::Http)?;

        Response::from_reqwest(response, &self.url)
    }

    /// 构建 HTTP 客户端
    fn build_client(&self) -> Result<blocking::Client> {
        let mut builder = blocking::Client::builder()
            .connect_timeout(Duration::from_secs(self.options.connect_timeout))
            .redirect(if self.options.follow_redirects {
                reqwest::redirect::Policy::limited(10)
            } else {
                reqwest::redirect::Policy::none()
            });

        if let Some(timeout) = self.options.timeout {
            builder = builder.timeout(Duration::from_secs(timeout));
        }

        // 添加头部信息
        let mut headers = header::HeaderMap::new();
        for (key, value) in &self.options.headers {
            if let Ok(header_name) = header::HeaderName::from_bytes(key.as_bytes()) {
                if let Ok(header_value) = header::HeaderValue::from_str(value) {
                    headers.insert(header_name, header_value);
                }
            }
        }

        // 应用所有头部
        builder = builder.default_headers(headers);

        // 构建客户端
        let client = builder.build().map_err(Error::Http)?;
        Ok(client)
    }

    /// 测试用 - 获取请求选项
    #[cfg(test)]
    pub fn get_options(&self) -> &RequestOptions {
        &self.options
    }

    /// 测试用 - 获取 URL
    #[cfg(test)]
    pub fn get_url(&self) -> &DocUrl {
        &self.url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_request() {
        let request = Request::new("https://example.com", None).unwrap();
        assert_eq!(request.url.to_string(), "https://example.com/");
        assert_eq!(request.options.follow_redirects, true);
    }

    #[test]
    fn test_custom_options() {
        let mut options = RequestOptions::default();
        options.follow_redirects = false;
        options
            .headers
            .insert("X-Test".to_string(), "value".to_string());

        let request = Request::new("https://example.com", Some(options)).unwrap();
        assert_eq!(request.options.follow_redirects, false);
        assert_eq!(
            request.options.headers.get("X-Test"),
            Some(&"value".to_string())
        );
    }
}
