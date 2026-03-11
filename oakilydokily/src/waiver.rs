// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use chrono::Utc;
use sha2::{Digest, Sha256};
use sqlx::sqlite::SqlitePool;
use std::path::Path;
use uuid::Uuid;

pub const TERMS_VERSION: &str = "2026-01";
const TERMS: &str = include_str!("../content/waiver_terms.txt");

/// f112 = terms_hash. SHA256 of waiver terms for versioning.
pub fn terms_hash() -> String {
    format!("{:x}", Sha256::digest(TERMS.as_bytes()))
}

/// f113 = init_pool. Create data dir, connect SQLite, migrate.
pub async fn init_pool(
    data_dir: &Path,
) -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    std::fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join("waivers.sqlite");
    let url = format!("sqlite:{}?mode=rwc", db_path.display());
    let pool = SqlitePool::connect(&url).await?;
    sqlx::query("PRAGMA journal_mode=WAL")
        .execute(&pool)
        .await?;
    migrate(&pool).await?;
    Ok(pool)
}

/// f113_memory = init_pool_memory. In-memory pool for tests.
pub async fn init_pool_memory() -> Result<SqlitePool, Box<dyn std::error::Error + Send + Sync>> {
    let pool = SqlitePool::connect("sqlite::memory:").await?;
    migrate(&pool).await?;
    Ok(pool)
}

async fn migrate(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS waivers (
            id TEXT PRIMARY KEY,
            full_name TEXT NOT NULL,
            email TEXT NOT NULL,
            signed_at TEXT NOT NULL,
            ip_address TEXT,
            user_agent TEXT,
            terms_version TEXT NOT NULL,
            terms_hash TEXT NOT NULL,
            consent_electronic INTEGER NOT NULL,
            signature_text TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    let _ = sqlx::query("ALTER TABLE waivers DROP COLUMN signing_on_behalf_of")
        .execute(pool)
        .await;
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            email TEXT PRIMARY KEY,
            password_hash TEXT NOT NULL,
            name TEXT NOT NULL,
            created_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// f114 = insert. Insert waiver record. Returns ref_id.
pub async fn insert(
    pool: &SqlitePool,
    full_name: &str,
    email: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
    signature_text: &str,
) -> Result<String, sqlx::Error> {
    let id = Uuid::new_v4().to_string();
    let signed_at = Utc::now().to_rfc3339();
    let hash = terms_hash();
    sqlx::query(
        r#"
        INSERT INTO waivers (id, full_name, email, signed_at, ip_address, user_agent, terms_version, terms_hash, consent_electronic, signature_text)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, 1, ?)
        "#,
    )
    .bind(&id)
    .bind(full_name)
    .bind(email)
    .bind(&signed_at)
    .bind(ip_address)
    .bind(user_agent)
    .bind(TERMS_VERSION)
    .bind(&hash)
    .bind(signature_text)
    .execute(pool)
    .await?;
    Ok(id)
}

/// f115 = terms_text. Raw waiver terms for display.
pub fn terms_text() -> &'static str {
    TERMS
}

/// f116 = user_create. Insert native user. Returns Err if email exists.
pub async fn user_create(
    pool: &SqlitePool,
    email: &str,
    password_hash: &str,
    name: &str,
) -> Result<(), sqlx::Error> {
    let created_at = Utc::now().to_rfc3339();
    sqlx::query(
        "INSERT INTO users (email, password_hash, name, created_at) VALUES (?, ?, ?, ?)",
    )
    .bind(email)
    .bind(password_hash)
    .bind(name)
    .bind(&created_at)
    .execute(pool)
    .await?;
    Ok(())
}

/// f117 = user_get. Lookup by email. Returns (name, password_hash).
pub async fn user_get(
    pool: &SqlitePool,
    email: &str,
) -> Result<Option<(String, String)>, sqlx::Error> {
    let row = sqlx::query_as::<_, (String, String)>(
        "SELECT name, password_hash FROM users WHERE email = ?",
    )
    .bind(email)
    .fetch_optional(pool)
    .await?;
    Ok(row)
}

/// f77 = validate_waiver_input. Why: Proptest (B4); single source of truth for length limits.
pub fn f77(full_name: &str, email: &str) -> Result<(), &'static str> {
    let n = full_name.trim();
    let e = email.trim();
    if n.is_empty() {
        return Err("name empty");
    }
    if e.is_empty() {
        return Err("email empty");
    }
    if n.len() > 200 {
        return Err("name too long");
    }
    if e.len() > 254 {
        return Err("email too long");
    }
    Ok(())
}
