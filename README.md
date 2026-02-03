LockBox — Secure File Encryption (Rust)
Video Demonstration: https://youtu.be/BqoQh51vrgo

LockBox is a secure file-encryption command-line tool written in Rust.
It encrypts and decrypts files using authenticated encryption (AES-256-GCM) and stores a master key securely in the operating system keyring.
Each encrypted file uses its own randomly generated file key. That file key is encrypted (“wrapped”) using the master key and stored in a versioned file header, ensuring key isolation and forward compatibility.
This project was built as the final project for CSC 363/463 – Safe Systems and focuses on correctness, memory safety, and misuse-resistant design rather than feature count.

**Project Goals**
The goal of LockBox was to build a realistic secure system, not a toy example. The design emphasizes:
Memory safety through Rust’s ownership model
Proper cryptographic design using vetted primitives
Explicit key separation and isolation
Clear failure modes and error handling
Avoidance of common security pitfalls

**System Overview**
For each encrypted file, LockBox generates and stores:
A unique random 32-byte file key
A random 12-byte AEAD nonce
A versioned header containing the wrapped file key
AES-256-GCM ciphertext with integrity protection
All sensitive values are wrapped in Zeroizing<T> so secrets are wiped from memory when dropped.
The master key never touches disk and always resides inside the OS keyring.

**Why This Fits “Safe Systems”**
LockBox demonstrates core safe-systems principles:
Memory safety enforced by Rust
Explicit zeroization of secret material
Use of authenticated encryption (AEAD) instead of custom crypto
Versioned file format for future extensibility
Per-file key isolation (compromise of one file does not affect others)
Clean error handling without silent failures
Clear separation between master keys and file keys

**Features**
_init_
    Initializes and securely stores a 32-byte master key in the OS keyring.
_lock <input> <output>_

Encrypts a file by:
- generating a fresh file key
- generating a fresh nonce
- wrapping the file key using the master key
- writing a structured header
- encrypting the file contents
_unlock <input> <output>_

Decrypts a file by:
- parsing the header
- unwrapping the file key
- decrypting the ciphertext
- restoring the plaintext file
  
Additional:
- Automatic zeroization of secrets
- Centralized LockBoxError error type
- Minimal dependencies and readable code

**High-Level Design**
1. Master Key
- Stored in the OS keyring (macOS Keychain, Windows Credential Vault, GNOME Keyring).
- Never written to disk.
2. File Keys
Each encrypted file uses its own randomly generated 32-byte key.
3. File Header
- Each encrypted file begins with:
- magic identifier
- version number
- nonce (12 bytes)
- wrapped file key length
- wrapped file key bytes
4. Encryption Flow
- Read input file
- Generate file key and nonce
- Wrap file key with master key
- Write header
- Encrypt contents
- Save ciphertext
5. Decryption Flow
- Parse header
- Recover file key
- Decrypt ciphertext
- Write output file


**Usage**
Initialize master key:
cargo run -- init

Encrypt a file:
cargo run -- lock input.txt output.lockbox

Decrypt a file:
cargo run -- unlock output.lockbox decrypted.txt

**Testing**
- Tested scenarios include:
- Round-trip encryption and decryption
- Invalid or missing master key behavior
- Zeroization of plaintext buffers
- Corrupted header detection
- Multiple file types
- Malformed ciphertext detection
Code Example:
let key = load_or_create_master_key();
let data = b"super secret";

let encrypted = encrypt_file(data, &*key);
let decrypted = decrypt_file(&encrypted, &*key);

assert_eq!(decrypted.as_ref(), data);


**Reflection**
This project clarified what “secure system design” means in practice. Most of the work was not writing complex code, but making careful decisions and avoiding unsafe assumptions.

-- What went well
- Rust’s ownership model simplified memory safety
- Zeroizing integrated naturally
- AEAD provided confidentiality and integrity
- OS keyrings simplified key management
  
-- Challenges
- Portable secure memory handling
- Avoiding accidental secret cloning
- Balancing scope vs correctness
- Designing key wrapping and file formats

-- Future Improvements
- Streaming or chunked encryption
- Optional secure memory locking
- Additional CLI tooling
- Formal threat modeling

**Takeaway**
LockBox is intentionally small, focused, and carefully designed.
It demonstrates safe systems principles through real implementation choices rather than theoretical discussion.


** Security Notes & Limitations **
This project is designed as a learning-focused secure system and makes the following assumptions:
- Files are encrypted and decrypted in-memory rather than streamed
- Secure memory locking (e.g., mlock) is not used for portability reasons
- The threat model assumes a trusted local OS and keyring
- Side-channel resistance is out of scope

These tradeoffs were made intentionally to prioritize correctness, clarity, and safe design.
