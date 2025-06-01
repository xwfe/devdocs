// filepath: /Users/bing/Projects/devdocs_rust/src/core/tests/response_test.rs
//! Response 模块测试
//!
//! 参考原始 Ruby 项目中的 response_test.rb 实现
//! 为 HTTP 响应提供单元测试

use crate::core::response::Response;
use crate::core::url::DocUrl;
use mockito::Server;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_creation() {
        // 设置测试服务器
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body("Test content")
            .expect(1);

        // 使用 reqwest 创建请求
        let client = reqwest::blocking::Client::new();
        let req = client.get(server.url()).send().unwrap();

        // 创建 Response 对象
        let url = DocUrl::parse(&server.url()).unwrap();
        let response = Response::from_reqwest(req, &url).unwrap();

        // 验证响应信息
        assert_eq!(response.code, 200);
        assert_eq!(response.body, "Test content");
        assert_eq!(response.content_type(), Some("text/plain"));
        assert_eq!(response.url.to_string(), server.url() + "/");

        mock.assert();
    }

    #[test]
    fn test_response_headers() {
        // 设置测试服务器
        let mut server = Server::new();
        let mock = server
            .mock("GET", "/headers")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_header("x-custom-header", "custom_value")
            .with_body("{}")
            .expect(1);

        // 使用 reqwest 创建请求
        let client = reqwest::blocking::Client::new();
        let url = format!("{}{}", server.url(), "/headers");
        let req = client.get(&url).send().unwrap();

        // 创建 Response 对象
        let doc_url = DocUrl::parse(&url).unwrap();
        let response = Response::from_reqwest(req, &doc_url).unwrap();

        // 验证响应头
        assert_eq!(response.content_type(), Some("application/json"));
        assert_eq!(
            response.headers.get("x-custom-header").unwrap(),
            "custom_value"
        );

        mock.assert();
    }

    #[test]
    fn test_response_redirects() {
        // 设置测试服务器
        let mut server = Server::new();

        // 设置重定向
        let redirect_mock = server
            .mock("GET", "/redirect")
            .with_status(302)
            .with_header("location", "/target")
            .expect(1);

        let target_mock = server
            .mock("GET", "/target")
            .with_status(200)
            .with_body("Target content")
            .expect(1);

        // 使用 reqwest 创建请求（自动跟随重定向）
        let client = reqwest::blocking::Client::builder()
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()
            .unwrap();

        let redirect_url = format!("{}{}", server.url(), "/redirect");
        let req = client.get(&redirect_url).send().unwrap();

        // 创建 Response 对象
        let doc_url = DocUrl::parse(&redirect_url).unwrap();
        let response = Response::from_reqwest(req, &doc_url).unwrap();

        // 验证最终响应
        assert_eq!(response.code, 200);
        assert_eq!(response.body, "Target content");

        // URL 应该是最终目标（根据 reqwest 的行为）
        assert!(response.url.to_string().ends_with("/target"));

        redirect_mock.assert();
        target_mock.assert();
    }
}
