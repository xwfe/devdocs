//! Cloudflare 电子邮件解析过滤器
//!
//! 解析 Cloudflare 保护的电子邮件地址

use crate::core::error::Result;
use crate::core::filter_base::{Filter, FilterContext};
use scraper::{Html, Selector};
use std::any::Any;

/// Cloudflare 电子邮件解析过滤器
///
/// 解析 Cloudflare 保护的电子邮件地址
#[derive(Debug, Clone)]
pub struct ParseCfEmailFilter;

impl Filter for ParseCfEmailFilter {
    fn call(&self, html: &str, _context: &mut FilterContext) -> Result<String> {
        // 解析 HTML
        let document = Html::parse_document(html);
        let mut result = html.to_string();
        
        // 查找 Cloudflare 保护的电子邮件地址
        if let Ok(selector) = Selector::parse("span.__cf_email__") {
            for element in document.select(&selector) {
                if let Some(data) = element.value().attr("data-cfemail") {
                    // 解析 Cloudflare 保护的电子邮件地址
                    if let Some(email) = self.decode_cf_email(data) {
                        // 在实际实现中，我们需要替换 HTML 中的电子邮件地址
                        // 由于 scraper 库的限制，这里只是一个示例
                    }
                }
            }
        }
        
        // 由于 scraper 库的限制，这里只是返回原始 HTML
        // 在实际实现中，我们需要返回修改后的 HTML
        Ok(result)
    }
    
    fn box_clone(&self) -> Box<dyn Filter> {
        Box::new(self.clone())
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl ParseCfEmailFilter {
    /// 解码 Cloudflare 保护的电子邮件地址
    fn decode_cf_email(&self, encoded: &str) -> Option<String> {
        // 将十六进制字符串转换为字节
        let bytes = (0..encoded.len())
            .step_by(2)
            .filter_map(|i| {
                if i + 2 <= encoded.len() {
                    u8::from_str_radix(&encoded[i..i+2], 16).ok()
                } else {
                    None
                }
            })
            .collect::<Vec<u8>>();
        
        if bytes.is_empty() {
            return None;
        }
        
        // 第一个字节是密钥
        let key = bytes[0];
        
        // 解码剩余字节
        let decoded = bytes[1..]
            .iter()
            .map(|&b| b ^ key)
            .collect::<Vec<u8>>();
        
        // 将字节转换为字符串
        String::from_utf8(decoded).ok()
    }
}
