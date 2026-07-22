# CryptoEd

CryptoEd is a lightweight terminal text editor for plain and password-protected files. It combines editing, Markdown preview, encryption, and decryption in one local workflow without requiring a GUI, cloud account, or background service.

Files open in preview mode by default. You can switch to the editor, make changes, preview the rendered Markdown, and save the result without leaving the terminal.

## Why This Project Exists

I created CryptoEd with two goals:

1. Build a simple terminal text editor with first-class Markdown support. CryptoEd provides a focused writing interface and a rendered preview without requiring a graphical editor.
2. Add transparent symmetric encryption while keeping decrypted documents off the network and off persistent storage.

For encrypted files, a key is derived from the password with **Argon2id**, using a fresh random salt. The document is then protected with **XChaCha20-Poly1305**, an authenticated symmetric cipher that encrypts the content and detects tampering. A new random 24-byte nonce is generated on every save, and the file header is authenticated together with the ciphertext.

> **The core rule:** decrypted data exists only in the application's volatile memory. CryptoEd never sends document contents over the network and never creates a temporary plaintext file. When saving an encrypted document, it encrypts the in-memory text first and writes only authenticated ciphertext to disk.

This is an application-level guarantee. Operating-system facilities such as swap, hibernation, crash dumps, or process inspection are outside CryptoEd's control and should be configured appropriately for stricter threat models.

## What It Solves

- Keeps private notes encrypted at rest.
- Removes the need to decrypt a file manually before editing and encrypt it again afterward.
- Provides a focused Markdown writing and preview workflow in the terminal.
- Opens existing files or starts new ones using the same command.
- Supports a configurable encrypted-file extension.

## Installation

CryptoEd requires a current stable Rust toolchain.

Build an optimized binary from the repository:

```shell
cargo build --release
./target/release/cryptoed --help
```

Alternatively, install it into Cargo's binary directory:

```shell
cargo install --path .
```

## Usage

Open or create a plain-text file:

```shell
cryptoed notes.md
```

Files with the `.enc` extension are encrypted by default. CryptoEd asks for a password before opening or creating them:

```shell
cryptoed private-notes.enc
```

Use `-e` or `--encrypted-ext` to select another encrypted-file extension:

```shell
cryptoed --encrypted-ext secure private-notes.secure
cryptoed -e vault passwords.vault
```

Use the same extension option every time you reopen the file. Files whose extension does not match the configured encrypted extension are treated as plain text.

To run without installing:

```shell
cargo run -- notes.md
cargo run -- -e secure private-notes.secure
```

## Keyboard Shortcuts

| Shortcut | Action |
| --- | --- |
| `Ctrl+R` | Switch between Markdown preview and editing |
| `Ctrl+S` | Save the current document |
| `Ctrl+X` | Exit; prompts when there are unsaved changes |
| `F2` | Toggle terminal text-selection mode |

## Encryption Notes

Encrypted files use Argon2id for password-based key derivation and XChaCha20-Poly1305 for authenticated encryption. A fresh random salt and nonce are generated on every save. CryptoEd works locally and does not create a temporary plaintext file, but decrypted content is necessarily held in memory while the editor is running.

There is no password recovery mechanism. Keep backups and store passwords safely. CryptoEd has not undergone an independent security audit and should not replace a dedicated secrets manager for high-risk credentials.

## Author

**Danilian Akhmedzianov**

- Email: [hugefast@gmail.com](mailto:hugefast@gmail.com)
- Telegram: [@jesusscript](https://t.me/jesusscript)

## License

CryptoEd is free software licensed under the [GNU General Public License, version 3 or later](LICENSE).
