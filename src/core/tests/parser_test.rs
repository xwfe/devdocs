//! Parser 模块测试
//!
//! 参考原始 Ruby 项目中的 parser_test.rb 实现
//! 为 HTML 解析器提供单元测试

// 从核心模块导入Parser和NodeExt
use crate::core::parser::NodeExt;
use crate::core::parser::Parser;
use markup5ever_arcdom::NodeData;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_creation() {
        let parser = Parser::new();
        // 不能直接访问私有字段，使用功能测试替代
        // 测试创建的parser可以正常工作
        let html = "<html><body>Test</body></html>";
        let result = parser.parse(html);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_html_document() {
        let parser = Parser::new();
        let html = r#"
            <!DOCTYPE html>
            <html>
                <head>
                    <title>Test Document</title>
                </head>
                <body>
                    <div>Hello, World!</div>
                </body>
            </html>
        "#;

        let result = parser.parse(html);
        assert!(result.is_ok());

        let dom = result.unwrap();

        // 检查根元素
        let root = dom.document;
        let children = root.children.borrow();
        let html_elem = children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "html")
        });
        assert!(html_elem.is_some(), "Expected to find <html> element");

        // 检查标题
        let html_elem = html_elem.unwrap();
        let html_children = html_elem.children.borrow();
        let head_elem = html_children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "head")
        }).unwrap();

        let head_children = head_elem.children.borrow();
        let title_elem = head_children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "title")
        }).unwrap();

        let title_text = title_elem.text_content();
        assert_eq!(title_text.trim(), "Test Document");

        // 检查 body 内容
        let body_elem = html_children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "body")
        }).unwrap();

        let body_children = body_elem.children.borrow();
        let div_elem = body_children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "div")
        }).unwrap();

        let div_text = div_elem.text_content();
        assert_eq!(div_text.trim(), "Hello, World!");
    }

    #[test]
    fn test_parse_html_fragment() {
        let parser = Parser::new();
        let html = "<div>Test Fragment</div>";

        let result = parser.parse_fragment(html, "div");
        assert!(result.is_ok());

        let dom = result.unwrap();
        let root = dom.document;
        let children = root.children.borrow();
        let div_elem = children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "div")
        }).unwrap();

        let text = div_elem.text_content();
        assert_eq!(text.trim(), "Test Fragment");
    }

    #[test]
    fn test_clean_html() {
        let parser = Parser::new();
        let html =
            r#"<!DOCTYPE html><html><body>Test with &#46;dot and &#x2E; entities</body></html>"#;

        let clean = parser.clean_html(html);

        // 应该移除 DOCTYPE，并替换所有点号实体为实际点号
        assert!(!clean.contains("<!DOCTYPE"));
        assert!(clean.contains("Test with .dot and . entities"));
    }

    #[test]
    fn test_node_text_content() {
        let parser = Parser::new();
        let html = r#"
            <div id="test">
                <span>Hello</span>
                World
            </div>
        "#;

        let dom = parser.parse_fragment(html, "div").unwrap();
        let root = dom.document;
        let children = root.children.borrow();
        let div_elem = children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "div")
        }).unwrap();

        // 测试 text_content 获取所有嵌套文本
        let text = div_elem.text_content();
        let text = text.trim().split_whitespace().collect::<Vec<_>>().join(" ");
        assert_eq!(text, "Hello World");
    }

    #[test]
    fn test_node_attr() {
        let parser = Parser::new();
        let html = r#"<div id="test" data-value="example">Content</div>"#;

        let dom = parser.parse_fragment(html, "div").unwrap();
        let root = dom.document;
        let children = root.children.borrow();
        let div_elem = children.iter().find(|node| {
            matches!(&node.data, NodeData::Element { name, .. } if name.local.as_ref() == "div")
        }).unwrap();

        // 测试获取属性
        let id = div_elem.attr("id");
        assert_eq!(id, Some("test".to_string()));

        let data_value = div_elem.attr("data-value");
        assert_eq!(data_value, Some("example".to_string()));

        // 测试获取不存在的属性
        let nonexistent = div_elem.attr("nonexistent");
        assert_eq!(nonexistent, None);
    }
}
