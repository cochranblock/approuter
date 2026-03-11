// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! Storage layer. sled + bincode + zstd per Kova standards. Zero-copy IVec where possible.

use serde::{Deserialize, Serialize};
use std::path::Path;
use thiserror::Error;

/// E0 = StorageError. All storage failures use thiserror for graceful handling.
#[derive(Debug, Error)]
pub enum E0 {
    #[error("sled open failed: {0}")]
    SledOpen(#[from] sled::Error),
    #[error("bincode encode failed: {0}")]
    BincodeEncode(#[from] bincode::error::EncodeError),
    #[error("bincode decode failed: {0}")]
    BincodeDecode(#[from] bincode::error::DecodeError),
    #[error("zstd compress failed: {0}")]
    ZstdCompress(std::io::Error),
    #[error("zstd decompress failed: {0}")]
    ZstdDecompress(std::io::Error),
}

/// t12 = Store. sled-backed KV with bincode serialization and zstd payload compression.
/// Why: Self-contained, no external DB. High perf on gaming laptop.
pub struct t12 {
    db: sled::Db,
}

impl t12 {
    /// f39 = open. Open sled DB at path. Creates dir if needed.
    /// Why: Single init point for all persistence.
    pub fn f39(p: impl AsRef<Path>) -> Result<Self, E0> {
        let db = sled::open(p)?;
        Ok(Self { db })
    }

    /// f40 = put_compressed. Serialize with bincode, compress with zstd, store.
    /// Why: Internal format is compact; zstd reduces disk I/O for large payloads.
    pub fn f40<K: AsRef<[u8]>, V: Serialize>(&self, key: K, value: &V) -> Result<(), E0> {
        let encoded = bincode::serde::encode_to_vec(value, bincode::config::standard())?;
        let compressed = zstd::encode_all(encoded.as_slice(), 3)
            .map_err(E0::ZstdCompress)?;
        self.db.insert(key, compressed)?;
        Ok(())
    }

    /// f41 = get_compressed. Fetch, decompress, deserialize. Returns None if key missing.
    /// Why: Symmetric to f40; zero-copy not used here due to decompression step.
    pub fn f41<K: AsRef<[u8]>, V: for<'de> Deserialize<'de>>(
        &self,
        key: K,
    ) -> Result<Option<V>, E0> {
        let Some(v) = self.db.get(key)? else {
            return Ok(None);
        };
        let decompressed = zstd::decode_all(v.as_ref()).map_err(E0::ZstdDecompress)?;
        let (decoded, _) = bincode::serde::decode_from_slice(&decompressed, bincode::config::standard())?;
        Ok(Some(decoded))
    }

    /// f42 = put_raw. Store raw bytes (no compression). Use for small keys or already-compressed data.
    /// Why: Avoid double compression; sled::IVec preserves zero-copy on read.
    pub fn f42<K: AsRef<[u8]>>(&self, key: K, value: &[u8]) -> Result<(), E0> {
        self.db.insert(key, value)?;
        Ok(())
    }

    /// f43 = get_raw. Fetch raw bytes. Returns sled::IVec for zero-copy.
    /// Why: Caller can deserialize without allocating if using bincode from IVec.
    pub fn f43<K: AsRef<[u8]>>(&self, key: K) -> Result<Option<sled::IVec>, E0> {
        Ok(self.db.get(key)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestVal {
        x: i32,
        s: String,
    }

    /// f39+f40+f41=open,put_compressed,get_compressed
    #[test]
    fn store_put_get_compressed_roundtrip() {
        let tmp = tempfile::TempDir::new().unwrap();
        let store = t12::f39(tmp.path()).unwrap();
        let v = TestVal {
            x: 42,
            s: "hello".into(),
        };
        store.f40(b"key1", &v).unwrap();
        let got: Option<TestVal> = store.f41(b"key1").unwrap();
        assert_eq!(got, Some(v));
    }

    /// f41=get_compressed
    #[test]
    fn store_get_missing_returns_none() {
        let tmp = tempfile::TempDir::new().unwrap();
        let store = t12::f39(tmp.path()).unwrap();
        let got: Option<TestVal> = store.f41(b"missing").unwrap();
        assert_eq!(got, None);
    }

    /// f42+f43=put_raw,get_raw
    #[test]
    fn store_put_raw_get_raw() {
        let tmp = tempfile::TempDir::new().unwrap();
        let store = t12::f39(tmp.path()).unwrap();
        store.f42(b"raw", b"bytes").unwrap();
        let got = store.f43(b"raw").unwrap();
        assert_eq!(got.as_ref().map(|v| v.as_ref()), Some(b"bytes" as &[u8]));
    }
}
