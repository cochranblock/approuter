// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! vault: Encryption/decryption core. "Radioactive Data" — PAN never plaintext.
//! t1 = Vault, f10 = encrypt_pan, f11 = decrypt_pan

mod error;
mod vault;

pub use error::E3;
pub use vault::{f10, f11, t1};
