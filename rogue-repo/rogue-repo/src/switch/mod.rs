// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! switch: ISO 8583 MTI 0200. f12 = build_0200, t2 = PurchaseRequest, t3 = Iso8583Message

mod error;
mod iso8583;

pub use error::E4;
pub use iso8583::{f12, t2, t3};
