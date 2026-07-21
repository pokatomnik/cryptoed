use argon2::{Algorithm, Argon2, Params, Version};
use chacha20poly1305::{
    Key, XChaCha20Poly1305, XNonce,
    aead::{Aead, KeyInit, Payload},
};
use zeroize::Zeroizing;

const MAGIC: &[u8; 4] = b"TUIE";
const FORMAT_VERSION: u8 = 1;

const SALT_LEN: usize = 16;
const NONCE_LEN: usize = 24;
const KEY_LEN: usize = 32;
const AUTH_TAG_LEN: usize = 16;

const HEADER_LEN: usize = MAGIC.len()
    + 1 // version
    + SALT_LEN
    + NONCE_LEN;

// Значения задаём явно, чтобы обновление дефолтов в крейте
// не сделало старые файлы нечитаемыми.
const ARGON2_MEMORY_KIB: u32 = 32 * 1024;
const ARGON2_ITERATIONS: u32 = 2;
const ARGON2_PARALLELISM: u32 = 1;

pub trait Crypto {
    fn encrypt(&self, bytes: &[u8], password: &str) -> anyhow::Result<Vec<u8>>;

    fn decrypt(&self, bytes: &[u8], password: &str) -> anyhow::Result<Vec<u8>>;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PasswordCrypto;

impl PasswordCrypto {
    pub fn new() -> Self {
        Self
    }
}

impl Crypto for PasswordCrypto {
    fn encrypt(&self, bytes: &[u8], password: &str) -> anyhow::Result<Vec<u8>> {
        let mut salt = [0_u8; SALT_LEN];
        let mut nonce_bytes = [0_u8; NONCE_LEN];

        getrandom::fill(&mut salt)?;

        getrandom::fill(&mut nonce_bytes)?;

        let key = derive_key(password, &salt)?;

        let cipher = XChaCha20Poly1305::new(&Key::from(*key));

        let nonce = XNonce::from(nonce_bytes);

        let mut header = Vec::with_capacity(HEADER_LEN);

        header.extend_from_slice(MAGIC);
        header.push(FORMAT_VERSION);
        header.extend_from_slice(&salt);
        header.extend_from_slice(&nonce_bytes);

        let ciphertext = cipher.encrypt(
            &nonce,
            Payload {
                msg: bytes,
                aad: &header,
            },
        )?;

        let mut output = Vec::with_capacity(HEADER_LEN + ciphertext.len());

        output.extend_from_slice(&header);
        output.extend_from_slice(&ciphertext);

        Ok(output)
    }

    fn decrypt(&self, bytes: &[u8], password: &str) -> anyhow::Result<Vec<u8>> {
        // Даже пустой plaintext имеет authentication tag.
        if bytes.len() < HEADER_LEN + AUTH_TAG_LEN {
            anyhow::bail!("Invalid format");
        }

        if &bytes[..MAGIC.len()] != MAGIC {
            anyhow::bail!("Invalid format");
        }

        let version = bytes[MAGIC.len()];

        if version != FORMAT_VERSION {
            anyhow::bail!("Unsupported version: {version}");
        }

        let salt_start = MAGIC.len() + 1;
        let salt_end = salt_start + SALT_LEN;
        let nonce_end = salt_end + NONCE_LEN;

        let salt: &[u8; SALT_LEN] = bytes[salt_start..salt_end].try_into()?;

        let nonce_bytes: [u8; NONCE_LEN] = bytes[salt_end..nonce_end].try_into()?;

        let header = &bytes[..HEADER_LEN];
        let ciphertext = &bytes[HEADER_LEN..];

        let key = derive_key(password, salt)?;

        let cipher = XChaCha20Poly1305::new(&Key::from(*key));

        let nonce = XNonce::from(nonce_bytes);

        let result = cipher.decrypt(
            &nonce,
            Payload {
                msg: ciphertext,
                aad: header,
            },
        )?;
        Ok(result)
    }
}

fn derive_key(password: &str, salt: &[u8]) -> anyhow::Result<Zeroizing<[u8; KEY_LEN]>> {
    let params = Params::new(
        ARGON2_MEMORY_KIB,
        ARGON2_ITERATIONS,
        ARGON2_PARALLELISM,
        Some(KEY_LEN),
    )
    .map_err(|_| anyhow::Error::msg("Invalid params"))?;

    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut key = Zeroizing::new([0_u8; KEY_LEN]);

    argon2
        .hash_password_into(password.as_bytes(), salt, &mut *key)
        .map_err(|_| anyhow::Error::msg("Failed to hash password"))?;

    Ok(key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encrypt_then_decrypt() {
        let crypto = PasswordCrypto;
        let plaintext = b"secret tui state";
        let password = "correct horse battery staple";

        let encrypted = crypto.encrypt(plaintext, password).unwrap();

        let decrypted = crypto.decrypt(&encrypted, password).unwrap();

        assert_eq!(decrypted, plaintext);
        assert_ne!(encrypted, plaintext);
    }

    #[test]
    fn encryption_is_randomized() {
        let crypto = PasswordCrypto;

        let first = crypto.encrypt(b"same data", "same password").unwrap();

        let second = crypto.encrypt(b"same data", "same password").unwrap();

        assert_ne!(first, second);
    }

    #[test]
    fn empty_plaintext_is_supported() {
        let crypto = PasswordCrypto;

        let encrypted = crypto.encrypt(b"", "").unwrap();

        let decrypted = crypto.decrypt(&encrypted, "").unwrap();

        assert!(decrypted.is_empty());
    }
}
