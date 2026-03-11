// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use aes_gcm::{aead::Aead, Aes256Gcm, KeyInit};
use base64::{engine::general_purpose::STANDARD as b64, Engine};
use rand::RngCore;

use crate::error::t18;

/// f14 = encrypt_token. Why: AES-256-GCM; nonce+ct b64 for storage.
pub fn f14(plaintext: &str, key: &[u8; 32]) -> Result<String, t18> {
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| t18::E5(e.to_string()))?;
    let mut nonce_bytes = [0u8; 12];
    rand::rngs::OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = aes_gcm::Nonce::from_slice(&nonce_bytes);
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_bytes())
        .map_err(|e| t18::E5(e.to_string()))?;
    let mut out = nonce_bytes.to_vec();
    out.extend(ciphertext);
    Ok(b64.encode(&out))
}

/// f15 = decrypt_token. Why: Reverse of f14; auth token decryption.
pub fn f15(ciphertext_b64: &str, key: &[u8; 32]) -> Result<String, t18> {
    let raw = b64.decode(ciphertext_b64).map_err(|e| t18::E5(e.to_string()))?;
    if raw.len() < 13 {
        return Err(t18::E5("Ciphertext too short".into()));
    }
    let (nonce_bytes, ct) = raw.split_at(12);
    let cipher = Aes256Gcm::new_from_slice(key).map_err(|e| t18::E5(e.to_string()))?;
    let nonce = aes_gcm::Nonce::from_slice(nonce_bytes);
    let plaintext = cipher
        .decrypt(nonce, ct)
        .map_err(|_| t18::E5("Decryption failed".into()))?;
    String::from_utf8(plaintext).map_err(|e| t18::E5(e.to_string()))
}
