#![deny(clippy::unwrap_used, clippy::expect_used)]
use digest::Digest;
use md5::Md5;

use crate::error::HashError;
use crate::hash::{HashAlgorithm, HashGenerator};

pub struct MD5Generator;

impl HashGenerator for MD5Generator {
    fn new() -> Self {
        MD5Generator
    }

    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError> {
        let mut hasher = Md5::new();
        hasher.update(input.as_bytes());
        Ok(hasher.finalize().to_vec())
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::MD5
    }
}
