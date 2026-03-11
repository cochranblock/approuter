// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub mod d1_auth;
#[cfg(feature = "tests")]
pub mod mock_google;
#[cfg(feature = "tests")]
pub mod screenshot;
#[cfg(feature = "tests")]
pub mod tests;
pub mod waiver;
pub mod web;

use sqlx::sqlite::SqlitePool;

/// t0 = AppState. s0=waiver pool s1=optional D1 auth (when OD_AUTH_D1=1)
#[derive(Clone)]
pub struct t0 {
    pub s0: SqlitePool,
    pub s1: Option<d1_auth::t78>,
}
pub use t0 as AppState;
