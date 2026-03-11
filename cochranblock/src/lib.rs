// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! CochranBlock — Rust-only SaaS. Kova: thiserror, no unwrap in lib, /// doc = Why.
//! Flattened: core/* → src/ (folder_token_map).

pub mod error;
pub mod std_aliases;

#[cfg(feature = "admin")]
pub mod config;
#[cfg(feature = "admin")]
pub mod crypto;
#[cfg(feature = "admin")]
pub mod auth;
#[cfg(feature = "admin")]
pub mod dns;
#[cfg(feature = "admin")]
pub mod db;

#[cfg(feature = "tests")]
pub mod tests;
#[cfg(feature = "tests")]
pub mod screenshot;
pub mod web;
pub mod ux;

/// t0 = AppState. Why: Shared state for router; empty for static site.
#[derive(Clone)]
pub struct t0 {}
