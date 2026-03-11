// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Backlog loading. f25=load_backlog from disk. Types in kova-core.

use std::path::Path;

use kova_core::Backlog;

/// f25 = load_backlog. Parse backlog.json from disk.
pub fn f25(p0: &Path) -> anyhow::Result<Backlog> {
    let v0 = std::fs::read_to_string(p0)?;
    let v1: Backlog = serde_json::from_str(&v0)?;
    Ok(v1)
}
