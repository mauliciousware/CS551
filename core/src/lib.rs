#![deny(clippy::unwrap_used, clippy::expect_used)]

// inport and export for make it easier in important items
pub mod algorithms;
pub mod error;
pub mod hash;
pub mod password;
pub mod threadpool;

pub use algorithms::*;
pub use error::HashError;
pub use hash::{HashAlgorithm, HashGenerator, create_hasher};
pub use password::PasswordGenerator;
pub use threadpool::ThreadedHashGenerator;

// 3rdparty
pub use hex;
