use crate::core::error::Result;
use crate::core::scraper::base::Scraper;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use once_cell::sync::Lazy;

/// 文档抓取器工厂函数类型
pub type ScraperFactory = fn(version: &str, output_path: &str) -> Box<dyn Scraper>;

/// 全局文档注册表
static SCRAPER_REGISTRY: Lazy<Arc<Mutex<ScraperRegistry>>> = Lazy::new(|| {
    Arc::new(Mutex::new(ScraperRegistry::new()))
});

/// 文档抓取器注册表
#[derive(Default)]
pub struct ScraperRegistry {
    factories: HashMap<String, ScraperFactory>,
}

impl ScraperRegistry {
    /// 创建新的注册表
    pub fn new() -> Self {
        Self {
            factories: HashMap::new(),
        }
    }

    /// 注册文档抓取器
    pub fn register(&mut self, name: &str, factory: ScraperFactory) {
        self.factories.insert(name.to_string(), factory);
    }

    /// 获取文档抓取器
    pub fn get(&self, name: &str, version: &str, output_path: &str) -> Option<Box<dyn Scraper>> {
        self.factories.get(name).map(|factory| factory(version, output_path))
    }

    /// 获取所有已注册的文档类型
    pub fn get_all_names(&self) -> Vec<String> {
        self.factories.keys().cloned().collect()
    }
}

/// 注册文档抓取器
pub fn register_scraper(name: &str, factory: ScraperFactory) {
    let mut registry = SCRAPER_REGISTRY.lock().unwrap();
    registry.register(name, factory);
}

/// 获取文档抓取器
pub fn get_scraper(name: &str, version: &str, output_path: &str) -> Option<Box<dyn Scraper>> {
    let registry = SCRAPER_REGISTRY.lock().unwrap();
    registry.get(name, version, output_path)
}

/// 获取所有已注册的文档类型
pub fn get_all_scraper_names() -> Vec<String> {
    let registry = SCRAPER_REGISTRY.lock().unwrap();
    registry.get_all_names()
}

/// 创建并运行文档抓取器
pub async fn run_scraper(name: &str, version: &str, output_path: &str) -> Result<()> {
    if let Some(mut scraper) = get_scraper(name, version, output_path) {
        scraper.run().await
    } else {
        Err(crate::core::error::Error::Configuration(format!("未找到文档抓取器: {}", name)))
    }
}
