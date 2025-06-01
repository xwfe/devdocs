//! Requester 模块测试
//!
//! 参考原始 Ruby 项目中的 requester_test.rb 实现
//! 为 HTTP 请求器提供单元测试

use crate::core::request::RequestOptions;
use crate::core::requester::Requester;
use crate::core::response::Response;
use mockito::Server;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_mock(
        server: &mut Server,
        path: &str,
        status: usize,
        body: &str,
        headers: Option<Vec<(&str, &str)>>,
    ) -> mockito::Mock {
        let mut m = server.mock("GET", path).with_status(status).with_body(body);

        if let Some(headers) = headers {
            for (key, value) in headers {
                m = m.with_header(key, value);
            }
        }

        m.expect(1)
    }

    #[test]
    fn test_requester_creation() {
        // 测试默认参数
        let requester = Requester::new(None, None);
        assert_eq!(requester.max_concurrency(), 20); // 默认最大并发数

        // 测试自定义参数
        let options = RequestOptions::default();
        let requester = Requester::new(Some(10), Some(options));
        assert_eq!(requester.max_concurrency(), 10); // 自定义最大并发数
    }

    #[test]
    fn test_requester_on_response() {
        let mut requester = Requester::new(None, None);
        let called = Arc::new(Mutex::new(false));

        let c = called.clone();
        requester.on_response(move |_response| {
            *c.lock().unwrap() = true;
            None
        });

        // 验证回调被添加
        assert_eq!(requester.on_response_count(), 1);
    }

    #[test]
    fn test_requester_run_single_url() {
        let mut server = Server::new();
        let path = "/single";
        let full_url = format!("{}{}", server.url(), path);

        // 设置单个请求的 mock
        let _m = setup_mock(
            &mut server,
            path,
            200,
            "Test content",
            Some(vec![("Content-Type", "text/plain")]),
        );

        let processed_urls = Arc::new(Mutex::new(HashSet::new()));
        let p_urls = processed_urls.clone();

        // 运行请求器
        let requester = Requester::run(vec![full_url.clone()], Some(5), None, move |response| {
            assert_eq!(response.code, 200);
            assert_eq!(response.body, "Test content");
            p_urls.lock().unwrap().insert(response.url.to_string());
            None
        })
        .unwrap();

        // 验证回调被调用
        assert_eq!(requester.on_response_count(), 1);
        assert!(processed_urls.lock().unwrap().contains(&full_url));
    }

    #[test]
    fn test_requester_run_multiple_urls() {
        let mut server = Server::new();

        // 设置多个请求的 mock
        let _m1 = setup_mock(
            &mut server,
            "/first",
            200,
            "First content",
            Some(vec![("Content-Type", "text/plain")]),
        );

        let _m2 = setup_mock(
            &mut server,
            "/second",
            200,
            "Second content",
            Some(vec![("Content-Type", "text/plain")]),
        );

        let first_url = format!("{}{}", server.url(), "/first");
        let second_url = format!("{}{}", server.url(), "/second");

        let processed_urls = Arc::new(Mutex::new(HashSet::new()));
        let p_urls = processed_urls.clone();

        // 运行请求器
        let _requester = Requester::run(
            vec![first_url.clone(), second_url.clone()],
            Some(2),
            None,
            move |response| {
                p_urls.lock().unwrap().insert(response.url.to_string());
                None
            },
        )
        .unwrap();

        // 验证两个 URL 都被处理
        let processed = processed_urls.lock().unwrap();
        assert!(processed.contains(&first_url));
        assert!(processed.contains(&second_url));
    }

    #[test]
    fn test_requester_with_new_urls() {
        let mut server = Server::new();

        // 设置多个请求的 mock
        let _m1 = setup_mock(
            &mut server,
            "/source",
            200,
            "Source content",
            Some(vec![("Content-Type", "text/plain")]),
        );

        let _m2 = setup_mock(
            &mut server,
            "/linked",
            200,
            "Linked content",
            Some(vec![("Content-Type", "text/plain")]),
        );

        let source_url = format!("{}{}", server.url(), "/source");
        let linked_url = format!("{}{}", server.url(), "/linked");

        // 关键: 将这两个 URL 存储起来，以便在测试验证时使用
        let source_url_for_assert = source_url.clone();
        let linked_url_for_assert = linked_url.clone();

        let processed_urls = Arc::new(Mutex::new(HashSet::new()));
        let p_urls = processed_urls.clone();

        // 运行请求器，在回调中添加新的 URL
        let _requester = Requester::run(vec![source_url.clone()], Some(1), None, move |response| {
            let mut urls = p_urls.lock().unwrap();
            urls.insert(response.url.to_string());

            // 返回新的 URL，模拟从源页面提取链接
            if response.url.to_string() == source_url {
                Some(vec![linked_url.clone()])
            } else {
                None
            }
        })
        .unwrap();

        // 验证两个 URL 都被处理（包括回调中添加的链接）
        let processed = processed_urls.lock().unwrap();
        assert!(processed.contains(&source_url_for_assert));
        assert!(processed.contains(&linked_url_for_assert));
    }

    #[test]
    fn test_requester_error_handling() {
        let mut server = Server::new();

        // 设置一个失败的请求
        let path = "/error";
        let _m = setup_mock(&mut server, path, 404, "Not Found", None);

        let full_url = format!("{}{}", server.url(), path);
        let processed_urls = Arc::new(Mutex::new(HashSet::new()));
        let p_urls = processed_urls.clone();

        // 即使请求失败，回调也应该被执行
        let _requester = Requester::run(vec![full_url.clone()], None, None, move |response| {
            p_urls.lock().unwrap().insert(response.url.to_string());
            assert_eq!(response.code, 404);
            assert_eq!(response.body, "Not Found");
            None
        })
        .unwrap();

        // 验证失败的 URL 也被处理了
        let processed = processed_urls.lock().unwrap();
        assert!(processed.contains(&full_url));
    }

    #[test]
    fn test_get_next_url() {
        let queue: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(vec![
            "http://example.com/1".to_string(),
            "http://example.com/2".to_string(),
            "http://example.com/3".to_string(),
        ]));

        let processed: Arc<Mutex<HashMap<String, bool>>> = Arc::new(Mutex::new(HashMap::new()));

        // 第一个 URL 应该被处理
        let url1 = Requester::get_next_url_for_test(&queue, &processed);
        assert_eq!(url1, Some("http://example.com/1".to_string()));

        // 标记第一个和第三个 URL 为已处理
        processed
            .lock()
            .unwrap()
            .insert("http://example.com/1".to_string(), true);
        processed
            .lock()
            .unwrap()
            .insert("http://example.com/3".to_string(), true);

        // 下一个未处理的 URL 应该是第二个
        let url2 = Requester::get_next_url_for_test(&queue, &processed);
        assert_eq!(url2, Some("http://example.com/2".to_string()));

        // 标记第二个 URL 为已处理
        processed
            .lock()
            .unwrap()
            .insert("http://example.com/2".to_string(), true);

        // 所有 URL 都已处理
        let url3 = Requester::get_next_url_for_test(&queue, &processed);
        assert_eq!(url3, None);
    }
}
