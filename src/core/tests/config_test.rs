//! Config 模块测试
//!
//! 为配置模块提供单元测试

use crate::core::config::Config;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();

        // 验证默认值
        assert_eq!(config.docs_path, "docs");
        assert_eq!(config.host, "127.0.0.1");
        assert_eq!(config.port, 8000);
        assert!(config.default_docs.contains(&"html".to_string()));
        assert!(config.default_docs.contains(&"css".to_string()));
        assert!(config.default_docs.contains(&"javascript".to_string()));
        assert!(config.default_docs.contains(&"rust".to_string()));
    }

    #[test]
    fn test_new_config() {
        let config = Config::new();

        // 验证 new() 和 default() 结果相同
        let default_config = Config::default();
        assert_eq!(config.docs_path, default_config.docs_path);
        assert_eq!(config.host, default_config.host);
        assert_eq!(config.port, default_config.port);
        assert_eq!(config.default_docs, default_config.default_docs);
    }

    #[test]
    fn test_with_docs_path() {
        let config = Config::new().with_docs_path("/custom/docs");

        // 验证自定义路径
        assert_eq!(config.docs_path, "/custom/docs");
    }

    #[test]
    fn test_with_default_docs() {
        let docs = vec![
            "python".to_string(),
            "golang".to_string(),
            "kotlin".to_string(),
        ];

        let config = Config::new().with_default_docs(docs.clone());

        // 验证自定义默认文档列表
        assert_eq!(config.default_docs, docs);
    }

    #[test]
    fn test_with_host() {
        let config = Config::new().with_host("0.0.0.0");

        // 验证自定义主机名
        assert_eq!(config.host, "0.0.0.0");
    }

    #[test]
    fn test_with_port() {
        let config = Config::new().with_port(9000);

        // 验证自定义端口
        assert_eq!(config.port, 9000);
    }

    #[test]
    fn test_builder_pattern() {
        let config = Config::new()
            .with_docs_path("/var/docs")
            .with_host("localhost")
            .with_port(3000)
            .with_default_docs(vec!["java".to_string(), "scala".to_string()]);

        // 验证构建器模式正确设置所有属性
        assert_eq!(config.docs_path, "/var/docs");
        assert_eq!(config.host, "localhost");
        assert_eq!(config.port, 3000);
        assert_eq!(config.default_docs.len(), 2);
        assert!(config.default_docs.contains(&"java".to_string()));
        assert!(config.default_docs.contains(&"scala".to_string()));
    }
}
