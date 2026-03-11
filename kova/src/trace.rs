// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Last pipeline run for "Explain" feature. In-memory only. t93=LastTrace.

use serde::{Deserialize, Serialize};

/// t93=LastTrace. Last pipeline run for Explain feature.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LastTrace {
    pub intent: String,
    pub user_msg: String,
    pub stage: String,       // "compile" | "clippy" | "tests"
    pub stderr: String,
    pub retry_count: u32,
    pub outcome: String,    // "success" | "failed"
    pub chain: Vec<String>, // "Attempt 1: compile failed" etc.
}
