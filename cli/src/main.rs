#![deny(clippy::unwrap_used, clippy::expect_used)]
use clap::Command;

mod commands;
mod error;
mod format;

use commands::{dump_hashes, gen_hashes, gen_passwords};
use error::HashassinResult;

fn main() {
    if let Err(err) = run() {
        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn run() -> HashassinResult<()> {
    let matches = setup_cli().get_matches();

    match matches.subcommand() {
        Some(("gen-passwords", sub_matches)) => gen_passwords::execute(sub_matches),
        Some(("gen-hashes", sub_matches)) => gen_hashes::execute(sub_matches),
        Some(("dump-hashes", sub_matches)) => dump_hashes::execute(sub_matches),
        _ => {
            println!("Please specify a valid subcommand");
            std::process::exit(1);
        }
    }
}

fn setup_cli() -> Command {
    Command::new("hashassin")
        .version("0.1.0")
        .author("Shree-Rafy")
        .about("Generates random passwords and hashes")
        .subcommand(gen_passwords::command())
        .subcommand(gen_hashes::command())
        .subcommand(dump_hashes::command())
}
