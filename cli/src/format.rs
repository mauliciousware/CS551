#![deny(clippy::unwrap_used, clippy::expect_used)]
use std::fs::File;
use std::io::Write;

use hashassin_core::hex;

use crate::error::{HashassinError, HashassinResult};

// file format constants according to requirements
pub const ALGORITHM_NAME_OFFSET: usize = 2;
pub const FILE_VERSION: u8 = 1;

/// for for the hash is like:
/// [Version (1 byte)][Algorithm Length (1 byte)][Algorithm Name][Password Length (1 byte)][Hashes...]
pub fn write_hash_file(
    output_file: &str,
    algorithm: &str,
    password_len: u8,
    hashes: &[Vec<u8>],
) -> HashassinResult<()> {
    let mut output = File::create(output_file)?;

    // 1- output file version
    output.write_all(&[FILE_VERSION])?;

    // 2- Write algorithm name length and name
    let algo_name = algorithm.to_lowercase();
    output.write_all(&[algo_name.len() as u8])?;
    output.write_all(algo_name.as_bytes())?;

    // output password length
    output.write_all(&[password_len])?;

    // output the hashes
    for hash in hashes {
        output.write_all(hash)?;
    }

    Ok(())
}

pub fn read_hash_file(input_file: &str) -> HashassinResult<(u8, String, u8, Vec<Vec<u8>>)> {
    let data = std::fs::read(input_file)?;

    if data.is_empty() {
        return Err(HashassinError::InvalidFileFormat("File is empty".into()));
    }

    // Retrieve the version
    let version = data[0];

    if data.len() < 2 {
        return Err(HashassinError::InvalidFileFormat("File too short".into()));
    }

    // Get the algorithm name length and name
    let algo_length = data[1] as usize;

    if data.len() < ALGORITHM_NAME_OFFSET + algo_length {
        return Err(HashassinError::InvalidFileFormat(
            "File too short for algorithm name".into(),
        ));
    }

    let algorithm = String::from_utf8(
        data[ALGORITHM_NAME_OFFSET..ALGORITHM_NAME_OFFSET + algo_length].to_vec(),
    )?;

    if data.len() < ALGORITHM_NAME_OFFSET + algo_length + 1 {
        return Err(HashassinError::InvalidFileFormat(
            "File too short for password length".into(),
        ));
    }

    // read the password length
    let password_length = data[ALGORITHM_NAME_OFFSET + algo_length];

    // calculate hash length based on algorithm
    let hash_length = get_hash_length(&algorithm)?;

    // extract the hashes
    let hash_start = ALGORITHM_NAME_OFFSET + algo_length + 1;
    let mut hashes = Vec::new();

    let mut offset = hash_start;
    while offset + hash_length <= data.len() {
        let hash = data[offset..offset + hash_length].to_vec();
        hashes.push(hash);
        offset += hash_length;
    }

    Ok((version, algorithm, password_length, hashes))
}

pub fn get_hash_length(algorithm: &str) -> HashassinResult<usize> {
    match algorithm {
        "md5" => Ok(16),
        "sha256" => Ok(32),
        "sha3_512" => Ok(64),
        "scrypt" => Ok(64),
        "sha512" => Ok(64),
        "blake3" => Ok(32),
        "argon2" => Ok(32),
        _ => Err(HashassinError::UnknownAlgorithm(algorithm.to_string())),
    }
}

pub fn format_hash(hash: &[u8], algorithm: &str) -> String {
    if algorithm == "scrypt" {
        match String::from_utf8(hash.to_vec()) {
            Ok(s) => s,
            Err(_) => hex::encode(hash),
        }
    } else {
        hex::encode(hash)
    }
}
