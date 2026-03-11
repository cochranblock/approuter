// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum E5 {
    #[error("insufficient bucks: {0}")]
    Insufficient(i64),
    #[error("device already authorized")]
    DeviceExists,
    #[error("db: {0}")]
    Db(String),
    #[error("not found: {0}")]
    NotFound(String),
}
