#![deny(clippy::unwrap_used, clippy::expect_used)]
use clap::{Arg, ArgMatches, Command};
use hashassin_core::{HashAlgorithm, ThreadedHashGenerator};
use std::str::FromStr;

use crate::error::{HashassinError, HashassinResult};
use crate::format::write_hash_file;

pub fn command() -> Command {
    Command::new("gen-hashes")
        .about("Generates hashes")
        .arg(
            Arg::new("in-file")
                .long("in-file")
                .value_name("FILE")
                .help("Input file name containing passwords")
                .required(true),
        )
        .arg(
            Arg::new("out-file")
                .long("out-file")
                .value_name("FILE")
                .help("Output file for hashes")
                .required(true),
        )
        .arg(
            Arg::new("threads")
                .long("threads")
                .value_name("THREADS")
                .help("Number of threads to use")
                .default_value("1")
                .value_parser(|s: &str| -> Result<usize, String> {
                    s.parse::<usize>()
                        .map_err(|_| "Threads has to be a positive integer".to_string())
                        .and_then(|n| {
                            if n > 0 {
                                Ok(n)
                            } else {
                                Err("Threads must be between 1 and system maximum (32 bits/64bits)".to_string())
                            }
                        })
                }),
        )
        .arg(
            Arg::new("algorithm")
                .long("algorithm")
                .value_name("ALGORITHM")
                .help("Hashing algorithm to use (md5, sha256, sha3_512, scrypt, sha512, blake3, argon2)")
                .required(true),
        )
}

pub fn execute(matches: &ArgMatches) -> HashassinResult<()> {
    let in_file = matches
        .get_one::<String>("in-file")
        .ok_or(HashassinError::CliError(
            "Failed to get 'in-file' value".to_string(),
        ))?;

    let out_file = matches
        .get_one::<String>("out-file")
        .ok_or(HashassinError::CliError(
            "Failed to get 'out-file' value".to_string(),
        ))?;

    let threads = matches
        .get_one::<usize>("threads")
        .ok_or(HashassinError::CliError(
            "Failed to get 'threads' value".to_string(),
        ))?;

    let algorithm_str = matches
        .get_one::<String>("algorithm")
        .ok_or(HashassinError::CliError(
            "Failed to get 'algorithm' value".to_string(),
        ))?;

    let algorithm = HashAlgorithm::from_str(algorithm_str)
        .map_err(|_| HashassinError::UnknownAlgorithm(algorithm_str.clone()))?;

    let passwords = std::fs::read_to_string(in_file)?
        .lines()
        .map(String::from)
        .collect::<Vec<String>>();

    let first_len = passwords
        .first()
        .ok_or(HashassinError::EmptyInputFile)?
        .len();

    if passwords.iter().any(|p| p.len() != first_len) {
        return Err(HashassinError::InconsistentPasswordLength);
    }

    let hasher = ThreadedHashGenerator::new(algorithm, *threads);
    let hashes = hasher
        .generate_hashes(passwords)
        .map_err(|e| HashassinError::CoreError(e.to_string()))?;

    // Write the output file
    write_hash_file(out_file, algorithm_str, first_len as u8, &hashes)?;

    Ok(())
}
