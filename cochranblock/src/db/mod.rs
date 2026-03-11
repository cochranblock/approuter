// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! Kova: sled only. bincode + zstd for payloads. Zero-copy IVec where possible.

use std::path::Path;
use std::sync::Arc;

use bincode::config::standard;
use sled::IVec;

use crate::auth::session::t3;
use crate::error::t18;

/// t9 = sled Db. Why: Kova sled-only; Arc for shared ownership across handlers.
pub type t9 = Arc<sled::Db>;

const P_ADMIN: &[u8] = b"admin:";
const P_SESSION: &[u8] = b"session:";
const P_SETTINGS: &[u8] = b"settings:";
const P_DNS_NEXT: &[u8] = b"dns_log:next";
const P_DNS_ENTRY: &[u8] = b"dns_log:e:";

#[derive(serde::Serialize, serde::Deserialize)]
struct DnsLogEntry {
    ts: String,
    ip: String,
    status: String,
    msg: String,
}

fn enc<T: serde::Serialize>(v: &T) -> Result<Vec<u8>, t18> {
    let raw = bincode::serde::encode_to_vec(v, standard()).map_err(|e| t18::E6(e.to_string()))?;
    zstd::encode_all(raw.as_slice(), 3).map_err(|e| t18::E6(e.to_string()))
}

fn dec<T: serde::de::DeserializeOwned>(b: &[u8]) -> Result<T, t18> {
    let raw = zstd::decode_all(b).map_err(|e| t18::E6(e.to_string()))?;
    bincode::serde::decode_from_slice(raw.as_slice(), standard())
        .map(|(v, _)| v)
        .map_err(|e| t18::E6(e.to_string()))
}

/// f20 = init_db. Why: sled at dir; parent of .db path or path as dir.
pub fn f20(db_path: &str) -> Result<t9, t18> {
    let dir = if db_path.ends_with(".db") || db_path.ends_with(".sqlite") {
        Path::new(db_path).parent().unwrap_or(Path::new("."))
    } else {
        Path::new(db_path)
    };
    if let Some(p) = dir.parent() {
        std::fs::create_dir_all(p).map_err(|e| t18::E6(e.to_string()))?;
    }
    let db = sled::open(dir).map_err(|e| t18::E6(e.to_string()))?;
    Ok(Arc::new(db))
}

/// f21 = run_migrations. Why: sled has no schema; no-op for compatibility.
pub fn f21(_db: &t9) -> Result<(), t18> {
    Ok(())
}

/// f36 = admin_get. Why: lookup by username.
pub fn f36(db: &t9, s10: &str) -> Result<Option<(String, String)>, t18> {
    let k = [P_ADMIN, s10.as_bytes()].concat();
    Ok(db
        .get(&k)
        .map_err(|e| t18::E6(e.to_string()))?
        .map(|v| (s10.to_string(), String::from_utf8_lossy(&v).into_owned())))
}

/// f37 = admin_insert. Why: reject duplicate (INSERT semantics).
pub fn f37(db: &t9, s10: &str, s11: &str) -> Result<(), t18> {
    let k = [P_ADMIN, s10.as_bytes()].concat();
    if db.contains_key(&k).map_err(|e| t18::E6(e.to_string()))? {
        return Err(t18::E6("admin already exists".into()));
    }
    db.insert(k, s11.as_bytes()).map_err(|e| t18::E6(e.to_string()))?;
    Ok(())
}

/// f38 = admin_has_any. Why: first-time setup check.
pub fn f38(db: &t9) -> Result<bool, t18> {
    let mut it = db.scan_prefix(P_ADMIN);
    Ok(it.next().is_some())
}

/// f39 = session_insert.
pub fn f39(db: &t9, s: &t3) -> Result<(), t18> {
    let k = [P_SESSION, s.s37.as_bytes()].concat();
    let v = enc(s)?;
    db.insert(k, v).map_err(|e| t18::E6(e.to_string()))?;
    Ok(())
}

/// f40 = session_get.
pub fn f40(db: &t9, s37: &str) -> Result<Option<t3>, t18> {
    let k = [P_SESSION, s37.as_bytes()].concat();
    Ok(db
        .get(&k)
        .map_err(|e| t18::E6(e.to_string()))?
        .map(|v| dec::<t3>(&v))
        .transpose()?)
}

/// f41 = session_delete.
pub fn f41(db: &t9, s37: &str) -> Result<(), t18> {
    let k = [P_SESSION, s37.as_bytes()].concat();
    db.remove(k).map_err(|e| t18::E6(e.to_string()))?;
    Ok(())
}

/// f33 = session_cleanup_expired. Why: delete expired sessions.
pub fn f33(db: &t9) -> Result<(), t18> {
    let now = chrono::Utc::now().to_rfc3339();
    let mut to_del = Vec::new();
    for r in db.scan_prefix(P_SESSION) {
        let (k, v) = r.map_err(|e| t18::E6(e.to_string()))?;
        if let Ok(s) = dec::<t3>(&v) {
            if s.s14 < now {
                to_del.push(k);
            }
        }
    }
    for k in to_del {
        db.remove(k).map_err(|e| t18::E6(e.to_string()))?;
    }
    Ok(())
}

/// f42 = settings_get.
pub fn f42(db: &t9, key: &str) -> Result<Option<String>, t18> {
    let k = [P_SETTINGS, key.as_bytes()].concat();
    Ok(db
        .get(&k)
        .map_err(|e| t18::E6(e.to_string()))?
        .map(|v| String::from_utf8_lossy(&v).into_owned()))
}

/// f43 = settings_upsert.
pub fn f43(db: &t9, key: &str, value: &str) -> Result<(), t18> {
    let k = [P_SETTINGS, key.as_bytes()].concat();
    db.insert(k, value.as_bytes()).map_err(|e| t18::E6(e.to_string()))?;
    Ok(())
}

/// f44 = dns_log_append.
pub fn f44(db: &t9, ip: &str, status: &str, msg: &str) -> Result<(), t18> {
    let id: u64 = db
        .update_and_fetch(P_DNS_NEXT, |old| {
            let n: u64 = old
                .and_then(|v| std::str::from_utf8(&v).ok())
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
            Some(IVec::from((n + 1).to_string().into_bytes()))
        })
        .map_err(|e| t18::E6(e.to_string()))?
        .and_then(|v| std::str::from_utf8(&v).ok()?.parse().ok())
        .unwrap_or(1);
    let entry = DnsLogEntry {
        ts: chrono::Utc::now().to_rfc3339(),
        ip: ip.to_string(),
        status: status.to_string(),
        msg: msg.to_string(),
    };
    let k = [P_DNS_ENTRY, format!("{:020}", id).as_bytes()].concat();
    let v = enc(&entry)?;
    db.insert(k, v).map_err(|e| t18::E6(e.to_string()))?;
    Ok(())
}

/// f45 = dns_log_recent. Why: latest N entries, id DESC.
pub fn f45(db: &t9, limit: i32) -> Result<Vec<(String, String, String, String)>, t18> {
    let mut out = Vec::new();
    for r in db.scan_prefix(P_DNS_ENTRY).rev().take(limit as usize) {
        let (_, v) = r.map_err(|e| t18::E6(e.to_string()))?;
        let e: DnsLogEntry = dec(&v)?;
        out.push((e.ts, e.ip, e.status, e.msg));
    }
    Ok(out)
}
