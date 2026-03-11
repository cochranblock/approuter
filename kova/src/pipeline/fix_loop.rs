// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Fix loop: categorize errors, call Mechanic (Fixer) model.

use std::path::Path;

use super::error_kind::error_block_with_context;

/// f116=fix_and_retry. Categorize stderr, feed context to Mechanic, return fixed code.
pub async fn fix_and_retry(
    fix_path: &Path,
    project_dir: &Path,
    stage: &str,
    stderr: &str,
    code: &str,
) -> Result<String, String> {
    let error_block = error_block_with_context(stage, stderr);
    let fix_prompt = format!(
        "Fix this Rust code. {}\n\nCode:\n```rust\n{}\n```\n\nReply with only the fixed code in ```rust ... ```.",
        error_block, code
    );
    let cursor = crate::cursor_prompts::load_cursor_prompts(project_dir);
    let fix_system = if cursor.is_empty() {
        format!("You fix Rust {} errors.", stage)
    } else {
        format!("You fix Rust {} errors.\n\n--- Cursor rules ---\n{}", stage, cursor)
    };
    crate::inference::f80(
        fix_path,
        &fix_system,
        &fix_prompt,
    )
    .await
    .map_err(|e| format!("Error on fix: {}", e))
}
