//! 错误处理模块

use std::error::Error as StdError;
use std::fmt;
use std::io;
use crate::core::models::entry::Invalid as EntryInvalid;

/// 应用错误类型
#[derive(Debug)]
pub enum Error {
    /// IO错误
    Io(io::Error),
    /// HTTP请求错误
    Http(reqwest::Error),
    /// HTTP错误代码
    HttpError(u16),
    /// 无效的内容类型
    InvalidContentType(String),
    /// 无效的URL
    InvalidUrl(String),
    /// URL解析错误
    UrlParse(url::ParseError),
    /// JSON处理错误
    Json(serde_json::Error),
    /// HTML解析错误
    Html(String),
    /// 解析错误
    ParseError(String),
    /// 文档处理错误
    Doc(String),
    /// Entry错误
    Entry(EntryInvalid),
    /// 通用错误消息
    Message(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Io(err) => write!(f, "IO错误: {}", err),
            Error::Http(err) => write!(f, "HTTP错误: {}", err),
            Error::HttpError(code) => write!(f, "HTTP错误代码: {}", code),
            Error::InvalidContentType(content_type) => {
                write!(f, "无效的内容类型: {}", content_type)
            }
            Error::InvalidUrl(url) => write!(f, "无效的URL: {}", url),
            Error::UrlParse(err) => write!(f, "URL解析错误: {}", err),
            Error::Json(err) => write!(f, "JSON错误: {}", err),
            Error::Html(msg) => write!(f, "HTML错误: {}", msg),
            Error::ParseError(msg) => write!(f, "解析错误: {}", msg),
            Error::Doc(msg) => write!(f, "文档错误: {}", msg),
            Error::Entry(err) => write!(f, "Entry错误: {}", err),
            Error::Message(msg) => write!(f, "{}", msg),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Io(err) => Some(err),
            Error::Http(err) => Some(err),
            Error::Json(err) => Some(err),
            Error::UrlParse(err) => Some(err),
            Error::Html(_)
            | Error::ParseError(_)
            | Error::Doc(_)
            | Error::Message(_)
            | Error::HttpError(_)
            | Error::InvalidContentType(_)
            | Error::InvalidUrl(_)
            | Error::Entry(_) => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Error::Io(err)
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Json(err)
    }
}

impl From<String> for Error {
    fn from(msg: String) -> Self {
        Error::Message(msg)
    }
}

impl From<&str> for Error {
    fn from(msg: &str) -> Self {
        Error::Message(msg.to_string())
    }
}

impl From<Box<dyn StdError>> for Error {
    fn from(err: Box<dyn StdError>) -> Self {
        Error::Message(err.to_string())
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::UrlParse(err)
    }
}

impl From<EntryInvalid> for Error {
    fn from(err: EntryInvalid) -> Self {
        Error::Entry(err)
    }
}

/// 应用结果类型
pub type Result<T> = std::result::Result<T, Error>;
