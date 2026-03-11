// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Project context for code gen. Cargo.toml, mentioned .rs files, recent changes.
//! f82=load_project_context

use std::path::Path;
use std::time::Duration;

use crate::recent_changes::t86;

/// Project context for coder prompt. Cargo.toml deps + file contents + recent changes.
#[derive(Debug, Default)]
/// t90=ProjectContext. Cargo.toml + mentioned .rs files for code-gen.
pub struct ProjectContext {
    pub cargo_toml: Option<String>,
    pub files: Vec<(String, String)>,
    /// f86 recent changes. When set, format_context includes tokenized list for LLM.
    pub recent_changes: Option<Vec<t86>>,
}

/// f82=load_project_context. Load Cargo.toml and .rs files mentioned in user_input.
pub fn f82(project_dir: &Path, user_input: &str) -> ProjectContext {
    f82_with_recent(project_dir, user_input, None)
}

/// f82_with_recent. Like f82 but optionally includes recent changes (f86) for LLM context.
/// within_minutes: files modified in last N minutes. None = skip recent changes.
pub fn f82_with_recent(
    project_dir: &Path,
    user_input: &str,
    within_minutes: Option<u64>,
) -> ProjectContext {
    let mut ctx = ProjectContext::default();

    let cargo_path = project_dir.join("Cargo.toml");
    if cargo_path.exists() {
        ctx.cargo_toml = std::fs::read_to_string(&cargo_path).ok();
    }

    for name in extract_mentioned_rs_files(user_input) {
        let content = try_read_rs_file(project_dir, &name);
        if let Some(c) = content {
            ctx.files.push((name, c));
        }
    }

    if let Some(mins) = within_minutes {
        let changes = crate::recent_changes::f86(project_dir, Duration::from_secs(mins * 60));
        if !changes.is_empty() {
            ctx.recent_changes = Some(changes);
        }
    }

    ctx
}

/// f83=target_file_hint. First .rs file mentioned in user input, for Apply target.
pub fn f83(s: &str) -> Option<String> {
    extract_mentioned_rs_files(s).into_iter().next()
}

/// Extract potential .rs filenames from user input (e.g. "plan.rs", "in compute.rs").
fn extract_mentioned_rs_files(s: &str) -> Vec<String> {
    let mut seen = std::collections::HashSet::new();
    let mut out = Vec::new();
    let s = s.to_lowercase();
    let mut i = 0;
    while i < s.len() {
        if let Some(idx) = s[i..].find(".rs") {
            let end = i + idx;
            let start = end.saturating_sub(50);
            let slice = &s[start..end];
            let word_start = slice
                .rfind(|c: char| !c.is_alphanumeric() && c != '_' && c != '/')
                .map(|p| p + 1)
                .unwrap_or(0);
            let word = slice[word_start..].trim();
            if !word.is_empty() && word.chars().all(|c| c.is_alphanumeric() || c == '_') {
                let name = format!("{}.rs", word);
                if seen.insert(name.clone()) {
                    out.push(name);
                }
            }
            i = end + 3;
        } else {
            break;
        }
    }
    out
}

fn try_read_rs_file(project_dir: &Path, name: &str) -> Option<String> {
    for subdir in ["src", ""] {
        let p = if subdir.is_empty() {
            project_dir.join(name)
        } else {
            project_dir.join(subdir).join(name)
        };
        if p.exists() {
            return std::fs::read_to_string(&p).ok();
        }
    }
    None
}

/// Format context for injection into coder prompt.
/// Includes recent changes (f86/f87) when present — helps LLM stay on task.
/// f112=format_context. Format ProjectContext for LLM prompt injection.
pub fn format_context(ctx: &ProjectContext) -> String {
    let mut parts = Vec::new();
    if let Some(ref cargo) = ctx.cargo_toml {
        parts.push(format!("## Cargo.toml\n```toml\n{}\n```", cargo.trim()));
    }
    for (path, content) in &ctx.files {
        parts.push(format!("## {}\n```rust\n{}\n```", path, content.trim()));
    }
    let base = if parts.is_empty() {
        String::new()
    } else {
        format!("\n\n---\nProject context:\n{}\n---\n", parts.join("\n\n"))
    };
    let recent = ctx
        .recent_changes
        .as_ref()
        .map(|c| crate::recent_changes::f87(c))
        .unwrap_or_default();
    format!("{}{}", base, recent)
}

#[cfg(test)]
mod tests {
    #[test]
    fn extract_mentioned_rs_files() {
        let s = "add a retry to plan.rs and compute.rs";
        let out = super::extract_mentioned_rs_files(s);
        assert!(out.contains(&"plan.rs".into()));
        assert!(out.contains(&"compute.rs".into()));
    }

    #[test]
    fn extract_dedupes() {
        let s = "plan.rs plan.rs";
        let out = super::extract_mentioned_rs_files(s);
        assert_eq!(out.len(), 1);
    }

    #[test]
    fn format_context_empty() {
        let ctx = super::ProjectContext::default();
        assert!(super::format_context(&ctx).is_empty());
    }

    #[test]
    fn format_context_cargo_only() {
        let ctx = super::ProjectContext {
            cargo_toml: Some("[package]\nname = \"foo\"".into()),
            files: vec![],
            recent_changes: None,
        };
        let s = super::format_context(&ctx);
        assert!(s.contains("Cargo.toml"));
        assert!(s.contains("name = \"foo\""));
    }

    #[test]
    fn format_context_with_files() {
        let ctx = super::ProjectContext {
            cargo_toml: None,
            files: vec![("lib.rs".into(), "fn main() {}".into())],
            recent_changes: None,
        };
        let s = super::format_context(&ctx);
        assert!(s.contains("lib.rs"));
        assert!(s.contains("fn main()"));
    }

    #[test]
    fn f82_loads_cargo_toml() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("Cargo.toml"), "[package]\nname = \"test\"").unwrap();
        let ctx = super::f82(tmp.path(), "add something");
        assert!(ctx.cargo_toml.is_some());
        assert!(ctx.cargo_toml.as_ref().unwrap().contains("test"));
    }

    #[test]
    fn f83_target_hint() {
        assert_eq!(super::f83("add to plan.rs"), Some("plan.rs".into()));
        assert_eq!(super::f83("fix compute.rs"), Some("compute.rs".into()));
        assert_eq!(super::f83("no file here"), None);
    }

    #[test]
    fn f82_loads_mentioned_rs_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src/plan.rs"), "// plan").unwrap();
        let ctx = super::f82(tmp.path(), "add to plan.rs");
        assert_eq!(ctx.files.len(), 1);
        assert_eq!(ctx.files[0].0, "plan.rs");
        assert!(ctx.files[0].1.contains("plan"));
    }

    #[test]
    fn f82_prefers_src_over_root() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::create_dir_all(tmp.path().join("src")).unwrap();
        std::fs::write(tmp.path().join("src/lib.rs"), "// from src").unwrap();
        std::fs::write(tmp.path().join("lib.rs"), "// from root").unwrap();
        let ctx = super::f82(tmp.path(), "lib.rs");
        assert_eq!(ctx.files.len(), 1);
        assert!(ctx.files[0].1.contains("from src"));
    }

    #[test]
    fn f82_skips_nonexistent_file() {
        let tmp = tempfile::TempDir::new().unwrap();
        let ctx = super::f82(tmp.path(), "add to nonexistent.rs");
        assert_eq!(ctx.files.len(), 0);
    }

    #[test]
    fn extract_handles_underscores() {
        let out = super::extract_mentioned_rs_files("fix my_module.rs");
        assert_eq!(out, ["my_module.rs"]);
    }
}
