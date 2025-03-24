#![deny(clippy::unwrap_used, clippy::expect_used)]
use blake3::hash;

use crate::error::HashError;
use crate::hash::{HashAlgorithm, HashGenerator};

pub struct BLAKE3Generator;

impl HashGenerator for BLAKE3Generator {
    fn new() -> Self {
        BLAKE3Generator
    }

    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError> {
        let hash_result = hash(input.as_bytes());
        Ok(hash_result.as_bytes().to_vec())
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::BLAKE3
    }
}
