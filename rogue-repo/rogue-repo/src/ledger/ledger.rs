// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! t4 = Ledger, f14 = add_device, f15 = provision_entitlement, f16 = credit_bucks

use sqlx::PgPool;
use uuid::Uuid;

use super::E5;

pub struct t4 {
    pool: PgPool,
}

impl t4 {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// f14 = add_device: deduct 420 bucks, insert fingerprint (non-destructive)
    pub async fn f14(&self, p6: Uuid, p7: &str) -> Result<(), E5> {
        const FEE: i64 = 420;
        let mut tx = self.pool.begin().await.map_err(|e| E5::Db(e.to_string()))?;

        let row: (i64,) =
            sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1 FOR UPDATE")
                .bind(p6)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| E5::Db(e.to_string()))?
                .ok_or_else(|| E5::NotFound("user".into()))?;

        if row.0 < FEE {
            return Err(E5::Insufficient(row.0));
        }

        let exists: Option<(i32,)> = sqlx::query_as(
            "SELECT 1 FROM devices WHERE user_id = $1 AND hardware_fingerprint = $2 AND is_authorized",
        )
        .bind(p6)
        .bind(p7)
        .fetch_optional(&mut *tx)
        .await
        .map_err(|e| E5::Db(e.to_string()))?;

        if exists.is_some() {
            return Err(E5::DeviceExists);
        }

        sqlx::query(
            "UPDATE users SET rogue_bucks_balance = rogue_bucks_balance - $1 WHERE id = $2",
        )
        .bind(FEE)
        .bind(p6)
        .execute(&mut *tx)
        .await
        .map_err(|e| E5::Db(e.to_string()))?;

        sqlx::query(
            "INSERT INTO devices (id, user_id, hardware_fingerprint, is_authorized, added_at)
             VALUES ($1, $2, $3, true, NOW())",
        )
        .bind(Uuid::new_v4())
        .bind(p6)
        .bind(p7)
        .execute(&mut *tx)
        .await
        .map_err(|e| E5::Db(e.to_string()))?;

        tx.commit().await.map_err(|e| E5::Db(e.to_string()))?;
        Ok(())
    }

    /// f15 = provision_entitlement: deduct 42 bucks, insert entitlement
    pub async fn f15(&self, p6: Uuid, p8: &str) -> Result<(), E5> {
        const COST: i64 = 42;
        let mut tx = self.pool.begin().await.map_err(|e| E5::Db(e.to_string()))?;

        let row: (i64,) =
            sqlx::query_as("SELECT rogue_bucks_balance FROM users WHERE id = $1 FOR UPDATE")
                .bind(p6)
                .fetch_optional(&mut *tx)
                .await
                .map_err(|e| E5::Db(e.to_string()))?
                .ok_or_else(|| E5::NotFound("user".into()))?;

        if row.0 < COST {
            return Err(E5::Insufficient(row.0));
        }

        sqlx::query(
            "UPDATE users SET rogue_bucks_balance = rogue_bucks_balance - $1 WHERE id = $2",
        )
        .bind(COST)
        .bind(p6)
        .execute(&mut *tx)
        .await
        .map_err(|e| E5::Db(e.to_string()))?;

        sqlx::query("INSERT INTO entitlements (id, user_id, game_id) VALUES ($1, $2, $3)")
            .bind(Uuid::new_v4())
            .bind(p6)
            .bind(p8)
            .execute(&mut *tx)
            .await
            .map_err(|e| E5::Db(e.to_string()))?;

        tx.commit().await.map_err(|e| E5::Db(e.to_string()))?;
        Ok(())
    }

    /// f16 = credit_bucks: add 420 for entry buy-in
    pub async fn f16(&self, p6: Uuid, amount: i64) -> Result<(), E5> {
        sqlx::query(
            "UPDATE users SET rogue_bucks_balance = rogue_bucks_balance + $1 WHERE id = $2",
        )
        .bind(amount)
        .bind(p6)
        .execute(&self.pool)
        .await
        .map_err(|e| E5::Db(e.to_string()))?;
        Ok(())
    }
}
