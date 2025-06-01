//! 文档索引生成器模块
//!
//! 提供生成文档索引的功能

use crate::core::error::{Error, Result};
use crate::core::parser::HtmlParser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 文档条目
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    /// 条目名称
    pub name: String,
    /// 条目路径
    pub path: String,
    /// 条目类型
    pub type_str: String,
    /// 父条目路径
    pub parent: Option<String>,
}

/// 文档索引
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Index {
    /// 条目列表
    pub entries: Vec<Entry>,
    /// 条目映射
    pub entries_map: HashMap<String, usize>,
}

impl Index {
    /// 创建新的文档索引
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            entries_map: HashMap::new(),
        }
    }
    
    /// 添加条目
    pub fn add_entry(&mut self, name: &str, path: &str, type_str: &str, parent: Option<&str>) {
        let entry = Entry {
            name: name.to_string(),
            path: path.to_string(),
            type_str: type_str.to_string(),
            parent: parent.map(|p| p.to_string()),
        };
        
        // 添加条目到列表
        let index = self.entries.len();
        self.entries.push(entry);
        
        // 添加条目到映射
        self.entries_map.insert(path.to_string(), index);
    }
    
    /// 获取条目
    pub fn get_entry(&self, path: &str) -> Option<&Entry> {
        if let Some(&index) = self.entries_map.get(path) {
            self.entries.get(index)
        } else {
            None
        }
    }
    
    /// 获取条目数量
    pub fn len(&self) -> usize {
        self.entries.len()
    }
    
    /// 检查索引是否为空
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
    
    /// 保存索引到文件
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    /// 从文件加载索引
    pub fn load(path: &Path) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let index = serde_json::from_str(&json)?;
        Ok(index)
    }
}

/// 文档索引生成器
pub struct Indexer {
    /// 文档目录
    pub doc_dir: PathBuf,
    /// 输出目录
    pub output_dir: PathBuf,
    /// 文档索引
    pub index: Index,
}

impl Indexer {
    /// 创建新的文档索引生成器
    pub fn new(doc_dir: &Path, output_dir: &Path) -> Self {
        Self {
            doc_dir: doc_dir.to_path_buf(),
            output_dir: output_dir.to_path_buf(),
            index: Index::new(),
        }
    }
    
    /// 生成索引
    pub fn generate(&mut self) -> Result<()> {
        // 检查文档目录是否存在
        if !self.doc_dir.exists() || !self.doc_dir.is_dir() {
            return Err(Error::Message(format!("文档目录不存在: {:?}", self.doc_dir)).into());
        }
        
        // 创建输出目录
        if !self.output_dir.exists() {
            fs::create_dir_all(&self.output_dir)?;
        }
        
        // 遍历文档目录
        self.traverse_directory(&self.doc_dir, None)?;
        
        // 保存索引
        let index_path = self.output_dir.join("index.json");
        self.index.save(&index_path)?;
        
        Ok(())
    }
    
