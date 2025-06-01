//! 事件订阅模块
//!
//! 参考原始 Ruby 项目中的 subscriber.rb 实现
//! 提供事件订阅和日志记录功能

use crate::core::instrumentable::InstrumentInfo;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use textwrap::fill;

/// 终端宽度环境变量
const ENV_COLUMNS: &str = "COLUMNS";

/// 默认终端宽度
const DEFAULT_TERMINAL_WIDTH: usize = 80;

/// 订阅者特征
pub trait Subscriber {
    /// 处理事件
    fn handle_event(&self, info: &InstrumentInfo);
}

/// 控制台订阅者，将事件输出到终端
pub struct ConsoleSubscriber {
    /// 是否使用彩色输出
    use_color: bool,
    /// 终端宽度
    terminal_width: Option<usize>,
    /// 输出流
    output: Arc<Mutex<StandardStream>>,
}

impl Default for ConsoleSubscriber {
    fn default() -> Self {
        Self::new()
    }
}

impl ConsoleSubscriber {
    /// 创建新的控制台订阅者
    pub fn new() -> Self {
        let color_choice = if atty::is(atty::Stream::Stdout) {
            ColorChoice::Auto
        } else {
            ColorChoice::Never
        };

        Self {
            use_color: color_choice != ColorChoice::Never,
            terminal_width: Self::get_terminal_width(),
            output: Arc::new(Mutex::new(StandardStream::stdout(color_choice))),
        }
    }

    /// 获取终端宽度
    fn get_terminal_width() -> Option<usize> {
        // 首先检查环境变量
        if let Ok(columns) = std::env::var(ENV_COLUMNS) {
            if let Ok(width) = columns.parse::<usize>() {
                return Some(width);
            }
        }

        // 然后尝试通过终端信息获取
        if let Some((width, _)) = term_size::dimensions() {
            return Some(width);
        }

        // 最后使用默认值
        Some(DEFAULT_TERMINAL_WIDTH)
    }

    /// 格式化 URL，移除协议部分
    fn format_url(&self, url: &str) -> String {
        url.replace("http://", "").replace("https://", "")
    }

    /// 格式化路径，移除当前目录前缀
    fn format_path(&self, path: &str) -> String {
        // 获取当前目录
        if let Ok(current_dir) = std::env::current_dir() {
            if let Some(current_path) = current_dir.to_str() {
                return path.replace(current_path, "");
            }
        }
        path.to_string()
    }

    /// 使文本对齐到终端宽度
    fn justify(&self, text: &str) -> String {
        let width = self.terminal_width.unwrap_or(DEFAULT_TERMINAL_WIDTH);

        // 检查是否有标签部分 [xxx]
        if let Some(tag_index) = text.rfind(" [") {
            let (content, tag) = text.split_at(tag_index);

            // 计算内容部分的最大宽度
            let max_content_width = width.saturating_sub(tag.len());

            // 截断并填充内容部分
            let truncated = if content.len() > max_content_width {
                let mut s = content[..max_content_width].to_string();
                if max_content_width > 3 {
                    s.replace_range(max_content_width - 3.., "...");
                }
                s
            } else {
                content.to_string()
            };

            // 添加空格使内容对齐
            let padded = format!(
                "{}{}",
                truncated,
                " ".repeat(max_content_width.saturating_sub(truncated.len()))
            );

            // 添加标签
            format!("{}{}", padded, tag)
        } else {
            // 无标签，直接处理整个文本
            if text.len() > width {
                let mut s = text[..width].to_string();
                if width > 3 {
                    s.replace_range(width - 3.., "...");
                }
                s
            } else {
                format!("{}{}", text, " ".repeat(width.saturating_sub(text.len())))
            }
        }
    }

    /// 输出日志
    fn log(&self, message: &str) {
        let justified = self.justify(message);

        let mut output = self.output.lock().unwrap();

        // 清除当前行
        let _ = write!(output, "\r");

        // 输出消息
        let _ = writeln!(output, "{}", justified);

        // 刷新输出
        let _ = output.flush();
    }

    /// 更改输出颜色
    fn set_color(&self, color: Option<Color>) {
        if self.use_color {
            let mut output = self.output.lock().unwrap();
            let mut color_spec = ColorSpec::new();

            if let Some(c) = color {
                color_spec.set_fg(Some(c));
            }

            let _ = output.set_color(&color_spec);
        }
    }

    /// 重置输出颜色
    fn reset_color(&self) {
        if self.use_color {
            let mut output = self.output.lock().unwrap();
            let _ = output.reset();
        }
    }
}

