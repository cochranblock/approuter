// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Backlog format. JSON-serializable. entry_to_intent is pure logic.

use serde::{Deserialize, Serialize};

use crate::intent::{t0, t1};

/// t8 = BacklogEntry. One item in backlog.json.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BacklogEntry {
    pub intent: String,
    #[serde(default)]
    pub project: Option<String>,
    #[serde(default)]
    pub approuter_dir: Option<String>,
    #[serde(default)]
    pub cmd: Option<String>,
    #[serde(default)]
    pub args: Option<Vec<String>>,
}

/// t9 = Backlog. items = Vec<BacklogEntry>.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Backlog {
    pub items: Vec<BacklogEntry>,
}

/// Map backlog entry to intent. Returns None if unsupported.
pub fn entry_to_intent(entry: &BacklogEntry) -> Option<t0> {
    let intent = entry.intent.to_lowercase();
    let project_hint = entry.project.as_ref().and_then(|p| {
        std::path::Path::new(p)
            .file_name()
            .and_then(|n| n.to_str())
            .map(|s| s.to_string())
    });
    let with_hint = |mut i: t0| {
        i.s1 = project_hint.clone();
        i
    };
    match intent.as_str() {
        "full-pipeline" => Some(with_hint(t0::f20())),
        "tunnel-update" => Some(with_hint(t0::f21())),
        "setup-roguerepo" => Some(with_hint(t0::f22())),
        "cloudflare-purge" => Some(with_hint(t0::f23())),
        "test" => Some(with_hint(t0::f19())),
        "custom" => {
            let cmd = entry.cmd.clone()?;
            let args = entry.args.clone().unwrap_or_default();
            Some(t0 {
                s0: t1::Custom { cmd, args },
                s1: project_hint,
                s2: vec![],
            })
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::intent::{t0, t1};

    #[test]
    fn entry_to_intent_full_pipeline() {
        let e = BacklogEntry {
            intent: "full-pipeline".into(),
            project: None,
            approuter_dir: None,
            cmd: None,
            args: None,
        };
        assert!(matches!(entry_to_intent(&e), Some(t0 { s0: t1::FullPipeline, .. })));
    }

    #[test]
    fn entry_to_intent_custom() {
        let e = BacklogEntry {
            intent: "custom".into(),
            project: None,
            approuter_dir: None,
            cmd: Some("cargo".into()),
            args: Some(vec!["check".into()]),
        };
        let got = entry_to_intent(&e).unwrap();
        match &got.s0 {
            t1::Custom { cmd, args } => {
                assert_eq!(cmd, "cargo");
                assert_eq!(args, &["check"]);
            }
            _ => panic!("expected Custom"),
        }
    }

    #[test]
    fn entry_to_intent_unknown_returns_none() {
        let e = BacklogEntry {
            intent: "unknown-intent".into(),
            project: None,
            approuter_dir: None,
            cmd: None,
            args: None,
        };
        assert!(entry_to_intent(&e).is_none());
    }
}
