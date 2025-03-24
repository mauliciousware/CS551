#![deny(clippy::unwrap_used, clippy::expect_used)]
use digest::Digest;
use sha2::Sha512;

use crate::error::HashError;
use crate::hash::{HashAlgorithm, HashGenerator};

pub struct SHA512Generator;

impl HashGenerator for SHA512Generator {
    fn new() -> Self {
        SHA512Generator
    }

    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError> {
        let mut hasher = Sha512::new();
        hasher.update(input.as_bytes());
        Ok(hasher.finalize().to_vec())
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::SHA512
    }
}
