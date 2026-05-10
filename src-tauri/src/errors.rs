use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error, Serialize)]
#[serde(tag = "code", content = "message")]
pub enum AppError {
    #[error("配置读取失败: {0}")]
    ConfigError(String),
    #[error("请先配置有效的上报服务器地址")]
    MissingServerConfig(String),
    #[error("请先配置设备密钥")]
    MissingDeviceKey(String),
    #[error("设备密钥无效: {0}")]
    InvalidKey(String),
    #[error("设备密钥已被撤销: {0}")]
    KeyRevoked(String),
    #[error("设备不存在: {0}")]
    DeviceNotFound(String),
    #[error("该密钥已绑定到其他设备，需确认接管: {0}")]
    TakeoverRequired(String),
    #[error("网络请求失败: {0}")]
    NetworkError(String),
    #[error("服务操作失败: {0}")]
    ServiceError(String),
    #[error("系统调用失败: {0}")]
    SystemError(String),
    #[error("更新检查失败: {0}")]
    UpdateError(String),
}
