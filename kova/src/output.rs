// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Output helpers. Diff, apply. f84=format_diff f85=resolve_target_path

use std::path::{Path, PathBuf};

/// f85=resolve_target_path. Project dir + hint -> full path. Default src/lib.rs.
pub fn f85(project_dir: &Path, hint: Option<&str>) -> PathBuf {
    let name = hint.unwrap_or("lib.rs");
    project_dir.join("src").join(name)
}

/// f84=format_diff. Unified diff of old vs new.
#[cfg(any(feature = "gui", feature = "serve"))]
pub fn f84(old: &str, new: &str) -> String {
    use similar::{ChangeTag, TextDiff};
    let diff = TextDiff::from_lines(old, new);
    let mut out = String::new();
    for op in diff.ops() {
        for change in diff.iter_changes(op) {
            let mark = match change.tag() {
                ChangeTag::Delete => "-",
                ChangeTag::Insert => "+",
                ChangeTag::Equal => " ",
            };
            out.push_str(mark);
            out.push_str(change.value());
            if !change.value().ends_with('\n') {
                out.push('\n');
            }
        }
    }
    out
}

#[cfg(test)]
#[cfg(any(feature = "gui", feature = "serve"))]
mod tests {
    use super::*;

    #[test]
    fn f84_diff() {
        let old = "fn a() {}\nfn b() {}";
        let new = "fn a() {}\nfn b() {}\nfn c() {}";
        let d = f84(old, new);
        assert!(d.contains("+"));
        assert!(d.contains("fn c()"));
    }

    #[test]
    fn f85_resolve_target() {
        let dir = std::path::Path::new("/tmp");
        assert_eq!(f85(dir, Some("plan.rs")).to_string_lossy(), "/tmp/src/plan.rs");
        assert_eq!(f85(dir, None).to_string_lossy(), "/tmp/src/lib.rs");
    }
}
