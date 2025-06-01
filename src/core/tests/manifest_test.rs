//! Manifest 模块测试
//!
//! 参考原始 Ruby 项目中的 manifest_test.rb 实现
//! 为清单管理提供单元测试

use crate::core::manifest::{DocSpec, Manifest};
use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    fn create_test_doc_spec(slug: &str) -> DocSpec {
        DocSpec {
            name: format!("Test Doc {}", slug),
            slug: slug.to_string(),
            doc_type: "library".to_string(),
            version: Some("1.0.0".to_string()),
            release: Some("stable".to_string()),
            links: Some({
                let mut links = HashMap::new();
                links.insert("home".to_string(), "https://example.com".to_string());
                links
            }),
            mtime: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            db_size: 1024,
        }
    }

    #[test]
    fn test_manifest_creation() {
        let manifest = Manifest::new();
        assert!(manifest.docs().is_empty());
    }

    #[test]
    fn test_add_doc() {
        let mut manifest = Manifest::new();
        let doc = create_test_doc_spec("test");

        manifest.add(doc.clone());
        assert_eq!(manifest.docs().len(), 1);

        let retrieved = manifest.get("test").unwrap();
        assert_eq!(retrieved.name, "Test Doc test");
        assert_eq!(retrieved.slug, "test");
    }

    #[test]
    fn test_remove_doc() {
        let mut manifest = Manifest::new();
        let doc = create_test_doc_spec("test");

        manifest.add(doc.clone());
        assert_eq!(manifest.docs().len(), 1);

        let removed = manifest.remove("test").unwrap();
        assert_eq!(manifest.docs().len(), 0);
        assert_eq!(removed.name, "Test Doc test");
        assert_eq!(removed.slug, "test");
    }

    #[test]
    fn test_from_json() {
        let json = r#"{
            "docs": {
                "test": {
                    "name": "Test Doc",
                    "slug": "test",
                    "type": "library",
                    "version": "1.0.0",
                    "release": "stable",
                    "links": {
                        "home": "https://example.com"
                    },
                    "mtime": 1609459200,
                    "db_size": 1024
                }
            }
        }"#;

        let result = Manifest::from_json(json);
        assert!(result.is_ok());

        let manifest = result.unwrap();
        assert_eq!(manifest.docs().len(), 1);

        let doc = manifest.get("test").unwrap();
        assert_eq!(doc.name, "Test Doc");
        assert_eq!(doc.slug, "test");
        assert_eq!(doc.doc_type, "library");
        assert_eq!(doc.version, Some("1.0.0".to_string()));
        assert_eq!(doc.release, Some("stable".to_string()));
        assert!(doc.links.is_some());
        assert_eq!(doc.links.as_ref().unwrap().get("home"), Some(&"https://example.com".to_string()));
        assert_eq!(doc.mtime, 1609459200);
        assert_eq!(doc.db_size, 1024);
    }

    #[test]
    fn test_to_json() {
        let mut manifest = Manifest::new();
        let doc = create_test_doc_spec("test");

        manifest.add(doc);

        let json_result = manifest.to_json();
        assert!(json_result.is_ok());

        let json = json_result.unwrap();
        assert!(json.contains("\"name\":\"Test Doc test\""));
        assert!(json.contains("\"slug\":\"test\""));
        assert!(json.contains("\"type\":\"library\""));
    }

    #[test]
    fn test_to_json_pretty() {
        let mut manifest = Manifest::new();
        let doc = create_test_doc_spec("test");

        manifest.add(doc);

        let json_result = manifest.to_json_pretty();
        assert!(json_result.is_ok());

        let json = json_result.unwrap();
        assert!(json.contains("\"name\": \"Test Doc test\""));
        assert!(json.contains("\"slug\": \"test\""));
        assert!(json.contains("\"type\": \"library\""));
    }

    #[test]
    fn test_get_mut() {
        let mut manifest = Manifest::new();
        let doc = create_test_doc_spec("test");

        manifest.add(doc);

        // 获取可变引用并修改
        let doc_mut = manifest.get_mut("test").unwrap();
        doc_mut.name = "Updated Name".to_string();

        // 检查修改是否生效
        let doc = manifest.get("test").unwrap();
        assert_eq!(doc.name, "Updated Name");
    }

    #[test]
    fn test_docs_mut() {
        let mut manifest = Manifest::new();
        let doc1 = create_test_doc_spec("test1");
        let doc2 = create_test_doc_spec("test2");

        manifest.add(doc1);
        
        // 获取可变引用并添加另一个文档
        let docs_mut = manifest.docs_mut();
        docs_mut.insert("test2".to_string(), doc2);

        // 检查添加是否生效
        assert_eq!(manifest.docs().len(), 2);
        assert!(manifest.get("test2").is_some());
    }
}
