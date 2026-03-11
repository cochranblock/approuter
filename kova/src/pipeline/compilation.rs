// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f91–f93. Cargo check, clippy, test.

use std::path::Path;
use std::process::Command;

/// f91=cargo_check. Returns (success, stderr).
pub fn cargo_check(project_dir: &Path) -> (bool, String) {
    run_cargo(project_dir, &["check"])
}

/// f92=cargo_clippy. Returns (success, stderr).
pub fn cargo_clippy(project_dir: &Path) -> (bool, String) {
    run_cargo(project_dir, &["clippy", "--", "-D", "warnings"])
}

/// f93=cargo_test. Returns (success, stderr).
pub fn cargo_test(project_dir: &Path) -> (bool, String) {
    run_cargo(project_dir, &["test"])
}

/// f117=run_cargo. Invoke cargo with args. Returns (success, stderr).
pub fn run_cargo(project_dir: &Path, args: &[&str]) -> (bool, String) {
    match Command::new("cargo")
        .args(args)
        .current_dir(project_dir)
        .output()
    {
        Ok(o) => {
            let stderr = String::from_utf8_lossy(&o.stderr).into_owned();
            (o.status.success(), stderr)
        }
        Err(e) => (false, e.to_string()),
    }
}
