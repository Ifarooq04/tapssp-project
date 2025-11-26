## VIDEO DEMONSTRATION
Here is the link: https://youtu.be/BqoQh51vrgo


# LockBox — CSC 363/463 Safe Systems Final Project

LockBox is a secure file-encryption command-line tool written in Rust.  
It encrypts and decrypts files using authenticated encryption (AES-256-GCM) and uses the operating system keyring for master-key storage.  
Each encrypted file has its own randomly generated file key, which is “wrapped” (encrypted) using the master key and stored in the file header.

The goal of this project was to build something safe, and realistic that demonstrates secure systems concepts: careful memory handling, key isolation, and misuse-resistant API design.

# Summary

LockBox lets users safely encrypt any file. For each encrypted file, the system creates:

- a unique random 32-byte file key  
- a random 12-byte AEAD nonce  
- a versioned header containing the wrapped file key  
- AES-256-GCM ciphertext with integrity protection  

All sensitive values are wrapped in `Zeroizing<T>` so they are wiped from memory when dropped.  
The master key never touches disk — it always lives inside the OS keyring.

# Why This Project Fits “Safe Systems”

This project demonstrates the main ideas from CSC 363:

- Memory safety through Rust’s ownership system  
- Zeroizing and avoiding unnecessary secret copies  
- Using well-designed crypto instead of custom primitives  
- Header format with versioning for future compatibility  
- Per-file key isolation (compromise of one file doesn’t affect others)  
- Clean error handling without silent failures  
- Proper master-key vs file-key separation  

Overall, it’s a small but solid example of a safe, documented, student-level system.


# Features

### `init`
Initializes and stores a 32-byte master key in the OS keyring.

### `lock <input> <output>`
Encrypts a file:
- generates a fresh file key  
- generates a fresh nonce  
- wraps the file key using AES-256-GCM + master key  
- writes a header  
- encrypts the entire file contents  

### `unlock <input> <output>`
Decrypts a file:
- reads and parses the header  
- unwraps the file key  
- decrypts the ciphertext  
- restores the plaintext file  

**Additional:**
- All secrets zeroized when dropped  
- Consistent `LockBoxError` error type  
- Minimal dependencies and readable code  

# High-Level Design

## 1. Master Key  
Stored using the OS keyring (macOS Keychain, Windows Credential Vault, GNOME Keyring).  
Never written to disk.

## 2. File Keys  
Each encrypted file uses its own randomly generated 32-byte key.

## 3. Header Format  
Every encrypted file begins with:
- magic ID  
- version number  
- nonce (12 bytes)  
- wrapped file key length  
- wrapped file key bytes  

## 4. Encryption Workflow  
1. Read input file  
2. Create file key + nonce  
3. Wrap file key with master key  
4. Write header  
5. Encrypt contents  
6. Save ciphertext  

## 5. Decryption Workflow  
1. Parse header  
2. Recover file key  
3. Decrypt ciphertext  
4. Write output file  


# Usage

### Initialize master key  
cargo run -- init

Encrypt -
cargo run -- lock input.txt output.lockbox

Decrypt -
cargo run -- unlock output.lockbox decrypted.txt

Testing
I tested the system on:
round-trip encrypt → decrypt
invalid key / wrong key behavior
zeroization of plaintext buffers
corrupted header detection
multiple file types
malformed ciphertext detection

Example test:
let key = load_or_create_master_key();
let data = b"super secret";

let encrypted = encrypt_file(data, &*key);
let decrypted = decrypt_file(&encrypted, &*key);

assert_eq!(decrypted.as_ref(), data);



Reflection
Working on this project helped me understand what “secure system design” actually means in practice as it was done here.
A lot of the work wasn’t about writing complicated code, it was about making careful design decisions and avoiding common pitfalls.

What Went Well For Me
Rust’s ownership model made memory safety easier
Zeroizing fit naturally and worked well
AES-GCM gave me confidentiality + integrity without reinventing crypto
Using the OS keyring simplified key management

What Was Challenging
Real secure memory (like mlock) is hard to do portably
Avoiding accidental clones of secret data
Balancing “feature wishlist” vs realistic scope
Understanding key wrapping and header formats

What I Learned
It’s better to do fewer things but do them correctly
OS-level security matters a lot more than I expected
AEAD modes prevent whole classes of integrity bugs
Being explicit about limitations is part of safe design

Future Improvements
If I continue this project, I’d want to add:
streaming/chunked encryption
optional secure memory locking
additional CLI tools
a more formal threat analysis

All in all, I really had a blast with this project. This project meets the goals of the Safe Systems course. It’s designed carefully, avoids unsafe practices, and documents what it does and does not do.
It’s not a massive app, but it’s secure, clean, and easy to understand and I honestly actually enjoyed building it. Thanks!!