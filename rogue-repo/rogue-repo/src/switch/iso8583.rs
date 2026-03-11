// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! f12 = build_0200, t2 = PurchaseRequest, t3 = Iso8583Message

use bitvec::prelude::*;
use bytes::BytesMut;
use std::io::Write;

use super::E4;

const MTI_0200: &[u8] = b"0200";

#[derive(Debug, Clone)]
pub struct t2 {
    pub pan_encrypted: Vec<u8>,
    pub amount_cents: u64,
    pub stan: u32,
}

impl Default for t2 {
    fn default() -> Self {
        Self {
            pan_encrypted: Vec::new(),
            amount_cents: 420,
            stan: 0,
        }
    }
}

#[derive(Debug)]
pub struct t3 {
    pub raw: Vec<u8>,
}

/// f12 = build_0200 for $4.20 (420 cents)
pub fn f12(req: &t2) -> Result<t3, E4> {
    if req.amount_cents == 0 || req.amount_cents > 99_999_999 {
        return Err(E4::Amount("1-99999999 cents".into()));
    }
    let now = chrono::Utc::now();
    let time_str = now.format("%H%M%S").to_string();
    let date_str = now.format("%m%d").to_string();
    let amount_str = format!("{:012}", req.amount_cents);
    let proc_code = "000000";
    let currency = "840";

    let mut bitmap = bitvec![u8, Msb0; 0u8; 64];
    bitmap.set(1, true);
    bitmap.set(2, true);
    bitmap.set(3, true);
    bitmap.set(10, true);
    bitmap.set(11, true);
    bitmap.set(12, true);
    bitmap.set(48, true);

    let bitmap_bytes: Vec<u8> = bitmap.as_raw_slice().to_vec();
    if bitmap_bytes.len() != 8 {
        return Err(E4::Pack("bitmap 8 bytes".into()));
    }

    let pan_len = req.pan_encrypted.len();
    if pan_len > 99 {
        return Err(E4::Pack("PAN too long".into()));
    }
    let mut pan_buf = BytesMut::new();
    pan_buf.extend_from_slice(format!("{:02}", pan_len).as_bytes());
    pan_buf.extend_from_slice(&req.pan_encrypted);

    let mut out = Vec::new();
    out.write_all(MTI_0200)
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(&bitmap_bytes)
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(&pan_buf)
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(proc_code.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(amount_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(format!("{:06}", req.stan).as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(time_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(date_str.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;
    out.write_all(currency.as_bytes())
        .map_err(|e| E4::Pack(e.to_string()))?;

    Ok(t3 { raw: out })
}
