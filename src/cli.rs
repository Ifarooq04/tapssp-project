use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "lockbox", about = "Encrypt and decrypt files safely")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Initializing master key in OS keyring
    Init,

    /// Encrypting a file
    Lock {
        input: PathBuf,
        output: PathBuf,
    },

    /// Decrypting a file
    Unlock {
        input: PathBuf,
        output: PathBuf,
    },
}

pub fn parse_args() -> Command {
    Cli::parse().command
}
