// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types)]
//! f86=recent_changes_snapshot, f87=format_recent_changes.
//! Tokenized output for LLM context. Stay on task with latest modified work.

use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

/// t86=RecentChange. s0=path, s1=mtime_secs, s2=kind.
#[derive(Debug, Clone)]
pub struct t86 {
    /// s0 = relative path from project root
    pub s0: String,
    /// s1 = mtime as unix secs (for ordering)
    pub s1: u64,
    /// s2 = kind: "modified" | "created" | "deleted" (polling: modified only)
    pub s2: String,
}

/// f86=recent_changes_snapshot. Poll project_dir for files modified within `within` duration.
/// Returns tokenized list. Ignores .git, target, node_modules.
pub fn f86(project_dir: &Path, within: Duration) -> Vec<t86> {
    let cutoff = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .saturating_sub(within);
    let mut out = Vec::new();
    let root = project_dir.canonicalize().unwrap_or_else(|_| project_dir.to_path_buf());
    walk_recent(&root, &root, cutoff.as_secs(), &mut out);
    out.sort_by(|a, b| b.s1.cmp(&a.s1)); // newest first
    out
}

fn walk_recent(root: &Path, dir: &Path, cutoff_secs: u64, out: &mut Vec<t86>) {
    let Ok(entries) = std::fs::read_dir(dir) else { return };
    for e in entries.flatten() {
        let path = e.path();
        let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        if name.starts_with('.') || name == "target" || name == "node_modules" {
            continue;
        }
        let meta = match std::fs::metadata(&path) {
            Ok(m) => m,
            Err(_) => continue,
        };
        if meta.is_dir() {
            walk_recent(root, &path, cutoff_secs, out);
        } else {
            let mtime = meta
                .modified()
                .ok()
                .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                .map(|d| d.as_secs())
                .unwrap_or(0);
            if mtime >= cutoff_secs {
                let rel = path
                    .strip_prefix(root)
                    .ok()
                    .and_then(|p| p.to_str())
                    .map(String::from)
                    .unwrap_or_else(|| path.display().to_string());
                out.push(t86 {
                    s0: rel,
                    s1: mtime,
                    s2: "modified".into(),
                });
            }
        }
    }
}

/// f87=format_recent_changes. Tokenized string for LLM prompt injection.
/// Compact: path + mtime. Use when non-empty.
pub fn f87(changes: &[t86]) -> String {
    if changes.is_empty() {
        return String::new();
    }
    let lines: Vec<String> = changes
        .iter()
        .map(|c| format!("  {} s1={}", c.s0, c.s1))
        .collect();
    format!(
        "\n\n---\nRecent changes (f86, newest first):\n{}\n---\n",
        lines.join("\n")
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn f86_empty_when_no_recent() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("old.rs"), "// old").unwrap();
        std::thread::sleep(Duration::from_secs(2));
        let out = f86(tmp.path(), Duration::from_secs(1));
        assert!(out.is_empty(), "no files modified in last 1s (file written 2s ago)");
    }

    #[test]
    fn f86_includes_recent_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("new.rs"), "// new").unwrap();
        let out = f86(tmp.path(), Duration::from_secs(5));
        assert!(!out.is_empty());
        assert!(out.iter().any(|c| c.s0 == "new.rs"));
    }

    #[test]
    fn f87_empty_when_no_changes() {
        assert!(f87(&[]).is_empty());
    }

    #[test]
    fn f87_formats_changes() {
        let c = vec![
            t86 {
                s0: "src/lib.rs".into(),
                s1: 1700000000,
                s2: "modified".into(),
            },
        ];
        let s = f87(&c);
        assert!(s.contains("src/lib.rs"));
        assert!(s.contains("f86"));
    }
}
