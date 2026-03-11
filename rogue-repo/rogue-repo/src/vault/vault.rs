// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! t1 = Vault, f10 = encrypt_pan, f11 = decrypt_pan

use aes_gcm::{
    aead::{Aead, KeyInit},
    Aes256Gcm,
};
use rand::RngCore;

use super::E3;

const NONCE_LEN: usize = 12;

pub struct t1 {
    cipher: Aes256Gcm,
}

impl t1 {
    pub fn new(key: &[u8; 32]) -> Result<Self, E3> {
        let cipher =
            Aes256Gcm::new_from_slice(key).map_err(|e| E3::Key(format!("invalid key: {}", e)))?;
        Ok(Self { cipher })
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>, E3> {
        let mut nonce = [0u8; NONCE_LEN];
        rand::rngs::OsRng.fill_bytes(&mut nonce);
        let ciphertext = self
            .cipher
            .encrypt((&nonce).into(), plaintext)
            .map_err(|e| E3::Encrypt(e.to_string()))?;
        let mut out = Vec::with_capacity(NONCE_LEN + ciphertext.len());
        out.extend_from_slice(&nonce);
        out.extend_from_slice(&ciphertext);
        Ok(out)
    }

    pub fn decrypt(&self, encrypted: &[u8]) -> Result<Vec<u8>, E3> {
        if encrypted.len() < NONCE_LEN {
            return Err(E3::Ciphertext("too short".into()));
        }
        let (n, ct) = encrypted.split_at(NONCE_LEN);
        self.cipher
            .decrypt(n.into(), ct)
            .map_err(|e| E3::Decrypt(e.to_string()))
    }
}

/// f10 = encrypt_pan, never log plaintext
pub fn f10(v: &t1, pan: &[u8]) -> Result<Vec<u8>, E3> {
    v.encrypt(pan)
}

/// f11 = decrypt_pan, use only when needed for ISO 8583
pub fn f11(v: &t1, enc: &[u8]) -> Result<Vec<u8>, E3> {
    v.decrypt(enc)
}
