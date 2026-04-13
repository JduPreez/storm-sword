use serde::{Deserialize, Serialize};
use thiserror::Error;

pub mod models;
pub mod services;

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

pub mod utils {
    use std::env;

    pub fn get_env_var(key: &str) -> Result<String, std::env::VarError> {
        env::var(key)
    }

    pub fn get_env_var_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    pub mod functional {
      #[macro_export]
      macro_rules! partial {
      ($f:expr, [$($bound:expr),+ $(,)?], [$($rest:ident),* $(,)?]) => {
          move |$($rest),*| $f($($bound.clone()),*, $($rest),*)
      };
      ($f:expr, [$($bound:expr),+ $(,)?]) => {
          move |arg| $f($($bound.clone()),*, arg)
      };
    }

    #[macro_export]
    macro_rules! partial_right {
      ($f:expr, [$($bound:expr),+ $(,)?], [$($rest:ident),* $(,)?]) => {
          move |$($rest),*| $f($($rest),*, $($bound.clone()),*)
      };
      ($f:expr, [$($bound:expr),+ $(,)?]) => {
          move |arg| $f(arg, $($bound.clone()),*)
      };
    }

    }
}
