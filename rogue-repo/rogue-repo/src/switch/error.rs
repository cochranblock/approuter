// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use thiserror::Error;

#[derive(Error, Debug)]
pub enum E4 {
    #[error("pack: {0}")]
    Pack(String),
    #[error("unpack: {0}")]
    Unpack(String),
    #[error("connection: {0}")]
    Connection(String),
    #[error("invalid amount: {0}")]
    Amount(String),
}