    /// 遍历目录
    fn traverse_directory(&mut self, dir: &Path, parent: Option<&str>) -> Result<()> {
        // 遍历目录中的文件和子目录
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // 如果是目录，递归遍历
                let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
                let dir_path = if let Some(parent) = parent {
                    format!("{}/{}", parent, dir_name)
                } else {
                    dir_name.clone()
                };
                
                // 添加目录条目
                self.index.add_entry(&dir_name, &dir_path, "Directory", parent);
                
                // 递归遍历子目录
                self.traverse_directory(&path, Some(&dir_path))?;
            } else if path.is_file() {
                // 如果是文件，处理文件
                if let Some(extension) = path.extension() {
                    if extension == "html" || extension == "htm" {
                        // 处理 HTML 文件
                        self.process_html_file(&path, parent)?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 处理 HTML 文件
    fn process_html_file(&mut self, file: &Path, parent: Option<&str>) -> Result<()> {
        // 读取文件内容
        let html = fs::read_to_string(file)?;
        
        // 解析 HTML
        let parser = HtmlParser::new(&html);
        
        // 获取文件名
        let file_name = file.file_name().unwrap().to_string_lossy().to_string();
        
        // 获取文件路径
        let file_path = if let Some(parent) = parent {
            format!("{}/{}", parent, file_name)
        } else {
            file_name.clone()
        };
        
        // 获取标题
        let title = parser.title().unwrap_or_else(|_| file_name.clone());
        
        // 添加文件条目
        self.index.add_entry(&title, &file_path, "Page", parent);
        
        // 提取页面中的条目
        self.extract_entries(&parser, &file_path)?;
        
        Ok(())
    }
    
    /// 提取条目
    fn extract_entries(&mut self, parser: &HtmlParser, parent: &str) -> Result<()> {
        // 提取标题元素
        for selector in &["h1", "h2", "h3", "h4", "h5", "h6"] {
            for element in parser.select(selector).iter() {
                // 获取标题文本
                let name = element.text().to_string().trim().to_string();
                
                // 跳过空标题
                if name.is_empty() {
                    continue;
                }
                
                // 获取标题 ID
                let id = if let Some(id_attr) = element.attr("id") {
                    id_attr.to_string()
                } else {
                    // 如果没有 ID，使用名称生成一个
                    name.to_lowercase().replace(' ', "-")
                };
                
                // 生成路径
                let path = format!("{}#{}", parent, id);
                
                // 确定类型
                let type_str = match *selector {
                    "h1" => "Section",
                    "h2" => "Subsection",
                    "h3" => "Method",
                    "h4" => "Property",
                    "h5" => "Parameter",
                    "h6" => "Detail",
                    _ => "Entry",
                };
                
                // 添加条目
                self.index.add_entry(&name, &path, type_str, Some(parent));
            }
        }
        
        Ok(())
    }
}

/// 生成文档索引
pub fn generate_index(doc_dir: &Path, output_dir: &Path) -> Result<Index> {
    let mut indexer = Indexer::new(doc_dir, output_dir);
    indexer.generate()?;
    Ok(indexer.index)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_index() {
        let mut index = Index::new();
        
        index.add_entry("Home", "index.html", "Page", None);
        index.add_entry("Introduction", "index.html#intro", "Section", Some("index.html"));
        index.add_entry("API", "api.html", "Page", None);
        index.add_entry("Methods", "api.html#methods", "Section", Some("api.html"));
        
        assert_eq!(index.len(), 4);
        assert!(!index.is_empty());
        
        let entry = index.get_entry("index.html").unwrap();
        assert_eq!(entry.name, "Home");
        assert_eq!(entry.type_str, "Page");
        assert_eq!(entry.parent, None);
        
        let entry = index.get_entry("api.html#methods").unwrap();
        assert_eq!(entry.name, "Methods");
        assert_eq!(entry.type_str, "Section");
        assert_eq!(entry.parent, Some("api.html".to_string()));
    }
    
    #[test]
    fn test_indexer() {
        // 创建临时目录
        let doc_dir = tempdir().unwrap();
        let output_dir = tempdir().unwrap();
        
        // 创建测试文件
        let index_path = doc_dir.path().join("index.html");
        let mut index_file = fs::File::create(&index_path).unwrap();
        writeln!(index_file, r#"<!DOCTYPE html>
        <html>
        <head>
            <title>Home</title>
        </head>
        <body>
            <h1 id="intro">Introduction</h1>
            <p>Welcome to the documentation.</p>
            <h2 id="getting-started">Getting Started</h2>
            <p>This is a getting started guide.</p>
        </body>
        </html>"#).unwrap();
        
        let api_path = doc_dir.path().join("api.html");
        let mut api_file = fs::File::create(&api_path).unwrap();
        writeln!(api_file, r#"<!DOCTYPE html>
        <html>
        <head>
            <title>API Reference</title>
        </head>
        <body>
            <h1 id="api">API Reference</h1>
            <p>This is the API reference.</p>
            <h2 id="methods">Methods</h2>
            <p>List of methods.</p>
            <h3 id="method1">Method 1</h3>
            <p>Description of method 1.</p>
        </body>
        </html>"#).unwrap();
        
        // 生成索引
        let index = generate_index(doc_dir.path(), output_dir.path()).unwrap();
        
        // 检查索引
        assert_eq!(index.len(), 7);
        
        let entry = index.get_entry("index.html").unwrap();
        assert_eq!(entry.name, "Home");
        assert_eq!(entry.type_str, "Page");
        
        let entry = index.get_entry("index.html#intro").unwrap();
        assert_eq!(entry.name, "Introduction");
        assert_eq!(entry.type_str, "Section");
        assert_eq!(entry.parent, Some("index.html".to_string()));
        
        let entry = index.get_entry("api.html#method1").unwrap();
        assert_eq!(entry.name, "Method 1");
        assert_eq!(entry.type_str, "Method");
        assert_eq!(entry.parent, Some("api.html".to_string()));
        
        // 检查索引文件
        let index_file_path = output_dir.path().join("index.json");
        assert!(index_file_path.exists());
        
        // 加载索引文件
        let loaded_index = Index::load(&index_file_path).unwrap();
        assert_eq!(loaded_index.len(), index.len());
    }
}
