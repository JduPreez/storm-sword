use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Resource not found: {0}")]
    NotFound(String),
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    #[error("Internal error: {0}")]
    Internal(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

impl ErrorResponse {
    pub fn new(error: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error: error.into(),
            message: message.into(),
        }
    }
}

/// Common utilities
pub mod utils {
    use std::env;

    pub fn get_env_var(key: &str) -> Result<String, std::env::VarError> {
        env::var(key)
    }

    pub fn get_env_var_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }
}