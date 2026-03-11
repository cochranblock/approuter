// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f50 = integration: real DB, real ledger f14/f15/f16

use std::time::Instant;

use crate::ledger::t4;
use sqlx::postgres::PgPoolOptions;
use uuid::Uuid;

use crate::tests::t24;

pub async fn f50() -> Vec<t24> {
    let mut out = Vec::new();

    if let Ok(url) = std::env::var("DATABASE_URL") {
        out.push(ledger_add_device(&url).await);
        out.push(ledger_provision_entitlement(&url).await);
        out.push(ledger_credit_bucks(&url).await);
        out.push(ledger_insufficient_rejected(&url).await);
        out.push(ledger_device_exists_rejected(&url).await);
    }

    out
}

async fn ledger_add_device(url: &str) -> t24 {
    let start = Instant::now();
    let pool = match PgPoolOptions::new().max_connections(2).connect(url).await {
        Ok(p) => p,
        Err(e) => {
            return t24 {
                name: "ledger_add_device".into(),
                passed: false,
                duration_ms: start.elapsed().as_millis() as u64,
                message: Some(format!("pool: {}", e)),
            };
        }
    };
    if let Err(e) = sqlx::migrate!("../migrations").run(&pool).await {
        return t24 {
            name: "ledger_add_device".into(),
            passed: false,
            duration_ms: start.elapsed().as_millis() as u64,
            message: Some(format!("migrate: {}", e)),
        };
    }
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 500)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .ok();
    let ledger = t4::new(pool.clone());
    let r = ledger.f14(uid, "fp-test-1").await;
    let Ok(()) = r else {
        return t24 {
            name: "ledger_add_device".into(),
            passed: false,
            duration_ms: start.elapsed().as_millis() as u64,
            message: Some(format!("f14: {:?}", r.err())),
        };
    };
    let row: (i64,) = sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1")
        .bind(uid)
        .fetch_one(&pool)
        .await
        .unwrap();
    let count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM devices WHERE user_id = $1 AND hardware_fingerprint = $2",
    )
    .bind(uid)
    .bind("fp-test-1")
    .fetch_one(&pool)
    .await
    .unwrap();
    sqlx::query("DELETE FROM devices WHERE user_id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = row.0 == 80 && count.0 == 1;
    t24 {
        name: "ledger_add_device".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("balance 80, device count 1".into())
        },
    }
}

async fn ledger_provision_entitlement(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 100)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    let r = ledger.f15(uid, "game-1").await;
    let Ok(()) = r else {
        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(uid)
            .execute(&pool)
            .await
            .ok();
        return t24 {
            name: "ledger_provision_entitlement".into(),
            passed: false,
            duration_ms: start.elapsed().as_millis() as u64,
            message: Some(format!("f15: {:?}", r.err())),
        };
    };
    let row: (i64,) = sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1")
        .bind(uid)
        .fetch_one(&pool)
        .await
        .unwrap();
    let has: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM entitlements WHERE user_id = $1 AND game_id = $2")
            .bind(uid)
            .bind("game-1")
            .fetch_one(&pool)
            .await
            .unwrap();
    sqlx::query("DELETE FROM entitlements WHERE user_id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = row.0 == 58 && has.0 == 1;
    t24 {
        name: "ledger_provision_entitlement".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("balance 58, entitlement exists".into())
        },
    }
}

async fn ledger_credit_bucks(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 0)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    ledger.f16(uid, 420).await.unwrap();
    let row: (i64,) = sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1")
        .bind(uid)
        .fetch_one(&pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = row.0 == 420;
    t24 {
        name: "ledger_credit_bucks".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("balance 420 after credit".into())
        },
    }
}

async fn ledger_insufficient_rejected(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 10)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    let r = ledger.f14(uid, "fp-x").await;
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = r.is_err();
    t24 {
        name: "ledger_insufficient_rejected".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("f14 with 10 bucks must fail".into())
        },
    }
}

async fn ledger_device_exists_rejected(url: &str) -> t24 {
    let start = Instant::now();
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(url)
        .await
        .unwrap();
    let uid = Uuid::new_v4();
    sqlx::query("INSERT INTO users (id, email, rogue_bucks_balance) VALUES ($1, $2, 1000)")
        .bind(uid)
        .bind(format!("test-{}@x.com", uid))
        .execute(&pool)
        .await
        .unwrap();
    sqlx::query(
        "INSERT INTO devices (id, user_id, hardware_fingerprint, is_authorized) VALUES (gen_random_uuid(), $1, $2, true)",
    )
        .bind(uid)
        .bind("fp-dupe")
        .execute(&pool)
        .await
        .unwrap();
    let ledger = t4::new(pool.clone());
    let r = ledger.f14(uid, "fp-dupe").await;
    sqlx::query("DELETE FROM devices WHERE user_id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    sqlx::query("DELETE FROM users WHERE id = $1")
        .bind(uid)
        .execute(&pool)
        .await
        .ok();
    let ok = r.is_err();
    t24 {
        name: "ledger_device_exists_rejected".into(),
        passed: ok,
        duration_ms: start.elapsed().as_millis() as u64,
        message: if ok {
            None
        } else {
            Some("f14 duplicate device must fail".into())
        },
    }
}
