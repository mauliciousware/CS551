#![deny(clippy::unwrap_used, clippy::expect_used)]
use argon2::Argon2;

use crate::error::HashError;
use crate::hash::{HashAlgorithm, HashGenerator};

pub struct Argon2Generator {
    salt: [u8; 16],
}

impl HashGenerator for Argon2Generator {
    fn new() -> Self {
        Argon2Generator {
            // TODO: let's have 1
            salt: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15],
        }
    }

    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError> {
        let argon2 = Argon2::default();
        let mut output = [0u8; 32];

        argon2
            .hash_password_into(input.as_bytes(), &self.salt, &mut output)
            .map_err(|e| HashError::HashingFailed(e.to_string()))?;

        Ok(output.to_vec())
    }

    fn algorithm(&self) -> HashAlgorithm {
        HashAlgorithm::Argon2
    }
}
