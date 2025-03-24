#![deny(clippy::unwrap_used, clippy::expect_used)]

mod argon2;
mod blake3;
mod md5;
mod scrypt;
mod sha256;
mod sha3_512;
mod sha512;

pub use argon2::Argon2Generator;
pub use blake3::BLAKE3Generator;
pub use md5::MD5Generator;
pub use scrypt::ScryptGenerator;
pub use sha3_512::SHA3_512Generator;
pub use sha256::SHA256Generator;
pub use sha512::SHA512Generator;
