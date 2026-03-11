// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use std::time::Instant;
use tempfile::TempDir;

use super::t24;
use super::assert_ok;
use crate::db;
use crate::auth::session;
use crate::crypto::token;

async fn run(p0: &str, p1: impl std::future::Future<Output = Result<(), String>>) -> t24 {
    let v0 = Instant::now();
    match p1.await {
        Ok(()) => t24 { s30: p0.into(), s31: true, s32: v0.elapsed().as_millis() as u64, s33: None },
        Err(msg) => t24 { s30: p0.into(), s31: false, s32: v0.elapsed().as_millis() as u64, s33: Some(msg) },
    }
}

pub async fn f50() -> Vec<t24> {
    let mut v0 = Vec::new();
    let v1 = TempDir::new().unwrap();
    let v2 = v1.path().to_str().unwrap().to_string();

    v0.push(run("db_init", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        assert_ok(db::f21(&v3).is_ok(), "migrations must apply")?;
        Ok(())
    }).await);
    v0.push(run("admin_crud", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        db::f37(&v3, "admin1", "hash1").map_err(|e| e.to_string())?;
        let v4 = db::f36(&v3, "admin1").map_err(|e| e.to_string())?;
        assert_ok(v4.is_some(), "admin must be retrievable")?;
        assert_ok(db::f38(&v3).map_err(|e| e.to_string())?, "admin count must be > 0")?;
        Ok(())
    }).await);
    v0.push(run("admin_duplicate_rejection", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        db::f37(&v3, "dup", "h1").map_err(|e| e.to_string())?;
        assert_ok(db::f37(&v3, "dup", "h2").is_err(), "duplicate admin must be rejected")?;
        Ok(())
    }).await);
    v0.push(run("session_crud", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        let v4 = session::t3::f18("u");
        db::f39(&v3, &v4).map_err(|e| e.to_string())?;
        let v5 = db::f40(&v3, &v4.s37).map_err(|e| e.to_string())?;
        assert_ok(v5.is_some(), "session must be stored")?;
        db::f41(&v3, &v4.s37).map_err(|e| e.to_string())?;
        let v6 = db::f40(&v3, &v4.s37).map_err(|e| e.to_string())?;
        assert_ok(v6.is_none(), "deleted session must be gone")?;
        Ok(())
    }).await);
    v0.push(run("encrypted_settings", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        let v4: [u8; 32] = crate::crypto::key::f31("key", "enc").map_err(|e| e.to_string())?;
        let v5 = token::f14("secret", &v4).map_err(|e| e.to_string())?;
        db::f43(&v3, "tok", &v5).map_err(|e| e.to_string())?;
        let v6 = db::f42(&v3, "tok").map_err(|e| e.to_string())?.ok_or("setting not found")?;
        assert_ok(token::f15(&v6, &v4).map_err(|e| e.to_string())? == "secret", "decrypt must match")?;
        Ok(())
    }).await);
    v0.push(run("encrypted_settings_wrong_key", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        let v4: [u8; 32] = crate::crypto::key::f31("key1", "enc").map_err(|e| e.to_string())?;
        let v5: [u8; 32] = crate::crypto::key::f31("key2", "enc").map_err(|e| e.to_string())?;
        let v6 = token::f14("secret", &v4).map_err(|e| e.to_string())?;
        db::f43(&v3, "tok", &v6).map_err(|e| e.to_string())?;
        let v7 = db::f42(&v3, "tok").map_err(|e| e.to_string())?.ok_or("setting not found")?;
        assert_ok(token::f15(&v7, &v5).is_err(), "wrong key must fail decrypt")?;
        Ok(())
    }).await);
    v0.push(run("plaintext_upsert", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        db::f43(&v3, "k", "v1").map_err(|e| e.to_string())?;
        db::f43(&v3, "k", "v2").map_err(|e| e.to_string())?;
        let v4 = db::f42(&v3, "k").map_err(|e| e.to_string())?.ok_or("setting not found")?;
        assert_ok(v4 == "v2", "upsert must overwrite")?;
        Ok(())
    }).await);
    v0.push(run("dns_log", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        db::f44(&v3, "1.2.3.4", "ok", "msg").map_err(|e| e.to_string())?;
        let v4 = db::f45(&v3, 5).map_err(|e| e.to_string())?;
        assert_ok(v4.len() == 1, format!("expected 1 log entry got {}", v4.len()))?;
        assert_ok(v4[0].1 == "1.2.3.4", "log must store IP")?;
        Ok(())
    }).await);
    v0.push(run("session_cleanup", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        let v4 = session::t3 { s37: "expired-id".into(), s10: "u".into(), s13: "2020-01-01T00:00:00Z".into(), s14: "2020-01-01T00:00:01Z".into() };
        db::f39(&v3, &v4).map_err(|e| e.to_string())?;
        db::f33(&v3).map_err(|e| e.to_string())?;
        let v5 = db::f40(&v3, "expired-id").map_err(|e| e.to_string())?;
        assert_ok(v5.is_none(), "expired session must be cleaned up")?;
        Ok(())
    }).await);
    v0.push(run("db_nested_path", async {
        let v3_path = v1.path().join("nested/sub").to_str().unwrap().to_string();
        let v3 = db::f20(&v3_path).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        db::f43(&v3, "nested_key", "nested_val").map_err(|e| e.to_string())?;
        let v4 = db::f42(&v3, "nested_key").map_err(|e| e.to_string())?.ok_or("not found")?;
        assert_ok(v4 == "nested_val", "nested db must persist")?;
        Ok(())
    }).await);
    v0.push(run("dns_log_limit", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        db::f44(&v3, "1.1.1.1", "a", "m1").map_err(|e| e.to_string())?;
        db::f44(&v3, "2.2.2.2", "b", "m2").map_err(|e| e.to_string())?;
        db::f44(&v3, "3.3.3.3", "c", "m3").map_err(|e| e.to_string())?;
        let v4 = db::f45(&v3, 2).map_err(|e| e.to_string())?;
        assert_ok(v4.len() == 2, format!("limit 2 must return 2, got {}", v4.len()))?;
        Ok(())
    }).await);
    v0.push(run("settings_missing_key", async {
        let v3 = db::f20(&v2).map_err(|e| e.to_string())?;
        db::f21(&v3).map_err(|e| e.to_string())?;
        let v4 = db::f42(&v3, "nonexistent_key_xyz").map_err(|e| e.to_string())?;
        assert_ok(v4.is_none(), "missing key must return None")?;
        Ok(())
    }).await);
    v0
}
