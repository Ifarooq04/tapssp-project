# tapssp-project
Final project for CSC363/463 Safe Systems
# tapssp-project — Name: LockBox

## Summary: One paragraph
LockBox is a Rust CLI that encrypts and decrypts files using AEAD with per-file random keys wrapped by a master key in the OS keyring. It focuses on safe key handling, zeroizing secrets, and a minimal, hard-to-misuse API.

## Why Choosing systems
- Careful memory ownership of secrets with `zeroize`.
- Streams large files with bounded memory.
- OS integration for key storage.

## MVP
- `lock <in> <out>`, `unlock <in> <out>`.
- Authenticated encryption, random nonces, versioned header.
- Constant-time comparisons, secure wipe on errors.

## Crates
- `ring` or `aes-gcm`, `rand`, `clap`, `zeroize`.
