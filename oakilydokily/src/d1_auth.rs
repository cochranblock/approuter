// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Sharded D1 auth storage. f78=user_get f79=user_create. Replaces on-premise users table.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use chrono::Utc;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const D1_API_BASE: &str = "https://api.cloudflare.com/client/v4";

/// t78 = D1AuthClient. s78=account_id s79=token s80=shard_ids
#[derive(Clone)]
pub struct t78 {
    s78: String,
    s79: String,
    s80: Vec<String>,
    client: Client,
}

impl t78 {
    pub fn shard_count(&self) -> usize {
        self.s80.len()
    }
}

/// f78 = user_get. Lookup by email. Returns (name, password_hash).
pub async fn f78(c: &t78, email: &str) -> Result<Option<(String, String)>, D1Error> {
    let db_id = f81(c, email);
    let sql = "SELECT name, password_hash FROM users WHERE email = ?1";
    let body = D1Query {
        sql: sql.to_string(),
        params: vec![serde_json::Value::String(email.to_string())],
    };
    let res = f82(c, db_id, &body).await?;
    let results = res.results();
    let row = match results.first() {
        Some(r) => r,
        None => return Ok(None),
    };
    let name = row.get("name").and_then(|v| v.as_str()).ok_or(D1Error::Parse)?;
    let pwhash = row
        .get("password_hash")
        .and_then(|v| v.as_str())
        .ok_or(D1Error::Parse)?;
    Ok(Some((name.to_string(), pwhash.to_string())))
}

/// f79 = user_create. Insert native user. Returns Err if email exists.
pub async fn f79(
    c: &t78,
    email: &str,
    password_hash: &str,
    name: &str,
) -> Result<(), D1Error> {
    let db_id = f81(c, email);
    let created_at = Utc::now().to_rfc3339();
    let sql = "INSERT INTO users (email, password_hash, name, created_at) VALUES (?1, ?2, ?3, ?4)";
    let body = D1Query {
        sql: sql.to_string(),
        params: vec![
            serde_json::Value::String(email.to_string()),
            serde_json::Value::String(password_hash.to_string()),
            serde_json::Value::String(name.to_string()),
            serde_json::Value::String(created_at),
        ],
    };
    f82(c, db_id, &body).await?;
    Ok(())
}

/// f81 = shard_for_email. Picks shard by hash(email) % N.
fn f81<'a>(c: &'a t78, email: &str) -> &'a str {
    let normalized = email.trim().to_lowercase();
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    let hash = hasher.finalize();
    let idx = u32::from_be_bytes([hash[0], hash[1], hash[2], hash[3]]) as usize;
    let n = c.s80.len().max(1);
    &c.s80[idx % n]
}

/// f82 = d1_query. POST to D1 REST API.
async fn f82(c: &t78, database_id: &str, body: &D1Query) -> Result<D1Response, D1Error> {
    let url = format!(
        "{}/accounts/{}/d1/database/{}/query",
        D1_API_BASE, c.s78, database_id
    );
    let res = c
        .client
        .post(&url)
        .header("Authorization", format!("Bearer {}", c.s79))
        .json(body)
        .send()
        .await
        .map_err(|e| D1Error::Request(e.to_string()))?;
    let status = res.status();
    let text = res.text().await.map_err(|e| D1Error::Request(e.to_string()))?;
    let parsed: D1Response =
        serde_json::from_str(&text).map_err(|_| D1Error::Parse)?;
    if !parsed.success {
        return Err(D1Error::Api(parsed.errors.join(", ")));
    }
    if !status.is_success() {
        return Err(D1Error::Request(format!("HTTP {}: {}", status, text)));
    }
    Ok(parsed)
}

/// f80_from_env = d1_client_from_env. Build t78 from env. Returns None if OD_AUTH_D1 not set.
pub fn f80_from_env() -> Option<t78> {
    if std::env::var("OD_AUTH_D1").ok().as_deref() != Some("1") {
        return None;
    }
    let s78 = std::env::var("CLOUDFLARE_ACCOUNT_ID").ok()?;
    let s79 = std::env::var("CLOUDFLARE_API_TOKEN").ok()?;
    let ids = std::env::var("OD_D1_SHARD_IDS").ok()?;
    let s80: Vec<String> = ids
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    if s78.is_empty() || s79.is_empty() || s80.is_empty() {
        return None;
    }
    Some(t78 {
        s78,
        s79,
        s80,
        client: Client::new(),
    })
}

#[derive(Serialize)]
struct D1Query {
    sql: String,
    params: Vec<serde_json::Value>,
}

#[derive(Deserialize)]
struct D1Response {
    success: bool,
    #[serde(default)]
    result: serde_json::Value,
    #[serde(default)]
    errors: Vec<String>,
}

impl D1Response {
    /// Extract results. REST API: result is object {results:[...]} or array [{results:[...]}].
    fn results(&self) -> Vec<serde_json::Map<String, serde_json::Value>> {
        let target = if let Some(arr) = self.result.as_array() {
            arr.first()
        } else if self.result.is_object() {
            Some(&self.result)
        } else {
            None
        };
        let target = match target {
            Some(t) => t,
            None => return vec![],
        };
        if let Some(arr) = target.get("results").and_then(|v| v.as_array()) {
            arr.iter()
                .filter_map(|v| v.as_object().cloned())
                .collect()
        } else if let Some(obj) = target.as_object() {
            vec![obj.clone()]
        } else {
            vec![]
        }
    }
}

#[derive(Debug)]
pub enum D1Error {
    Request(String),
    Api(String),
    Parse,
}

impl std::fmt::Display for D1Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            D1Error::Request(s) => write!(f, "D1 request: {}", s),
            D1Error::Api(s) => write!(f, "D1 API: {}", s),
            D1Error::Parse => write!(f, "D1 parse error"),
        }
    }
}

impl std::error::Error for D1Error {}
