use crate::error::{LockBoxError, LockBoxResult};
use crate::header::Header;
use crate::io::{open_input, open_output};

use aes_gcm::{Aes256Gcm, Key, Nonce};
use aes_gcm::aead::{Aead, KeyInit};

use rand::RngCore;
use zeroize::Zeroizing;

use std::io::{Read, Write};
use std::path::Path;

// Convert &[u8] to a fixed 32-byte array for AES-256
fn to_key32(bytes: &[u8]) -> Result<[u8; 32], LockBoxError> {
    if bytes.len() != 32 {
        return Err(LockBoxError::Crypto("Key must be 32 bytes".into()));
    }
    let mut arr = [0u8; 32];
    arr.copy_from_slice(bytes);
    Ok(arr)
}

// Encrypt a file
pub fn encrypt_file(
    master_key: &Zeroizing<Vec<u8>>,
    input: &Path,
    output: &Path,
) -> LockBoxResult<()> {
    let mut infile = open_input(input)?;
    let mut outfile = open_output(output)?;

    // Random file key
    let mut file_key = Zeroizing::new(vec![0u8; 32]);
    rand::thread_rng().fill_bytes(&mut file_key[..]);

    // Random nonce
    let mut nonce_bytes = vec![0u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce_bytes[..]);

    // Wrap file key with master key
    let master_key_arr = to_key32(&master_key[..])?;
    let master_cipher =
        Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&master_key_arr));

    let wrapped_key = master_cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), file_key.as_slice())
        .map_err(|e| LockBoxError::Crypto(e.to_string()))?;

    // Write header
    let header = Header {
        nonce: nonce_bytes.clone(),
        wrapped_key,
    };
    header.write_to(&mut outfile)?;

    // Encrypt file contents with file key
    let file_key_arr = to_key32(&file_key[..])?;
    let file_cipher =
        Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&file_key_arr));

    let mut buffer = Vec::new();
    infile.read_to_end(&mut buffer)?;

    let ciphertext = file_cipher
        .encrypt(Nonce::from_slice(&nonce_bytes), buffer.as_ref())
        .map_err(|e| LockBoxError::Crypto(e.to_string()))?;

    outfile.write_all(&ciphertext)?;
    Ok(())
}

// Decrypt a file
pub fn decrypt_file(
    master_key: &Zeroizing<Vec<u8>>,
    input: &Path,
    output: &Path,
) -> LockBoxResult<()> {
    let mut infile = open_input(input)?;
    let mut outfile = open_output(output)?;

    // Read header
    let header = Header::read_from(&mut infile)?;

    // Unwrap file key with master key
    let master_key_arr = to_key32(&master_key[..])?;
    let master_cipher =
        Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&master_key_arr));

    let file_key_bytes = master_cipher
        .decrypt(Nonce::from_slice(&header.nonce), header.wrapped_key.as_ref())
        .map_err(|e| LockBoxError::Crypto(e.to_string()))?;

    let file_key = Zeroizing::new(file_key_bytes);

    let file_key_arr = to_key32(&file_key[..])?;
    let file_cipher =
        Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(&file_key_arr));

    // Read ciphertext
    let mut ciphertext = Vec::new();
    infile.read_to_end(&mut ciphertext)?;

    let plaintext = file_cipher
        .decrypt(Nonce::from_slice(&header.nonce), ciphertext.as_ref())
        .map_err(|e| LockBoxError::Crypto(e.to_string()))?;

    outfile.write_all(&plaintext)?;
    Ok(())
}
