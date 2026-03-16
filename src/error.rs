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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_formatting() {
        assert_eq!(
            MonitorError::WindowsApi("test error".to_string()).to_string(),
            "Windows API error: test error"
        );
        assert_eq!(
            MonitorError::MonitorNotFound("DISPLAY1".to_string()).to_string(),
            "Monitor not found: DISPLAY1"
        );
        assert_eq!(
            MonitorError::UnsupportedOperation("set brightness".to_string()).to_string(),
            "Unsupported operation: set brightness"
        );
        assert_eq!(
            MonitorError::EnumerationFailed.to_string(),
            "Failed to enumerate monitors"
        );
        assert_eq!(
            MonitorError::PhysicalMonitorHandleFailed.to_string(),
            "Failed to get physical monitor handle"
        );
        assert_eq!(
            MonitorError::VcpNotSupported.to_string(),
            "VCP feature not supported"
        );
    }
}
