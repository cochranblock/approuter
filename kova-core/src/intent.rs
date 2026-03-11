// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types)]
//! Intent layer. t0=Intent t1=IntentKind t2=Constraint. No I/O.

use serde::{Deserialize, Serialize};

/// t0 = Intent. Structured intent: what user wants, separate from how.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct t0 {
    pub s0: t1,
    pub s1: Option<String>,
    pub s2: Vec<t2>,
}

/// t1 = IntentKind
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum t1 {
    Compile {
        release: bool,
        check_only: bool,
    },
    Test,
    FixWarnings,
    FullPipeline,
    TunnelUpdate,
    SetupRoguerepo,
    CloudflarePurge,
    Custom {
        cmd: String,
        args: Vec<String>,
    },
}

/// t2 = Constraint. Deontic: must, may, must-not.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum t2 {
    MustNotBreakTests,
    MayUseGpu,
    TimeoutSecs(u64),
    MaxJobs(usize),
}

impl t0 {
    pub fn f18(p4: bool) -> Self {
        Self {
            s0: t1::Compile {
                release: p4,
                check_only: false,
            },
            s1: None,
            s2: vec![t2::MustNotBreakTests],
        }
    }

    pub fn f19() -> Self {
        Self {
            s0: t1::Test,
            s1: None,
            s2: vec![t2::MustNotBreakTests],
        }
    }

    pub fn f20() -> Self {
        Self {
            s0: t1::FullPipeline,
            s1: None,
            s2: vec![t2::MustNotBreakTests, t2::MayUseGpu],
        }
    }

    pub fn f21() -> Self {
        Self {
            s0: t1::TunnelUpdate,
            s1: None,
            s2: vec![],
        }
    }

    pub fn f22() -> Self {
        Self {
            s0: t1::SetupRoguerepo,
            s1: None,
            s2: vec![],
        }
    }

    pub fn f23() -> Self {
        Self {
            s0: t1::CloudflarePurge,
            s1: None,
            s2: vec![],
        }
    }
}

/// f62 = parse_intent. Map keywords → intent. No LLM.
pub fn f62(input: &str) -> Option<t0> {
    let lower = input.to_lowercase();
    let trimmed = lower.trim();
    if trimmed.is_empty() {
        return None;
    }
    if trimmed.contains("full") || trimmed.contains("pipeline") || trimmed.contains("run pipeline") {
        return Some(t0::f20());
    }
    if trimmed.contains("tunnel") || trimmed.contains("update tunnel") {
        return Some(t0::f21());
    }
    if trimmed.contains("setup") && (trimmed.contains("rogue") || trimmed.contains("repo")) {
        return Some(t0::f22());
    }
    if trimmed.contains("cloudflare") || trimmed.contains("purge") || trimmed.contains("cache") {
        return Some(t0::f23());
    }
    if trimmed.contains("test") && !trimmed.contains("full") {
        return Some(t0::f19());
    }
    if trimmed.contains("compile") || trimmed.contains("build") {
        let release = trimmed.contains("release");
        return Some(t0::f18(release));
    }
    if trimmed.contains("fix") && (trimmed.contains("warn") || trimmed.contains("warning")) {
        return Some(t0 {
            s0: t1::FixWarnings,
            s1: None,
            s2: vec![t2::MustNotBreakTests],
        });
    }
    None
}

/// Intent name for display (e.g. "full-pipeline").
pub fn intent_name(i: &t1) -> &'static str {
    match i {
        t1::Compile { .. } => "compile",
        t1::Test => "test",
        t1::FixWarnings => "fix-warnings",
        t1::FullPipeline => "full-pipeline",
        t1::TunnelUpdate => "tunnel-update",
        t1::SetupRoguerepo => "setup-roguerepo",
        t1::CloudflarePurge => "cloudflare-purge",
        t1::Custom { .. } => "custom",
    }
}
