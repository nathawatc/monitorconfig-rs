use thiserror::Error;

#[derive(Error, Debug)]
pub enum MonitorError {
    #[error("Windows API error: {0}")]
    WindowsApi(String),

    #[error("Monitor not found: {0}")]
    MonitorNotFound(String),

    #[error("Unsupported operation: {0}")]
    UnsupportedOperation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Failed to enumerate monitors")]
    EnumerationFailed,

    #[error("Failed to get physical monitor handle")]
    PhysicalMonitorHandleFailed,

    #[error("VCP feature not supported")]
    VcpNotSupported,
}

pub type Result<T> = std::result::Result<T, MonitorError>;
