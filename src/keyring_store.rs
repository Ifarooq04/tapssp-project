use crate::error::{LockBoxError, LockBoxResult};
use keyring::Entry;
use rand::RngCore;
use zeroize::Zeroizing;

const SERVICE: &str = "lockbox";
const ACCOUNT: &str = "master-key";

pub fn init_master_key() -> LockBoxResult<()> {
    let entry = Entry::new(SERVICE, ACCOUNT)
        .map_err(|e| LockBoxError::Keyring(e.to_string()))?;

    // If a key already exists, do nothing
    if entry.get_password().is_ok() {
        println!("Master key already exists.");
        return Ok(());
    }

    // Create a random 32-byte key
    let mut key = [0u8; 32];
    rand::thread_rng().fill_bytes(&mut key);

    // Store key in keyring
    entry
        .set_password(&base64::encode(key))
        .map_err(|e| LockBoxError::Keyring(e.to_string()))?;

    // Wipe memory after storing
    let key = Zeroizing::new(key);
    drop(key);

    println!("Created new master key.");
    Ok(())
}

pub fn load_master_key() -> LockBoxResult<Zeroizing<Vec<u8>>> {
    let entry = Entry::new(SERVICE, ACCOUNT)
        .map_err(|e| LockBoxError::Keyring(e.to_string()))?;

    let encoded = entry
        .get_password()
        .map_err(|e| LockBoxError::Keyring(e.to_string()))?;

    // Decode from base64
    let bytes = base64::decode(encoded)
        .map_err(|e| LockBoxError::Keyring(e.to_string()))?;

    Ok(Zeroizing::new(bytes))
}
