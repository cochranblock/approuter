// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use std::time::Instant;

use super::t24;
use super::assert_ok;
use crate::crypto::{key, token};
use crate::auth::{password, session};
use crate::config::t1;
use crate::error::t18;
use crate::dns::cloudflare;

async fn run(p0: &str, p1: impl std::future::Future<Output = Result<(), String>>) -> t24 {
    let v0 = Instant::now();
    match p1.await {
        Ok(()) => t24 { s30: p0.into(), s31: true, s32: v0.elapsed().as_millis() as u64, s33: None },
        Err(msg) => t24 { s30: p0.into(), s31: false, s32: v0.elapsed().as_millis() as u64, s33: Some(msg) },
    }
}

pub async fn f49() -> Vec<t24> {
    let mut v0 = Vec::new();
    v0.push(run("crypto_roundtrip", async {
        let v1: [u8; 32] = key::f31("master", "ctx").map_err(|e| e.to_string())?;
        let v2 = token::f14("secret", &v1).map_err(|e| e.to_string())?;
        let v3 = token::f15(&v2, &v1).map_err(|e| e.to_string())?;
        assert_ok(v3 == "secret", "decrypt did not match original")?;
        Ok(())
    }).await);
    v0.push(run("crypto_tamper", async {
        let v1: [u8; 32] = key::f31("m", "c").map_err(|e| e.to_string())?;
        let mut v2 = token::f14("x", &v1).map_err(|e| e.to_string())?.into_bytes();
        v2[20] ^= 0xff;
        assert_ok(token::f15(&String::from_utf8_lossy(&v2), &v1).is_err(), "tampered token should fail decrypt")?;
        Ok(())
    }).await);
    v0.push(run("crypto_wrong_key", async {
        let v1: [u8; 32] = key::f31("a", "c").map_err(|e| e.to_string())?;
        let v2: [u8; 32] = key::f31("b", "c").map_err(|e| e.to_string())?;
        let v3 = token::f14("x", &v1).map_err(|e| e.to_string())?;
        assert_ok(token::f15(&v3, &v2).is_err(), "wrong key should fail decrypt")?;
        Ok(())
    }).await);
    v0.push(run("crypto_determinism", async {
        let v1: [u8; 32] = key::f31("m", "c").map_err(|e| e.to_string())?;
        let v2 = token::f14("x", &v1).map_err(|e| e.to_string())?;
        let v3 = token::f14("x", &v1).map_err(|e| e.to_string())?;
        assert_ok(v2 != v3, "AEAD must produce different ciphertext each time")?;
        Ok(())
    }).await);
    v0.push(run("crypto_not_plaintext", async {
        let v1: [u8; 32] = key::f31("m", "c").map_err(|e| e.to_string())?;
        let v2 = token::f14("my_secret", &v1).map_err(|e| e.to_string())?;
        assert_ok(!v2.contains("my_secret"), "ciphertext must not leak plaintext")?;
        Ok(())
    }).await);
    v0.push(run("key_derivation_isolation", async {
        let v1 = key::f31("m", "ctx1").map_err(|e| e.to_string())?;
        let v2 = key::f31("m", "ctx2").map_err(|e| e.to_string())?;
        assert_ok(v1 != v2, "different context must yield different keys")?;
        assert_ok(key::f31("m", "c").map_err(|e| e.to_string())? != key::f32("m").map_err(|e| e.to_string())?, "f31 vs f32 must differ")?;
        Ok(())
    }).await);
    v0.push(run("password_hash_verify", async {
        let v1 = password::f16("test123").map_err(|e| e.to_string())?;
        assert_ok(password::f17("test123", &v1).map_err(|e| e.to_string())?, "correct password must verify")?;
        Ok(())
    }).await);
    v0.push(run("password_unique_salts", async {
        let v1 = password::f16("x").map_err(|e| e.to_string())?;
        let v2 = password::f16("x").map_err(|e| e.to_string())?;
        assert_ok(v1 != v2, "each hash must use unique salt")?;
        Ok(())
    }).await);
    v0.push(run("password_wrong_returns_false", async {
        let v1 = password::f16("right").map_err(|e| e.to_string())?;
        assert_ok(!password::f17("wrong", &v1).map_err(|e| e.to_string())?, "wrong password must not verify")?;
        Ok(())
    }).await);
    v0.push(run("session_creation", async {
        let v1 = session::t3::f18("user");
        assert_ok(v1.s10 == "user", "session must store username")?;
        assert_ok(v1.f19(), "new session must be valid")?;
        Ok(())
    }).await);
    v0.push(run("session_expired_invalid", async {
        let v1 = session::t3 { s37: "x".into(), s10: "u".into(), s13: "2020-01-01T00:00:00Z".into(), s14: "2020-01-01T00:00:01Z".into() };
        assert_ok(!v1.f19(), "expired session must be invalid")?;
        Ok(())
    }).await);
    v0.push(run("config_defaults", async {
        let v1 = t1::default();
        assert_ok(v1.s16 == 3000, "default port must be 3000")?;
        assert_ok(v1.s18 == "data", "default db dir must be data")?;
        Ok(())
    }).await);
    v0.push(run("config_db_path", async {
        let v1 = t1::default();
        assert_ok(v1.db_path() == "data/portfolio.db", "db_path must be data/portfolio.db")?;
        Ok(())
    }).await);
    v0.push(run("config_with_values", async {
        let v1 = t1::with_values(8080, "0.0.0.0", "/var/data", "key", "https://cf", "https://ip", 60);
        assert_ok(v1.s16 == 8080, "port must match")?;
        assert_ok(v1.s17 == "0.0.0.0", "bind must match")?;
        assert_ok(v1.db_path() == "/var/data/portfolio.db", "db_path must reflect data dir")?;
        Ok(())
    }).await);
    v0.push(run("error_no_internal_leaks", async {
        let v1 = t18::E5("secret db path".into()).public_message();
        assert_ok(!v1.contains("secret"), "public message must not leak internal details")?;
        Ok(())
    }).await);
    v0.push(run("dns_url_build", async {
        let v1 = cloudflare::f47("https://api.cf.com", "zone1", "rec1");
        assert_ok(v1.contains("zone1"), "URL must contain zone")?;
        assert_ok(v1.contains("rec1"), "URL must contain record")?;
        Ok(())
    }).await);
    v0.push(run("dns_ip_changed", async {
        assert_ok(cloudflare::f48("1.2.3.4", "1.2.3.5"), "different IPs must be changed")?;
        assert_ok(!cloudflare::f48("1.2.3.4", "1.2.3.4"), "same IP must not be changed")?;
        Ok(())
    }).await);
    v0.push(run("dns_empty_token", async {
        assert_ok(cloudflare::f46("").is_err(), "empty token must error")?;
        Ok(())
    }).await);
    v0.push(run("key_f32_deterministic", async {
        let v1 = key::f32("seed").map_err(|e| e.to_string())?;
        let v2 = key::f32("seed").map_err(|e| e.to_string())?;
        assert_ok(v1 == v2, "f32 must be deterministic for same input")?;
        assert_ok(v1.len() == 32, "f32 must produce 32 bytes")?;
        Ok(())
    }).await);
    v0.push(run("session_id_format", async {
        let v1 = session::t3::f18("u");
        assert_ok(v1.s37.len() == 36, "session id must be UUID length")?;
        assert_ok(v1.s37.contains('-'), "session id must be UUID format")?;
        Ok(())
    }).await);
    v0
}
