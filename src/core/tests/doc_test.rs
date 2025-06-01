//! Doc 模块测试
//!
//! 参考原始 Ruby 项目中的 doc_test.rb 实现
//! 为 Doc 及相关结构体提供单元测试

use crate::core::doc::{Doc, DocMeta, EntryIndex, PageDb, DB_FILENAME, INDEX_FILENAME, META_FILENAME};
use crate::core::error::Result;
use crate::core::index_entry::{IndexEntry, IndexType};
use crate::storage::store::Store;
use serde_json::json;
use std::collections::HashMap;
use std::time::UNIX_EPOCH;

// 实现一个测试用的 Doc 结构体
struct TestDoc {
    name: String,
    slug: String,
    doc_type: String,
    version: Option<String>,
    release: Option<String>,
    links: HashMap<String, String>,
}

impl TestDoc {
    fn new(name: &str, slug: &str, doc_type: &str) -> Self {
        Self {
            name: name.to_string(),
            slug: slug.to_string(),
            doc_type: doc_type.to_string(),
            version: None,
            release: None,
            links: HashMap::new(),
        }
    }
}

impl Doc for TestDoc {
    fn name(&self) -> &str {
        &self.name
    }

    fn slug(&self) -> &str {
        &self.slug
    }

    fn doc_type(&self) -> &str {
        &self.doc_type
    }

    fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    fn release(&self) -> Option<&str> {
        self.release.as_deref()
    }

    fn links(&self) -> HashMap<String, String> {
        self.links.clone()
    }

    fn build_page(&self, _id: &str) -> Result<Option<HashMap<String, serde_json::Value>>> {
        // 简单测试实现
        let mut page = HashMap::new();
        page.insert("path".to_string(), json!("test_path"));
        page.insert("output".to_string(), json!("test_output"));
        page.insert("store_path".to_string(), json!("test_store_path"));
        
        let entry = IndexEntry {
            name: "Test Entry".to_string(),
            path: "test_path".to_string(),
            entry_type: "function".to_string(),
        };
        
        page.insert("entries".to_string(), json!([entry]));
        Ok(Some(page))
    }

    fn build_pages<F>(&self, mut callback: F) -> Result<()>
    where
        F: FnMut(HashMap<String, serde_json::Value>),
    {
        // 简单测试实现
        if let Some(page) = self.build_page("")? {
            callback(page);
        }
        Ok(())
    }

    fn get_scraper_version(&self, _opts: &HashMap<String, String>) -> Result<String> {
        Ok("1.0.0".to_string())
    }

    fn get_latest_version(&self, _opts: &HashMap<String, String>) -> Result<String> {
        Ok("1.0.0".to_string())
    }
}

// 实现一个测试用的 Store
struct TestStore {
    data: HashMap<String, String>,
}

impl TestStore {
    fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl Store for TestStore {
    fn read(&self, path: &str) -> Result<String> {
        self.data.get(path)
            .cloned()
            .ok_or_else(|| crate::core::error::Error::NotFound(format!("Path not found: {}", path)))
    }

    fn write(&mut self, path: &str, content: &str) -> Result<()> {
        self.data.insert(path.to_string(), content.to_string());
        Ok(())
    }

    fn delete(&mut self, path: &str) -> Result<()> {
        self.data.remove(path);
        Ok(())
    }

    fn exist(&self, path: &str) -> bool {
        self.data.contains_key(path)
    }

    fn mtime(&self, _path: &str) -> Result<std::time::SystemTime> {
        Ok(UNIX_EPOCH)
    }

    fn size(&self, path: &str) -> Result<usize> {
        Ok(self.data.get(path).map_or(0, |s| s.len()))
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_doc_meta() {
        let meta = DocMeta {
            name: "Test".to_string(),
            slug: "test".to_string(),
            doc_type: "library".to_string(),
            version: Some("1.0.0".to_string()),
            release: Some("stable".to_string()),
            links: {
                let mut links = HashMap::new();
                links.insert("home".to_string(), "https://example.com".to_string());
                links
            },
            mtime: Some(123456789),
            db_size: Some(1024),
        };
        
        assert_eq!(meta.name, "Test");
        assert_eq!(meta.slug, "test");
        assert_eq!(meta.doc_type, "library");
        assert_eq!(meta.version, Some("1.0.0".to_string()));
        assert_eq!(meta.release, Some("stable".to_string()));
        assert_eq!(meta.links.get("home"), Some(&"https://example.com".to_string()));
        assert_eq!(meta.mtime, Some(123456789));
        assert_eq!(meta.db_size, Some(1024));
    }
    
    #[test]
    fn test_page_db() {
        let mut db = PageDb::new();
        assert!(db.is_empty());
        assert_eq!(db.len(), 0);
        
        db.add("test".to_string(), "content".to_string());
        assert!(!db.is_empty());
        assert_eq!(db.len(), 1);
        
        let json = db.to_json();
        assert!(json.contains("test"));
        assert!(json.contains("content"));
    }
    
    #[test]
    fn test_entry_index() {
        let mut index = EntryIndex::new();
        assert!(index.is_empty());
        
        let entry = IndexEntry {
            name: "Test Entry".to_string(),
            path: "test_path".to_string(),
            entry_type: "function".to_string(),
        };
        
        index.add(entry);
        assert!(!index.is_empty());
        
        let full_index = index.to_full_index();
        assert_eq!(full_index.entries.len(), 1);
        assert_eq!(full_index.entries[0].name, "Test Entry");
        assert_eq!(full_index.types.len(), 1);
        assert!(full_index.types.contains_key("function"));
    }
    
    #[test]
    fn test_doc_trait() {
        let doc = TestDoc::new("Test Doc", "test_doc", "library");
        
        assert_eq!(doc.name(), "Test Doc");
        assert_eq!(doc.slug(), "test_doc");
        assert_eq!(doc.doc_type(), "library");
        assert_eq!(doc.path(), "test_doc");
        assert_eq!(doc.index_path(), "test_doc/index.json");
        assert_eq!(doc.db_path(), "test_doc/db.json");
        assert_eq!(doc.meta_path(), "test_doc/meta.json");
        
        let meta = doc.as_json();
        assert_eq!(meta.name, "Test Doc");
        assert_eq!(meta.slug, "test_doc");
        assert_eq!(meta.doc_type, "library");
        assert_eq!(meta.version, None);
        assert_eq!(meta.release, None);
    }
    
    #[test]
    fn test_store_page() {
        let doc = TestDoc::new("Test Doc", "test_doc", "library");
        let mut store = TestStore::new();
        
        // 测试存储页面
        let result = doc.store_page(&mut store, "test_id");
        assert!(result.is_ok());
        assert!(result.unwrap());
        
        // 检查 store 是否包含必要的文件
        assert!(store.exist("test_store_path"));
        assert_eq!(store.data.get("test_store_path"), Some(&"test_output".to_string()));
    }
    
    #[test]
    fn test_store_all() {
        let doc = TestDoc::new("Test Doc", "test_doc", "library");
        let mut store = TestStore::new();
        
        // 测试存储所有页面
        let result = doc.store_all(&mut store);
        assert!(result.is_ok());
        
        // 检查是否创建了元数据文件
        let meta_path = doc.meta_path();
        assert!(store.exist(&meta_path));
    }
}
