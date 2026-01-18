use crate::secret::SecretError;
use argon2::Argon2;
use base64::Engine;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};
use rand::RngCore;

pub struct Crypto {
    cipher: ChaCha20Poly1305,
}

impl Crypto {
    // Fixed 32-byte salt
    const SALT: &'static [u8] = b"vaulter_fixed_salt_v1_no_change!";

    pub fn new(passphrase: &str) -> Result<Self, SecretError> {
        // Derive a 32-byte key from the passphrase using Argon2
        let argon2 = Argon2::default();
        let mut key_bytes = [0u8; 32];

        argon2
            .hash_password_into(passphrase.as_bytes(), Self::SALT, &mut key_bytes)
            .map_err(|e| SecretError::CryptoErr(format!("Failed to derive key: {}", e)))?;

        let cipher = ChaCha20Poly1305::new(Key::from_slice(&key_bytes));

        Ok(Crypto { cipher })
    }

    pub fn encrypt(&self, plaintext: &str) -> Result<String, SecretError> {
        // Generate a random nonce (12 bytes for ChaCha20Poly1305)
        let mut nonce_bytes = [0u8; 12];
        OsRng.fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = self
            .cipher
            .encrypt(nonce, plaintext.as_bytes())
            .map_err(|e| SecretError::CryptoErr(format!("Encryption failed: {}", e)))?;

        // Combine nonce + ciphertext and encode as base64
        let mut combined = nonce_bytes.to_vec();
        combined.extend_from_slice(&ciphertext);

        Ok(base64::engine::general_purpose::STANDARD.encode(&combined))
    }

    pub fn decrypt(&self, encrypted: &str) -> Result<String, SecretError> {
        use base64::Engine;

        let combined = base64::engine::general_purpose::STANDARD
            .decode(encrypted)
            .map_err(|e| SecretError::CryptoErr(format!("Base64 decode failed: {}", e)))?;

        // Split nonce and ciphertext
        if combined.len() < 12 {
            return Err(SecretError::CryptoErr("Invalid encrypted data".to_string()));
        }

        let (nonce_bytes, ciphertext) = combined.split_at(12);
        let nonce = Nonce::from_slice(nonce_bytes);

        let plaintext = self
            .cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| SecretError::CryptoErr(format!("Decryption failed: {}", e)))?;

        String::from_utf8(plaintext)
            .map_err(|e| SecretError::CryptoErr(format!("Invalid UTF-8: {}", e)))
    }
}
