#![deny(clippy::unwrap_used, clippy::expect_used)]
use std::str::FromStr;

use crate::algorithms::*;
use crate::error::HashError;

#[derive(Debug, Clone, Copy)]
pub enum HashAlgorithm {
    MD5,
    SHA256,
    SHA3_512,
    Scrypt,
    SHA512,
    BLAKE3,
    Argon2,
}

impl PartialEq for HashAlgorithm {
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl Eq for HashAlgorithm {}

impl FromStr for HashAlgorithm {
    type Err = HashError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "md5" => Ok(HashAlgorithm::MD5),
            "sha256" => Ok(HashAlgorithm::SHA256),
            "sha3_512" => Ok(HashAlgorithm::SHA3_512),
            "scrypt" => Ok(HashAlgorithm::Scrypt),
            "sha512" => Ok(HashAlgorithm::SHA512),
            "blake3" => Ok(HashAlgorithm::BLAKE3),
            "argon2" => Ok(HashAlgorithm::Argon2),
            _ => Err(HashError::InvalidAlgorithm(s.to_string())),
        }
    }
}

// Cool trait to hadle hashes isolated from everything else
pub trait HashGenerator {
    fn new() -> Self
    where
        Self: Sized;
    fn generate_hash(&self, input: &str) -> Result<Vec<u8>, HashError>;
    fn algorithm(&self) -> HashAlgorithm;
}

pub fn create_hasher(algorithm: HashAlgorithm) -> Result<Box<dyn HashGenerator>, HashError> {
    match algorithm {
        HashAlgorithm::MD5 => Ok(Box::new(MD5Generator::new())),
        HashAlgorithm::SHA256 => Ok(Box::new(SHA256Generator::new())),
        HashAlgorithm::SHA3_512 => Ok(Box::new(SHA3_512Generator::new())),
        HashAlgorithm::Scrypt => Ok(Box::new(ScryptGenerator::new())),
        HashAlgorithm::SHA512 => Ok(Box::new(SHA512Generator::new())),
        HashAlgorithm::BLAKE3 => Ok(Box::new(BLAKE3Generator::new())),
        HashAlgorithm::Argon2 => Ok(Box::new(Argon2Generator::new())),
    }
}
