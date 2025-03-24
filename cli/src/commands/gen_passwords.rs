#![deny(clippy::unwrap_used, clippy::expect_used)]
use clap::{Arg, ArgMatches, Command};
use hashassin_core::PasswordGenerator;
use std::fs::File;
use std::io::{self, Write};

use crate::error::{HashassinError, HashassinResult};

pub fn command() -> Command {
    Command::new("gen-passwords")
        .about("Generates random passwords")
        .arg(
            Arg::new("chars")
                .long("chars")
                .value_name("CHARS")
                .help("Number of characters in each password (8-bit value)")
                .default_value("4")
                .value_parser(|s: &str| -> Result<u8, String> {
                    s.parse::<u8>()
                        .map_err(|_| "Chars must be a positive 8-bit integer".to_string())
                        .and_then(|n| {
                            if n > 0 {
                                Ok(n)
                            } else {
                                Err("Chars must be greater than 0".to_string())
                            }
                        })
                }),
        )
        .arg(
            Arg::new("out-file")
                .long("out-file")
                .value_name("FILE")
                .help("Output file (default: stdout)"),
        )
        .arg(
            Arg::new("threads")
                .long("threads")
                .value_name("THREADS")
                .help("Number of threads to use")
                .default_value("1")
                .value_parser(|s: &str| -> Result<usize, String> {
                    s.parse::<usize>()
                        .map_err(|_| "Threads must be a positive integer".to_string())
                        .and_then(|n| {
                            if n > 0 {
                                Ok(n)
                            } else {
                                Err("Threads must be between 1 and system maximum".to_string())
                            }
                        })
                }),
        )
        .arg(
            Arg::new("num")
                .long("num")
                .value_name("NUM")
                .help("Number of passwords to generate")
                .required(true)
                .value_parser(|s: &str| -> Result<usize, String> {
                    s.parse::<usize>()
                        .map_err(|_| "Num must be a positive integer".to_string())
                        .and_then(|n| {
                            if n > 0 {
                                Ok(n)
                            } else {
                                Err("Num must be between 1 and system maximum".to_string())
                            }
                        })
                }),
        )
}

pub fn execute(matches: &ArgMatches) -> HashassinResult<()> {
    let chars = matches
        .get_one::<u8>("chars")
        .ok_or(HashassinError::CliError(
            "Failed to get 'chars' value".to_string(),
        ))?;

    let threads = matches
        .get_one::<usize>("threads")
        .ok_or(HashassinError::CliError(
            "Failed to get 'threads' value".to_string(),
        ))?;

    let num = matches
        .get_one::<usize>("num")
        .ok_or(HashassinError::CliError(
            "Failed to get 'num' value".to_string(),
        ))?;

    let passwords = PasswordGenerator::generate_passwords(*num, *chars, *threads)
        .map_err(|e| HashassinError::CoreError(e.to_string()))?;

    let mut writer: Box<dyn Write> = if let Some(out_file) = matches.get_one::<String>("out-file") {
        Box::new(File::create(out_file)?)
    } else {
        Box::new(io::stdout())
    };

    for password in passwords {
        writeln!(writer, "{}", password)?;
    }

    Ok(())
}
