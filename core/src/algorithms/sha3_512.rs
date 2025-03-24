#![deny(clippy::unwrap_used, clippy::expect_used)]
use digest::Digest;
use sha3::Sha3_512;

use crate::error::HashError;
use crate::hash::{HashAlgorithm, HashGenerator};

pub struct SHA3_512Generator;

impl HashGenerator for SHA3_512Generator {
    fn new() -> Self {
        SHA3_512Generator
    }

    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError> {
        let mut hasher = Sha3_512::new();
        hasher.update(input.as_bytes());
        Ok(hasher.finalize().to_vec())
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::SHA3_512
    }
}
