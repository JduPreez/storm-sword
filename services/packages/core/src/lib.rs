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

/// Common utilities
pub mod utils {
    use std::env;

    pub fn get_env_var(key: &str) -> Result<String, std::env::VarError> {
        env::var(key)
    }

    pub fn get_env_var_or(key: &str, default: &str) -> String {
        env::var(key).unwrap_or_else(|_| default.to_string())
    }

    pub mod functional {}
}

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

#[cfg(test)]
mod functional_tests {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn format_name(first: String, last: String, suffix: String) -> String {
        format!("{} {}{}", first, last, suffix)
    }

    #[test]
    fn left_partial_single_remaining_arg() {
        let add_10 = partial!(add, [10]);

        assert_eq!(add_10(5), 15);
    }

    #[test]
    fn right_partial_single_remaining_arg() {
        let plus_10 = partial_right!(add, [10]);

        assert_eq!(plus_10(5), 15);
    }

    #[test]
    fn left_partial_multi_remaining_args() {
        let with_first_name = partial!(format_name, [String::from("Ada")], [last, suffix]);

        assert_eq!(
            with_first_name(String::from("Lovelace"), String::from("!")),
            String::from("Ada Lovelace!")
        );
    }

    #[test]
    fn left_partial_multi_initial_args() {
        let with_first_name = partial!(
          format_name, [String::from("Ada"), String::from("Lovelace")], [suffix]);

        assert_eq!(
            with_first_name(String::from("!")),
            String::from("Ada Lovelace!")
        );
    }

    #[test]
    fn right_partial_multi_remaining_args() {
        let with_suffix = partial_right!(format_name, [String::from("!")], [first, last]);

        assert_eq!(
            with_suffix(String::from("Ada"), String::from("Lovelace")),
            String::from("Ada Lovelace!")
        );
    }
}