// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum E3 {
    #[error("encryption failed: {0}")]
    Encrypt(String),
    #[error("decryption failed: {0}")]
    Decrypt(String),
    #[error("invalid key: {0}")]
    Key(String),
    #[error("invalid ciphertext: {0}")]
    Ciphertext(String),
}
