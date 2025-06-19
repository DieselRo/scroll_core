// ==================================
// src/adk/common/error.rs
// ==================================

use std::fmt;

/// Error type for ADK operations
#[derive(Debug)]
pub enum AdkError {
    /// Invalid request parameters
    InvalidRequest(String),

    /// Error from the LLM model
    ModelError(String),

    /// Session-related error
    SessionError(String),

    /// Database error
    Database(String),

    /// I/O error
    Io(std::io::Error),

    /// Serialization/deserialization error
    Serialization(String),

    /// Tool execution error
    ToolExecution(String),

    /// Authorization error
    Auth(String),

    /// Not found error
    NotFound(String),

    /// Generic error
    Other(String),
}

impl fmt::Display for AdkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdkError::InvalidRequest(msg) => write!(f, "Invalid request: {}", msg),
            AdkError::ModelError(msg) => write!(f, "Model error: {}", msg),
            AdkError::SessionError(msg) => write!(f, "Session error: {}", msg),
            AdkError::Database(msg) => write!(f, "Database error: {}", msg),
            AdkError::Io(err) => write!(f, "IO error: {}", err),
            AdkError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
            AdkError::ToolExecution(msg) => write!(f, "Tool execution error: {}", msg),
            AdkError::Auth(msg) => write!(f, "Auth error: {}", msg),
            AdkError::NotFound(msg) => write!(f, "Not found: {}", msg),
            AdkError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AdkError {}

impl From<std::io::Error> for AdkError {
    fn from(err: std::io::Error) -> Self {
        AdkError::Io(err)
    }
}

impl From<serde_json::Error> for AdkError {
    fn from(err: serde_json::Error) -> Self {
        AdkError::Serialization(err.to_string())
    }
}

impl From<sea_orm::DbErr> for AdkError {
    fn from(err: sea_orm::DbErr) -> Self {
        AdkError::Database(err.to_string())
    }
}

impl From<String> for AdkError {
    fn from(err: String) -> Self {
        AdkError::Other(err)
    }
}

impl From<&str> for AdkError {
    fn from(err: &str) -> Self {
        AdkError::Other(err.to_string())
    }
}
