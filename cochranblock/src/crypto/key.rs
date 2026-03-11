// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use hkdf::Hkdf;
use sha2::Sha256;

use crate::error::t18;

/// f31 = key_derive — HKDF, context-specific. Why: Kova forbids unwrap in lib; expand can fail.
pub fn f31(master: &str, context: &str) -> Result<[u8; 32], t18> {
    let salt = b"portfolio-salt";
    let hk = Hkdf::<Sha256>::new(Some(salt), master.as_bytes());
    let mut out = [0u8; 32];
    hk.expand(context.as_bytes(), &mut out).map_err(|e| t18::E5(e.to_string()))?;
    Ok(out)
}

/// f32 = key_derive_default — HKDF, default context. Why: session signing key derivation.
pub fn f32(master: &str) -> Result<[u8; 32], t18> {
    f31(master, "session-signing-key-v1")
}
