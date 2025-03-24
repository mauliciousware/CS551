# Hashassin Project

This is our password generation and hashing tool for CS551.

## What this thing does

Hashassin lets you:
- Generate random passwords
- Hash passwords using different algorithms
- View the hashes in a readable format

## Algorithms we implemented

We've got several hashing algorithms working:
- MD5
- SHA256 
- SHA512
- SHA3-512
- BLAKE3
- Scrypt 
- Argon2 

## Project Structure

We split the project into two crates:
- `hashassin_core`: All the behind-the-scenes stuff
- `hashassin`: The CLI that users interact with

## Getting Started

### You'll need:
- Rust
- Cargo

### Building it
Just clone and build:
```bash
git clone https://github.com/2025-Spring-CS-551/project-1-biryani-bachata.git
cd project-1-biryani-bachata
cargo build
```

## How to Use It

### Making passwords:
```bash
./target/debug/hashassin gen-passwords --num 10 --chars 12 --threads 4 --out-file passwords.txt
```

### Creating hashes:
```bash
./target/debug/hashassin gen-hashes --in-file passwords.txt --out-file hashes.bin --algorithm sha256 --threads 4
```

### Viewing hashes:
```bash
./target/debug/hashassin dump-hashes --in-file hashes.bin --out-file hashes.txt
```

## Testing

We made a test script that checks all the algorithms:
```bash
./test_algorithms.sh
```

## Libraries we used
- clap: For CLI argument parsing (this library is awesome!)
- rand: For generating random passwords
- Various hashing libraries: md-5, sha2, sha3, blake3, scrypt, argon2
- thiserror: Made error handling way less painful
- hex: For converting bytes to hexadecimal strings
- tracing: For logging (though we didn't use it much in the end)


### Required Features
- ✅ Binary named "hashassin"
- ✅ Command: gen-passwords with required options
- ✅ Command: gen-hashes with required options
- ✅ Command: dump-hashes with required options
- ✅ Support for md5, sha256, sha3 512, and scrypt hash algorithms
- ✅ Binary file format that stores hashes correctly
- ✅ All password lengths must be the same
- ✅ Correct output format for dump-hashes

### COOL STUFF !

- ✅ Modular architecture
- ✅ Support argon2,blake3 and sha512 hash algorithms
- ✅ Automated testing script
- ✅ Centralized Error Handling (cli/src/error.rs)

### Grading Rubric

Required Files
- ✅ HONESTY.md 
- ✅ CREDITS.md 
- ✅ README.md 
- Command: gen-passwords
- ✅ --chars parameter implementation
- ✅ --out-file parameter implementation
- ✅ --threads parameter implementation
- ✅ --num parameter implementation
- Command: gen-hashes
- ✅ --in-file parameter implementation
- ✅ --out-file parameter implementation
- ✅ --threads parameter implementation
- ✅ --algorithm parameter implementation
- Command: dump-hashes
- ✅ --in-file parameter implementation
- Code Quality
- ✅ No cargo check warnings
- ✅ No cargo clippy warnings
- ✅ Comprehensive documentation
- ✅ No unwraps or expects
- ✅ Proper error handling throughout
- ✅ Proper logging with tracing
- Additional Features (Cool Stuff)
- ✅ Modular architecture with clean separation of concerns
- ✅ Additional hash algorithms (argon2, blake3, sha512)
- ✅ Automated testing script for all algorithms
- ✅ Centralized error handling system
- ✅ Generic hash algorithm implementation# CS551
