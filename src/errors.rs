use std::fmt;
use std::io;

#[derive(Debug)]
pub enum ServiceError {
    Io(io::Error),
    SerdeJson(serde_json::Error),
    _NotFound(usize),
    _CommandExecutionError(String),
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ServiceError::Io(err) => write!(f, "I/O error: {}", err),
            ServiceError::SerdeJson(err) => write!(f, "Serialization error: {}", err),
            ServiceError::_NotFound(id) => write!(f, "Service with ID {} not found", id),
            ServiceError::_CommandExecutionError(cmd) => write!(f, "Failed to execute command: {}", cmd),
        }
    }
}

impl From<io::Error> for ServiceError {
    fn from(error: io::Error) -> Self {
        ServiceError::Io(error)
    }
}

impl From<serde_json::Error> for ServiceError {
    fn from(error: serde_json::Error) -> Self {
        ServiceError::SerdeJson(error)
    }
}
