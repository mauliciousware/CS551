#![deny(clippy::unwrap_used, clippy::expect_used)]
use digest::Digest;
use sha2::Sha256;

use crate::error::HashError;
use crate::hash::{HashAlgorithm, HashGenerator};

pub struct SHA256Generator;

impl HashGenerator for SHA256Generator {
    fn new() -> Self {
        SHA256Generator
    }

    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError> {
        let mut hasher = Sha256::new();
        hasher.update(input.as_bytes());
        Ok(hasher.finalize().to_vec())
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::SHA256
    }
}
