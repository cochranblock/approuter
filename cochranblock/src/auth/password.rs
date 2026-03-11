// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use argon2::password_hash::{rand_core::OsRng, SaltString};
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};

use crate::error::t18;

/// f16 = hash_password — Argon2id, random salt
pub fn f16(password: &str) -> Result<String, t18> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| t18::E5(e.to_string()))
}

/// f17 = verify_password — Argon2 verify
pub fn f17(password: &str, hash: &str) -> Result<bool, t18> {
    let parsed = PasswordHash::new(hash).map_err(|e| t18::E5(e.to_string()))?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}
