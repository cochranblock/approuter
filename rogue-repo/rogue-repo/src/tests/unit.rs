// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f49 = unit tests: vault round-trip, tamper rejection; switch f12

use std::time::Instant;

use crate::switch::{f12, t2};
use crate::vault::{f10, f11, t1};

use crate::tests::t24;

pub fn f49() -> Vec<t24> {
    vec![
        vault_round_trip(),
        vault_tamper_rejected(),
        vault_wrong_key(),
        vault_nonce_uniqueness(),
        vault_decrypt_short_fails(),
        vault_empty_plaintext(),
        switch_build_0200(),
        switch_invalid_amount(),
        switch_amount_in_output(),
        switch_currency_840_in_output(),
        switch_amount_out_of_range_fails(),
        switch_pan_too_long_fails(),
    ]
}

fn vault_round_trip() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = match t1::new(&key) {
        Ok(x) => x,
        Err(e) => {
            return t24 {
                name: "vault_round_trip".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("vault new: {}", e)),
            };
        }
    };
    let plain = b"4111111111111111";
    let enc = match f10(&v, plain) {
        Ok(x) => x,
        Err(e) => {
            return t24 {
                name: "vault_round_trip".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("encrypt: {}", e)),
            };
        }
    };
    let dec = match f11(&v, &enc) {
        Ok(x) => x,
        Err(e) => {
            return t24 {
                name: "vault_round_trip".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("decrypt: {}", e)),
            };
        }
    };
    let ok = dec == plain;
    t24 {
        name: "vault_round_trip".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("decrypted != plain".into())
        },
    }
}

fn vault_tamper_rejected() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let plain = b"4111111111111111";
    let mut enc = f10(&v, plain).unwrap();
    if enc.len() > 20 {
        enc[20] = enc[20].wrapping_add(1);
    }
    let dec = f11(&v, &enc);
    let ok = dec.is_err();
    t24 {
        name: "vault_tamper_rejected".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("tampered ciphertext should fail".into())
        },
    }
}

fn vault_wrong_key() -> t24 {
    let start = Instant::now();
    let key1 = [0u8; 32];
    let key2 = [1u8; 32];
    let v1 = t1::new(&key1).unwrap();
    let v2 = t1::new(&key2).unwrap();
    let plain = b"4111111111111111";
    let enc = f10(&v1, plain).unwrap();
    let dec = f11(&v2, &enc);
    let ok = dec.is_err();
    t24 {
        name: "vault_wrong_key".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("wrong key should fail".into())
        },
    }
}

fn switch_build_0200() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: b"encrypted_pan_data".to_vec(),
        amount_cents: 420,
        stan: 12345,
    };
    let msg = match f12(&req) {
        Ok(m) => m,
        Err(e) => {
            return t24 {
                name: "switch_build_0200".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("f12: {}", e)),
            };
        }
    };
    let raw = &msg.raw;
    let has_mti = raw.starts_with(b"0200");
    let has_bitmap = raw.len() >= 12 && raw[4..12].iter().any(|&b| b != 0);
    let has_amount = raw.windows(12).any(|w| w == b"000000000420");
    let ok = has_mti && has_bitmap && has_amount;
    t24 {
        name: "switch_build_0200".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("MTI+bitmap+amount in 0200".into())
        },
    }
}

fn vault_nonce_uniqueness() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let plain = b"4111111111111111";
    let enc1 = f10(&v, plain).unwrap();
    let enc2 = f10(&v, plain).unwrap();
    let ok = enc1 != enc2;
    t24 {
        name: "vault_nonce_uniqueness".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("same plaintext must produce different ciphertext".into())
        },
    }
}

fn vault_decrypt_short_fails() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let short = [0u8; 8];
    let r = f11(&v, &short);
    let ok = r.is_err();
    t24 {
        name: "vault_decrypt_short_fails".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("ciphertext < nonce len must fail".into())
        },
    }
}

fn vault_empty_plaintext() -> t24 {
    let start = Instant::now();
    let key = [0u8; 32];
    let v = t1::new(&key).unwrap();
    let enc = f10(&v, &[]).unwrap();
    let dec = f11(&v, &enc).unwrap();
    let ok = dec.is_empty();
    t24 {
        name: "vault_empty_plaintext".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("encrypt/decrypt empty round-trip".into())
        },
    }
}

fn switch_invalid_amount() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![],
        amount_cents: 0,
        stan: 0,
    };
    let r = f12(&req);
    let ok = r.is_err();
    t24 {
        name: "switch_invalid_amount".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("amount 0 should fail".into())
        },
    }
}

fn switch_amount_in_output() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![1, 2, 3],
        amount_cents: 420,
        stan: 999999,
    };
    let msg = match f12(&req) {
        Ok(m) => m,
        Err(e) => {
            return t24 {
                name: "switch_amount_in_output".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("f12: {}", e)),
            };
        }
    };
    let raw = String::from_utf8_lossy(&msg.raw);
    let ok = raw.contains("000000000420");
    t24 {
        name: "switch_amount_in_output".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("amount 420 must appear as 12-digit field".into())
        },
    }
}

fn switch_currency_840_in_output() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![],
        amount_cents: 420,
        stan: 0,
    };
    let msg = f12(&req).unwrap();
    let ok = msg.raw.ends_with(b"840");
    t24 {
        name: "switch_currency_840_in_output".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("currency 840 USD must be last field".into())
        },
    }
}

fn switch_amount_out_of_range_fails() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![],
        amount_cents: 999_999_999,
        stan: 0,
    };
    let r = f12(&req);
    let ok = r.is_err();
    t24 {
        name: "switch_amount_out_of_range_fails".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("amount > 99999999 must fail".into())
        },
    }
}

fn switch_pan_too_long_fails() -> t24 {
    let start = Instant::now();
    let req = t2 {
        pan_encrypted: vec![0; 100],
        amount_cents: 420,
        stan: 0,
    };
    let r = f12(&req);
    let ok = r.is_err();
    t24 {
        name: "switch_pan_too_long_fails".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("PAN > 99 bytes must fail".into())
        },
    }
}
