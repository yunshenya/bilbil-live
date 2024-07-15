use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum BilError {
    Serde(serde_json::Error),
    Network(reqwest::Error),
    Params(String),
    IoError(io::Error),
}

impl Display for BilError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let hit = match self {
            BilError::Serde(source) => format!("格式化失败: {}", source),
            BilError::Network(source) => format!("网络错误: {}", source),
            BilError::Params(msg) => msg.to_string(),
            BilError::IoError(source) => format!("io异常: {}", source),
        };
        write!(f, "{}", hit)
    }
}

impl From<reqwest::Error> for BilError {
    fn from(value: reqwest::Error) -> Self {
        BilError::Network(value)
    }
}

impl From<serde_json::Error> for BilError {
    fn from(value: serde_json::Error) -> Self {
        BilError::Serde(value)
    }
}

impl From<io::Error> for BilError {
    fn from(err: io::Error) -> Self {
        BilError::IoError(err)
    }
}

impl From<String> for BilError {
    fn from(value: String) -> Self {
        BilError::Params(value)
    }
}
impl std::error::Error for BilError {}

pub type BilCoreResult<T> = Result<T, BilError>;
