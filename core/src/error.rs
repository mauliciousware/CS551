#![deny(clippy::unwrap_used, clippy::expect_used)]
use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum HashError {
    #[error("Invalid algorithm specified: {0}")]
    InvalidAlgorithm(String),

    #[error("Hashing failed: {0}")]
    HashingFailed(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}