impl Subscriber for ConsoleSubscriber {
    fn handle_event(&self, info: &InstrumentInfo) {
        match info.name.as_str() {
            "response.request" => {
                if let Some(url) = info.payload.get("url") {
                    let formatted_url = self.format_url(url);

                    // 设置颜色
                    self.set_color(Some(Color::Cyan));

                    // 输出日志
                    self.log(&format!("Request: {}", formatted_url));

                    // 重置颜色
                    self.reset_color();
                }
            }
            "handle_response.requester" => {
                if let Some(url) = info.payload.get("url") {
                    let formatted_url = self.format_url(url);

                    // 设置颜色
                    self.set_color(Some(Color::Green));

                    // 输出日志
                    self.log(&format!("Response: {}", formatted_url));

                    // 重置颜色
                    self.reset_color();
                }
            }
            "index.doc" | "db.doc" => {
                let event_type = if info.name == "index.doc" {
                    "Index"
                } else {
                    "Database"
                };

                if let Some(before) = info.payload.get("before") {
                    if let Some(after) = info.payload.get("after") {
                        let before_size = before.len();
                        let after_size = after.len();

                        // 设置颜色
                        self.set_color(Some(Color::Yellow));

                        // 输出日志
                        self.log(&format!(
                            "{}: {} -> {} bytes [{}%]",
                            event_type,
                            before_size,
                            after_size,
                            if before_size > 0 {
                                ((after_size as f64 - before_size as f64) / before_size as f64
                                    * 100.0)
                                    .round()
                            } else {
                                0.0
                            }
                        ));

                        // 重置颜色
                        self.reset_color();
                    }
                }
            }
            "warn.doc" => {
                if let Some(msg) = info.payload.get("msg") {
                    // 设置颜色
                    self.set_color(Some(Color::Red));

                    // 输出日志
                    self.log(&format!("Warning: {}", msg));

                    // 重置颜色
                    self.reset_color();
                }
            }
            _ => {
                // 默认日志格式
                let mut message = format!("Event: {}", info.name);

                // 添加持续时间（如果有）
                if let Some(duration) = info.duration {
                    message.push_str(&format!(" [{:.2}ms]", duration.as_millis()));
                }

                // 输出日志
                self.log(&message);
            }
        }
    }
}

/// 文件订阅者，将事件记录到文件
pub struct FileSubscriber {
    /// 日志文件路径
    file_path: String,
}

impl FileSubscriber {
    /// 创建新的文件订阅者
    pub fn new(file_path: &str) -> Self {
        Self {
            file_path: file_path.to_string(),
        }
    }
}

impl Subscriber for FileSubscriber {
    fn handle_event(&self, info: &InstrumentInfo) {
        // 格式化日志消息
        let mut message = format!(
            "[{}] {}",
            info.name,
            chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
        );

        // 添加持续时间（如果有）
        if let Some(duration) = info.duration {
            message.push_str(&format!(" [{:.2}ms]", duration.as_millis()));
        }

        // 添加有效负载
        if !info.payload.is_empty() {
            message.push_str(": ");
            for (key, value) in &info.payload {
                message.push_str(&format!("{}={}, ", key, value));
            }
            message.pop(); // 移除最后的逗号
            message.pop(); // 移除最后的空格
        }

        // 追加到文件
        if let Ok(mut file) = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
        {
            let _ = writeln!(file, "{}", message);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::instrumentable;
    use std::collections::HashMap;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_console_subscriber_justify() {
        let subscriber = ConsoleSubscriber::new();

        // 测试普通文本
        let text = "Hello, world!";
        let justified = subscriber.justify(text);
        assert!(justified.len() >= text.len());

        // 测试带标签的文本
        let text_with_tag = "Processing [INFO]";
        let justified = subscriber.justify(text_with_tag);
        assert!(justified.contains("[INFO]"));
    }

    #[test]
    fn test_file_subscriber() {
        // 创建临时文件路径
        let file_path = "test_log.txt";

        // 确保文件不存在
        if Path::new(file_path).exists() {
            fs::remove_file(file_path).unwrap();
        }

        // 创建文件订阅者
        let subscriber = FileSubscriber::new(file_path);

        // 创建测试事件
        let mut payload = HashMap::new();
        payload.insert("key".to_string(), "value".to_string());

        let info = InstrumentInfo {
            name: "test_event".to_string(),
            start_time: std::time::Instant::now(),
            duration: Some(std::time::Duration::from_millis(100)),
            payload,
        };

        // 处理事件
        subscriber.handle_event(&info);

        // 验证文件存在并包含日志
        assert!(Path::new(file_path).exists());
        let content = fs::read_to_string(file_path).unwrap();
        assert!(content.contains("test_event"));
        assert!(content.contains("key=value"));

        // 清理临时文件
        fs::remove_file(file_path).unwrap();
    }
}
