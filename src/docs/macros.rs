//! 文档宏模块
//!
//! 提供用于简化文档注册和过滤器创建的宏

/// 注册文档宏
///
/// 用于简化文档注册过程
#[macro_export]
macro_rules! register_doc {
    (
        name: $name:expr,
        slug: $slug:expr,
        type: $type:expr,
        version: $version:expr,
        release: $release:expr,
        base_url: $base_url:expr,
        root_path: $root_path:expr,
        $(links: {
            $($link_key:expr => $link_value:expr),*
        },)?
        $(html_filters: [$($html_filter:expr),*],)?
        $(text_filters: [$($text_filter:expr),*],)?
        $(options: {
            $($option_key:expr => $option_value:expr),*
        })?
    ) => {
        {
            use $crate::core::scraper_base::Scraper;
            use $crate::docs::autoload::get_filter;
            use $crate::docs::registry_new::register_doc;
            
            // 创建抓取器
            let mut scraper = Box::new(Scraper::new($name, $slug, $type, $version, $release));
            
            // 设置基础 URL 和根路径
            scraper.scraper.set_base_url($base_url.to_string());
            scraper.scraper.set_root_path($root_path.to_string());
            
            // 添加链接
            $(
                $(
                    scraper.scraper.add_link($link_key.to_string(), $link_value.to_string());
                )*
            )?
            
            // 添加 HTML 过滤器
            $(
                let html_filters = vec![$($html_filter),*];
                for filter_name in html_filters {
                    if let Some(filter) = get_filter(filter_name) {
                        scraper.scraper.add_html_filter(filter);
                    }
                }
            )?
            
            // 添加文本过滤器
            $(
                let text_filters = vec![$($text_filter),*];
                for filter_name in text_filters {
                    if let Some(filter) = get_filter(filter_name) {
                        scraper.scraper.add_text_filter(filter);
                    }
                }
            )?
            
            // 设置选项
            $(
                $(
                    scraper.scraper.set_option($option_key.to_string(), $option_value.to_string());
                )*
            )?
            
            // 注册文档
            register_doc($slug, scraper);
            
            // 返回抓取器
            scraper
        }
    };
}

/// 创建文档过滤器宏
///
/// 用于简化过滤器创建过程
#[macro_export]
macro_rules! create_filter {
    (
        name: $name:ident,
        call: |$html:ident, $context:ident| $body:block
    ) => {
        #[derive(Clone, Default)]
        pub struct $name;
        
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }
        }
        
        impl $crate::core::filter_base::Filter for $name {
            fn apply(&self, $html: &str, $context: &mut $crate::core::filter_base::FilterContext) -> $crate::core::error::Result<String> {
                let mut document = nipper::Document::from($html);
                $body
            }
            
            fn box_clone(&self) -> Box<dyn $crate::core::filter_base::Filter> {
                Box::new(self.clone())
            }
        }
    };
}

/// 创建条目过滤器宏
///
/// 用于简化条目过滤器创建过程
#[macro_export]
macro_rules! create_entries_filter {
    (
        name: $name:ident,
        get_name: |$html_name:ident, $context_name:ident| $name_body:block,
        get_type: |$html_type:ident, $context_type:ident| $type_body:block,
        additional_entries: |$html_entries:ident, $context_entries:ident| $entries_body:block
    ) => {
        #[derive(Clone, Default)]
        pub struct $name;
        
        impl $name {
            pub fn new() -> Self {
                Self::default()
            }
            
            pub fn get_name(&self, $html_name: &str, $context_name: &$crate::core::filter_base::FilterContext) -> String {
                let document = nipper::Document::from($html_name);
                $name_body
            }
            
            pub fn get_type(&self, $html_type: &str, $context_type: &$crate::core::filter_base::FilterContext) -> String {
                let document = nipper::Document::from($html_type);
                $type_body
            }
            
            pub fn additional_entries(&self, $html_entries: &str, $context_entries: &$crate::core::filter_base::FilterContext) -> Vec<(String, String, String)> {
                let document = nipper::Document::from($html_entries);
                $entries_body
            }
        }
        
        impl $crate::core::filter_base::Filter for $name {
            fn apply(&self, html: &str, _context: &mut $crate::core::filter_base::FilterContext) -> $crate::core::error::Result<String> {
                // 条目过滤器通常不修改 HTML
                Ok(html.to_string())
            }
            
            fn box_clone(&self) -> Box<dyn $crate::core::filter_base::Filter> {
                Box::new(self.clone())
            }
        }
    };
}
