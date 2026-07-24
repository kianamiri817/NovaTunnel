use chacha20poly1305::{ChaCha20Poly1305, Key, KeyInit, Nonce};
use chacha20poly1305::aead::Aead;
use hkdf::Hkdf;
use sha2::Sha256;
use crate::error::Result;

pub struct NovaCrypto {
    encrypt_key: [u8; 32],
    decrypt_key: [u8; 32],
    encrypt_counter: u64,
    decrypt_counter: u64,
}

impl NovaCrypto {
    pub fn new(shared_secret: [u8; 32]) -> Self {
        // Derive separate encryption and decryption keys
        let hk = Hkdf::<Sha256>::new(None, &shared_secret);
        
        let mut encrypt_key = [0u8; 32];
        let mut decrypt_key = [0u8; 32];
        
        hk.expand(b"novatunnel-encrypt", &mut encrypt_key)
            .expect("Failed to derive encryption key");
        hk.expand(b"novatunnel-decrypt", &mut decrypt_key)
            .expect("Failed to derive decryption key");
        
        Self {
            encrypt_key,
            decrypt_key,
            encrypt_counter: 0,
            decrypt_counter: 0,
        }
    }

    pub fn encrypt(&mut self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.encrypt_key));
        
        // Use counter as nonce (12 bytes)
        let nonce_bytes = self.encrypt_counter.to_le_bytes();
        let mut nonce = [0u8; 12];
        nonce[..8].copy_from_slice(&nonce_bytes);
        
        let nonce = Nonce::from_slice(&nonce);
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| crate::error::Error::Crypto(e.to_string()))?;
        
        self.encrypt_counter += 1;
        
        Ok(ciphertext)
    }

    pub fn decrypt(&mut self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.decrypt_key));
        
        // Use counter as nonce (12 bytes)
        let nonce_bytes = self.decrypt_counter.to_le_bytes();
        let mut nonce = [0u8; 12];
        nonce[..8].copy_from_slice(&nonce_bytes);
        
        let nonce = Nonce::from_slice(&nonce);
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| crate::error::Error::Crypto(e.to_string()))?;
        
        self.decrypt_counter += 1;
        
        Ok(plaintext)
    }

    pub fn encrypt_with_counter(&self, plaintext: &[u8], counter: u64) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.encrypt_key));
        
        let nonce_bytes = counter.to_le_bytes();
        let mut nonce = [0u8; 12];
        nonce[..8].copy_from_slice(&nonce_bytes);
        
        let nonce = Nonce::from_slice(&nonce);
        let ciphertext = cipher.encrypt(nonce, plaintext)
            .map_err(|e| crate::error::Error::Crypto(e.to_string()))?;
        
        Ok(ciphertext)
    }

    pub fn decrypt_with_counter(&self, ciphertext: &[u8], counter: u64) -> Result<Vec<u8>> {
        let cipher = ChaCha20Poly1305::new(Key::from_slice(&self.decrypt_key));
        
        let nonce_bytes = counter.to_le_bytes();
        let mut nonce = [0u8; 12];
        nonce[..8].copy_from_slice(&nonce_bytes);
        
        let nonce = Nonce::from_slice(&nonce);
        let plaintext = cipher.decrypt(nonce, ciphertext)
            .map_err(|e| crate::error::Error::Crypto(e.to_string()))?;
        
        Ok(plaintext)
    }
}
