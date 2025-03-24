#![deny(clippy::unwrap_used, clippy::expect_used)]
use scrypt::{Params as ScryptParams, scrypt};

use crate::error::HashError;
use crate::hash::{HashAlgorithm, HashGenerator};

pub struct ScryptGenerator {
    params: ScryptParams,
}

impl HashGenerator for ScryptGenerator {
    fn new() -> Self {
        let params = ScryptParams::new(14, 8, 1, 64).unwrap_or_else(|_| {
            ScryptParams::new(10, 8, 1, 32).unwrap_or_else(|_| ScryptParams::recommended())
        });

        ScryptGenerator { params }
    }

    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError> {
        let mut output = vec![0u8; 64];
        let salt = b"randomsalt";

        scrypt(input.as_bytes(), salt, &self.params, &mut output)
            .map_err(|e| HashError::HashingFailed(e.to_string()))?;

        Ok(output)
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::Scrypt
    }
}
