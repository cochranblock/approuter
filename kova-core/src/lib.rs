// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Kova core types. No I/O. WASM-safe. Shared by kova server and WASM client.

pub mod intent;
pub mod backlog;

pub use intent::{f62, intent_name, t0, t1, t2};
pub use backlog::{entry_to_intent, Backlog, BacklogEntry};
