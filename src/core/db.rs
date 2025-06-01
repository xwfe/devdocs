//! 文档数据库管理器模块
//!
//! 提供管理文档内容数据库的功能

use crate::core::error::{Error, Result};
use crate::core::parser::HtmlParser;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// 文档页面
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// 页面路径
    pub path: String,
    /// 页面标题
    pub title: String,
    /// 页面内容
    pub content: String,
}

/// 文档数据库
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Database {
    /// 页面映射
    pub pages: HashMap<String, Page>,
}

impl Database {
    /// 创建新的文档数据库
    pub fn new() -> Self {
        Self {
            pages: HashMap::new(),
        }
    }
    
    /// 添加页面
    pub fn add_page(&mut self, path: &str, title: &str, content: &str) {
        let page = Page {
            path: path.to_string(),
            title: title.to_string(),
            content: content.to_string(),
        };
        
        self.pages.insert(path.to_string(), page);
    }
    
    /// 获取页面
    pub fn get_page(&self, path: &str) -> Option<&Page> {
        self.pages.get(path)
    }
    
    /// 获取页面数量
    pub fn len(&self) -> usize {
        self.pages.len()
    }
    
    /// 检查数据库是否为空
    pub fn is_empty(&self) -> bool {
        self.pages.is_empty()
    }
    
    /// 保存数据库到文件
    pub fn save(&self, path: &Path) -> Result<()> {
        let json = serde_json::to_string(self)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    /// 从文件加载数据库
    pub fn load(path: &Path) -> Result<Self> {
        let json = fs::read_to_string(path)?;
        let db = serde_json::from_str(&json)?;
        Ok(db)
    }
}

/// 文档数据库管理器
pub struct DatabaseManager {
    /// 文档目录
    pub doc_dir: PathBuf,
    /// 输出目录
    pub output_dir: PathBuf,
    /// 文档数据库
    pub db: Database,
}

impl DatabaseManager {
    /// 创建新的文档数据库管理器
    pub fn new(doc_dir: &Path, output_dir: &Path) -> Self {
        Self {
            doc_dir: doc_dir.to_path_buf(),
            output_dir: output_dir.to_path_buf(),
            db: Database::new(),
        }
    }
    
    /// 生成数据库
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
        self.traverse_directory(&self.doc_dir, "")?;
        
        // 保存数据库
        let db_path = self.output_dir.join("db.json");
        self.db.save(&db_path)?;
        
        Ok(())
    }
    
    /// 遍历目录
    fn traverse_directory(&mut self, dir: &Path, parent_path: &str) -> Result<()> {
        // 遍历目录中的文件和子目录
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // 如果是目录，递归遍历
                let dir_name = path.file_name().unwrap().to_string_lossy().to_string();
                let dir_path = if parent_path.is_empty() {
                    dir_name.clone()
                } else {
                    format!("{}/{}", parent_path, dir_name)
                };
                
                // 递归遍历子目录
                self.traverse_directory(&path, &dir_path)?;
            } else if path.is_file() {
                // 如果是文件，处理文件
                if let Some(extension) = path.extension() {
                    if extension == "html" || extension == "htm" {
                        // 处理 HTML 文件
                        self.process_html_file(&path, parent_path)?;
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// 处理 HTML 文件
    fn process_html_file(&mut self, file: &Path, parent_path: &str) -> Result<()> {
        // 读取文件内容
        let html = fs::read_to_string(file)?;
        
        // 解析 HTML
        let parser = HtmlParser::new(&html);
        
        // 获取文件名
        let file_name = file.file_name().unwrap().to_string_lossy().to_string();
        
        // 获取文件路径
        let file_path = if parent_path.is_empty() {
            file_name.clone()
        } else {
            format!("{}/{}", parent_path, file_name)
        };
        
        // 获取标题
        let title = parser.title().unwrap_or_else(|_| file_name.clone());
        
        // 获取内容
        let content = self.extract_content(&parser)?;
        
        // 添加页面
        self.db.add_page(&file_path, &title, &content);
        
        Ok(())
    }
    
    /// 提取内容
    fn extract_content(&self, parser: &HtmlParser) -> Result<String> {
        // 获取主要内容
        let content = parser.select("body").first();
        
        if let Some(content) = content {
            // 清理内容
            let mut content_parser = HtmlParser::new(&content.html().to_string_lossy());
            content_parser.clean();
            
            // 返回清理后的内容
            Ok(content_parser.html())
        } else {
            Err(Error::Message("未找到内容".to_string()).into())
        }
    }
}

/// 生成文档数据库
pub fn generate_database(doc_dir: &Path, output_dir: &Path) -> Result<Database> {
    let mut manager = DatabaseManager::new(doc_dir, output_dir);
    manager.generate()?;
    Ok(manager.db)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;
    
    #[test]
    fn test_database() {
        let mut db = Database::new();
        
        db.add_page("index.html", "Home", "<h1>Welcome</h1>");
        db.add_page("api.html", "API", "<h1>API Reference</h1>");
        
        assert_eq!(db.len(), 2);
        assert!(!db.is_empty());
        
        let page = db.get_page("index.html").unwrap();
        assert_eq!(page.title, "Home");
        assert_eq!(page.content, "<h1>Welcome</h1>");
        
        let page = db.get_page("api.html").unwrap();
        assert_eq!(page.title, "API");
        assert_eq!(page.content, "<h1>API Reference</h1>");
    }
    
    #[test]
    fn test_database_manager() {
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
            <h1>Welcome</h1>
            <p>This is the home page.</p>
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
            <h1>API Reference</h1>
            <p>This is the API reference.</p>
        </body>
        </html>"#).unwrap();
        
        // 生成数据库
        let db = generate_database(doc_dir.path(), output_dir.path()).unwrap();
        
        // 检查数据库
        assert_eq!(db.len(), 2);
        
        let page = db.get_page("index.html").unwrap();
        assert_eq!(page.title, "Home");
        assert!(page.content.contains("<h1>Welcome</h1>"));
        
        let page = db.get_page("api.html").unwrap();
        assert_eq!(page.title, "API Reference");
        assert!(page.content.contains("<h1>API Reference</h1>"));
        
        // 检查数据库文件
        let db_file_path = output_dir.path().join("db.json");
        assert!(db_file_path.exists());
        
        // 加载数据库文件
        let loaded_db = Database::load(&db_file_path).unwrap();
        assert_eq!(loaded_db.len(), db.len());
    }
}
