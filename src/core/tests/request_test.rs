//! Request 模块测试
//!
//! 参考原始 Ruby 项目中的 request_test.rb 实现
//! 为 HTTP 请求提供单元测试

use crate::core::request::{Request, RequestOptions};
use crate::core::url::DocUrl;
use mockito::Server;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_creation() {
        // 创建基本请求
        let request = Request::new("https://example.com", None).unwrap();
        assert_eq!(request.get_url().to_string(), "https://example.com/");
    }

    #[test]
    fn test_request_with_options() {
        // 创建自定义选项的请求
        let mut options = RequestOptions::default();
        options.follow_redirects = false;
        options
            .headers
            .insert("X-Test".to_string(), "test_value".to_string());

        let request = Request::new("https://example.com", Some(options)).unwrap();
        assert_eq!(request.get_options().follow_redirects, false);
        assert_eq!(
            request.get_options().headers.get("X-Test"),
            Some(&"test_value".to_string())
        );
    }

    #[test]
    fn test_request_run() {
        // 设置 mock 服务器
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/html")
            .with_body("<html><body>Hello world!</body></html>")
            .expect(1);

        // 创建并运行请求
        let request_url = server.url();
        let request = Request::new(&request_url, None).unwrap();
        let response = request.run();

        // 验证请求成功
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.body, "<html><body>Hello world!</body></html>");
        assert_eq!(response.content_type(), Some("text/html"));

        // 验证 mock 被调用
        mock.assert();
    }

    #[test]
    fn test_request_error() {
        // 无效的 URL 应该失败
        let result = Request::new("invalid-url", None);
        assert!(result.is_err());

        // 不存在的服务器应该超时或产生连接错误
        let request = Request::new("http://nonexistent.example.com", None).unwrap();
        let response = request.run();
        assert!(response.is_err());
    }

    #[test]
    fn test_request_static_run() {
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/static")
            .with_status(200)
            .with_body("Static content")
            .expect(1);

        // 使用静态方法直接运行请求
        let url = format!("{}{}", server.url(), "/static");
        let response = Request::run_once(&url, None);

        // 验证请求成功
        assert!(response.is_ok());
        let response = response.unwrap();
        assert_eq!(response.code, 200);
        assert_eq!(response.body, "Static content");

        // 验证 mock 被调用
        mock.assert();
    }
}
