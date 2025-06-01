use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct IndexEntry {
    pub name: String,
    pub path: String,
    #[serde(rename = "type")]
    pub entry_type: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct IndexType {
    pub name: String,
    pub count: usize,
    pub slug: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FullIndex {
    pub entries: Vec<IndexEntry>,
    pub types: Vec<IndexType>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{json, Value};

    #[test]
    fn test_index_entry_serialization() {
        let entry = IndexEntry {
            name: "Test Entry".to_string(),
            path: "/path/to/entry".to_string(),
            entry_type: "function".to_string(),
        };
        
        let json = serde_json::to_string(&entry).unwrap();
        let expected = json!({"name": "Test Entry", "path": "/path/to/entry", "type": "function"});
        let actual: Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_index_entry_deserialization() {
        let json = r#"{"name": "Test Entry", "path": "/path/to/entry", "type": "function"}"#;
        let entry: IndexEntry = serde_json::from_str(json).unwrap();
        
        assert_eq!(entry.name, "Test Entry");
        assert_eq!(entry.path, "/path/to/entry");
        assert_eq!(entry.entry_type, "function");
    }
    
    #[test]
    fn test_index_type_serialization() {
        let index_type = IndexType {
            name: "Function".to_string(),
            count: 42,
            slug: "function".to_string(),
        };
        
        let json = serde_json::to_string(&index_type).unwrap();
        let expected = json!({"name": "Function", "count": 42, "slug": "function"});
        let actual: Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_index_type_deserialization() {
        let json = r#"{"name": "Function", "count": 42, "slug": "function"}"#;
        let index_type: IndexType = serde_json::from_str(json).unwrap();
        
        assert_eq!(index_type.name, "Function");
        assert_eq!(index_type.count, 42);
        assert_eq!(index_type.slug, "function");
    }
    
    #[test]
    fn test_full_index_serialization() {
        let entry = IndexEntry {
            name: "Test Entry".to_string(),
            path: "/path/to/entry".to_string(),
            entry_type: "function".to_string(),
        };
        
        let index_type = IndexType {
            name: "Function".to_string(),
            count: 1,
            slug: "function".to_string(),
        };
        
        let full_index = FullIndex {
            entries: vec![entry],
            types: vec![index_type],
        };
        
        let json = serde_json::to_string(&full_index).unwrap();
        let expected = json!({
            "entries": [{"name": "Test Entry", "path": "/path/to/entry", "type": "function"}],
            "types": [{"name": "Function", "count": 1, "slug": "function"}]
        });
        let actual: Value = serde_json::from_str(&json).unwrap();
        
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_full_index_deserialization() {
        let json = r#"{
            "entries": [{"name": "Test Entry", "path": "/path/to/entry", "type": "function"}],
            "types": [{"name": "Function", "count": 1, "slug": "function"}]
        }"#;
        
        let full_index: FullIndex = serde_json::from_str(json).unwrap();
        
        assert_eq!(full_index.entries.len(), 1);
        assert_eq!(full_index.entries[0].name, "Test Entry");
        assert_eq!(full_index.entries[0].path, "/path/to/entry");
        assert_eq!(full_index.entries[0].entry_type, "function");
        
        assert_eq!(full_index.types.len(), 1);
        assert_eq!(full_index.types[0].name, "Function");
        assert_eq!(full_index.types[0].count, 1);
        assert_eq!(full_index.types[0].slug, "function");
    }
}
