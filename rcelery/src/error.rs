use thiserror::Error;

#[derive(Debug, Error)]
pub enum CeleryError {
    #[error("任务未找到: {0}")]
    TaskNotFound(String),

    #[error("无效参数: {0}")]
    InvalidParam(String),

    #[error("未知错误")]
    Unknown,

    #[error("连接错误: {0}")]
    ConnectionError(String),
}
