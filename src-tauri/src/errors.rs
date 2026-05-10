use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum AppError {
    #[error("配置读取失败: {0}")]
    ConfigError(String),
    #[error("服务操作失败: {0}")]
    ServiceError(String),
    #[error("系统调用失败: {0}")]
    SystemError(String),
    #[error("更新检查失败: {0}")]
    UpdateError(String),
}
