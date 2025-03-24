#![deny(clippy::unwrap_used, clippy::expect_used)]
use clap::{Arg, ArgMatches, Command};

use crate::error::HashassinResult;
use crate::format::{format_hash, read_hash_file};

pub fn command() -> Command {
    Command::new("dump-hashes")
        .about("Dumps hashes from a binary file to plaintext")
        .arg(
            Arg::new("in-file")
                .long("in-file")
                .value_name("FILE")
                .help("Input file containing hashes to dump")
                .required(true),
        )
}

pub fn execute(matches: &ArgMatches) -> HashassinResult<()> {
    let in_file = matches.get_one::<String>("in-file").ok_or_else(|| {
        crate::error::HashassinError::CliError("Failed to get 'in-file' param value".to_string())
    })?;

    // Read the hash file
    let (version, algorithm, password_length, hashes) = read_hash_file(in_file)?;

    // read file header information as expected format
    println!("VERSION: {}", version);
    println!("ALGORITHM: {}", algorithm);
    println!("PASSWORD LENGTH: {}", password_length);

    // out put each hash
    for hash in &hashes {
        println!("{}", format_hash(hash, &algorithm));
    }

    Ok(())
}
