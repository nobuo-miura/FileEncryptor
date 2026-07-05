# FileEncryptor

[English](README.md) | [日本語](README.ja.md)

A secure and user-friendly file encryption and decryption tool built with Rust.  
Supports both a command-line interface (CLI) and a graphical user interface (GUI).

## ✨ Features

- 🔐 AES-256-GCM encryption
- 🛡️ Password-based key derivation using Argon2
- 📁 File selection and saving through GUI
- 🧑‍💻 CLI support for scripting and automation
- 🚀 Cross-platform with minimal dependencies

## 🛠️ Installation

```bash
git clone https://github.com/nobuo-miura/FileEncryptor.git
cd FileEncryptor
cargo build --release
```

## 🔧 Usage

### CLI

```bash
# Encrypt a file
cargo run -- encrypt <input_file> <output_file> <password>

# Decrypt a file
cargo run -- decrypt <input_file> <output_file> <password>

# Launch GUI
cargo run -- gui
```

### GUI

Just run:

```bash
cargo run -- gui
```

You'll be able to select files, enter a password, and encrypt/decrypt files with a simple interface.

## 📂 Folder Structure

```
.
├── src/
│   ├── main.rs      # CLI entry point
│   ├── gui.rs       # GUI implementation
│   └── crypto.rs    # Encryption/decryption logic
└── Cargo.toml
```

## 🔒 Security Notes

- Uses AES-256-GCM with random nonce and salt per encryption
- Keys are derived using Argon2 with a unique salt

## 📃 License

MIT License
