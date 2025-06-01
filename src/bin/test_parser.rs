use xwdoc::core::parser::Parser;

fn main() {
    println!("测试 parse_fragment 方法");
    let parser = Parser::new();
    let html = "<div>Test Fragment</div>";
    let context_node = "div";

    match parser.parse_fragment(html, context_node) {
        Ok(_) => println!("parse_fragment 方法成功解析 HTML"),
        Err(e) => println!("parse_fragment 方法出错：{}", e),
    }
}
