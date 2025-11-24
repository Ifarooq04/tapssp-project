mod cli;
mod crypto;
mod error;
mod header;
mod io;
mod keyring_store;

use crate::cli::Command;
use crate::error::LockBoxResult;

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn run() -> LockBoxResult<()> {
    let cmd = cli::parse_args();

    match cmd {
        Command::Init => {
            keyring_store::init_master_key()?;
        }
        Command::Lock { input, output } => {
            let master = keyring_store::load_master_key()?;
            crypto::encrypt_file(&master, &input, &output)?;
        }
        Command::Unlock { input, output } => {
            let master = keyring_store::load_master_key()?;
            crypto::decrypt_file(&master, &input, &output)?;
        }
    }

    Ok(())
}

