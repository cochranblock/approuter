// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

//! t18 = AppError. Kova: thiserror for all custom errors. No unwrap/expect in lib.

use thiserror::Error;

/// t18 = AppError. Central error type for CochranBlock. Why: single source for status codes and display.
#[derive(Debug, Error)]
pub enum t18 {
    #[error("Unauthorized")]
    E0,
    #[error("Invalid credentials")]
    E1,
    #[error("Session expired")]
    E2,
    #[error("External service error: {0}")]
    E3(String),
    #[error("IP lookup failed: {0}")]
    E4(String),
    #[error("Encryption error: {0}")]
    E5(String),
    #[error("Database error: {0}")]
    E6(String),
    #[error("Configuration error: {0}")]
    E7(String),
    #[error("Setup required")]
    E8,
    #[error("Too many requests")]
    E9,
    #[error("Internal error: {0}")]
    E10(String),
}

impl t18 {
    /// Why: HTTP handlers need status codes without matching on variants.
    pub fn status_code(&self) -> u16 {
        match self {
            t18::E0 | t18::E1 | t18::E2 => 401,
            t18::E3(_) => 502,
            t18::E4(_) => 503,
            t18::E5(_) | t18::E6(_) | t18::E7(_) | t18::E10(_) => 500,
            t18::E8 => 307,
            t18::E9 => 429,
        }
    }

    /// Why: Public API returns safe messages; internal details stay in Debug.
    pub fn public_message(&self) -> &'static str {
        match self {
            t18::E0 => "Unauthorized",
            t18::E1 => "Invalid credentials",
            t18::E2 => "Session expired",
            t18::E3(_) => "External service error",
            t18::E4(_) => "IP lookup failed",
            t18::E5(_) => "Encryption error",
            t18::E6(_) => "Database error",
            t18::E7(_) => "Configuration error",
            t18::E8 => "Setup required",
            t18::E9 => "Too many requests",
            t18::E10(_) => "Internal error",
        }
    }
}

