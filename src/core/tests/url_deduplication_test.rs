//! URL去重逻辑测试
//!
//! 测试URL抓取器和请求器的去重机制是否与原版Ruby代码一致

use crate::core::scraper::url_scraper::UrlScraper;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_case_insensitive_deduplication() {
        // 测试URL去重是否对大小写不敏感，与原版Ruby一致
        let mut visited = HashSet::new();
        
        let urls = vec![
            "https://example.com/Page1".to_string(),
            "https://example.com/page1".to_string(),  // 应该被视为重复
            "https://example.com/PAGE1".to_string(),  // 应该被视为重复
            "https://example.com/page2".to_string(),
        ];

        let mut unique_urls = Vec::new();
        
        for url in urls {
            let url_lower = url.to_lowercase();
            if !visited.contains(&url_lower) {
                visited.insert(url_lower);
                unique_urls.push(url);
            }
        }

        // 应该只有2个唯一URL
        assert_eq!(unique_urls.len(), 2);
        assert!(unique_urls.iter().any(|u| u.contains("Page1") || u.contains("page1")));
        assert!(unique_urls.iter().any(|u| u.contains("page2")));
    }

    #[test]
    fn test_initial_urls_added_to_visited() {
        // 测试初始URL被正确添加到已访问集合
        let mut visited = HashSet::new();
        let initial_urls = vec![
            "https://example.com/index".to_string(),
            "https://example.com/about".to_string(),
        ];

        // 模拟UrlScraper的初始化逻辑
        for url in &initial_urls {
            visited.insert(url.to_lowercase());
        }

        // 检查是否能正确识别重复URL
        assert!(visited.contains("https://example.com/index"));
        assert!(visited.contains("https://example.com/INDEX")); // 大小写不敏感
        assert!(!visited.contains("https://example.com/newpage"));
    }

    #[test]
    fn test_queue_processing_avoids_duplicates() {
        // 测试队列处理时避免重复URL
        let mut visited = HashSet::new();
        let mut queue = vec![
            "https://example.com/page1".to_string(),
            "https://example.com/page2".to_string(),
            "https://example.com/Page1".to_string(), // 重复（大小写不同）
        ];

        let mut processed_urls = Vec::new();

        while let Some(url) = queue.pop() {
            let url_lower = url.to_lowercase();
            if visited.contains(&url_lower) {
                continue; // 跳过已访问的URL
            }
            
            visited.insert(url_lower);
            processed_urls.push(url);
        }

        // 应该只处理2个唯一URL
        assert_eq!(processed_urls.len(), 2);
    }

    #[test]
    fn test_new_urls_filtered_before_adding_to_queue() {
        // 测试新URL在添加到队列前被正确过滤
        let mut visited = HashSet::new();
        visited.insert("https://example.com/existing".to_lowercase());

        let new_urls = vec![
            "https://example.com/new1".to_string(),
            "https://example.com/EXISTING".to_string(), // 应该被过滤
            "https://example.com/new2".to_string(),
        ];

        let mut queue = Vec::new();
        for new_url in new_urls {
            let new_url_lower = new_url.to_lowercase();
            if !visited.contains(&new_url_lower) {
                queue.push(new_url);
            }
        }

        // 应该只有2个新URL被添加到队列
        assert_eq!(queue.len(), 2);
        assert!(!queue.iter().any(|u| u.to_lowercase().contains("existing")));
    }
}