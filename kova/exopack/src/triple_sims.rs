// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! f60 = triple_sims — run test runner 3 times; all must pass.
//! TRIPLE SIMS: 3 sequential passes, exit 0 only if all pass.

use std::path::Path;
use std::process::Command;
use std::time::Instant;

/// f61 = run_cargo_test_n. Runs `cargo test` N times in project_dir. Returns (ok, stderr).
/// For kova and other orchestrators. N=3 = TRIPLE SIMS.
pub fn f61(project_dir: &Path, n: u32) -> (bool, String) {
    f61_with_args(project_dir, n, &[])
}

/// f61_with_args. f61 variant with extra cargo args (e.g. --no-default-features).
pub fn f61_with_args(project_dir: &Path, n: u32, args: &[&str]) -> (bool, String) {
    for i in 1..=n {
        let mut cmd = Command::new("cargo");
        cmd.arg("test").current_dir(project_dir);
        cmd.args(args);
        match cmd.output()
        {
            Ok(o) if o.status.success() => continue,
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr).into_owned();
                return (false, format!("TRIPLE SIMS pass {}/{} failed:\n{}", i, n, stderr));
            }
            Err(e) => return (false, format!("cargo test: {}", e)),
        }
    }
    (true, String::new())
}

/// Discover -test binary name from Cargo.toml. Returns first [[bin]] with name ending in "-test".
pub fn f63_discover_test_bin(project_dir: &Path) -> Option<String> {
    let manifest = project_dir.join("Cargo.toml");
    let content = std::fs::read_to_string(&manifest).ok()?;
    let mut in_bin = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("[[bin]]") {
            in_bin = true;
            continue;
        }
        if in_bin && trimmed.starts_with("name = ") {
            if let Some(name) = trimmed.strip_prefix("name = ").and_then(|s| s.strip_prefix('"')).and_then(|s| s.strip_suffix('"')) {
                if name.ends_with("-test") {
                    return Some(name.to_string());
                }
            }
            in_bin = false;
        }
    }
    None
}

/// f62=live_demo. Build and run a -test binary with live stdout/stderr for evaluation in motion.
/// Uses Stdio::inherit so output streams to the terminal. Returns exit status.
pub fn f62_live_demo(
    project_dir: &Path,
    bin_name: &str,
    cargo_args: &[&str],
) -> std::io::Result<std::process::ExitStatus> {
    let manifest = project_dir.join("Cargo.toml");
    if !manifest.exists() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            format!("Cargo.toml not found in {}", project_dir.display()),
        ));
    }

    // Build
    let mut build = std::process::Command::new("cargo");
    build
        .arg("build")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--bin")
        .arg(bin_name)
        .args(cargo_args)
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit());
    let status = build.status()?;
    if !status.success() {
        return Ok(status);
    }

    // Run with live output. TEST_DEMO=1 enables demo mode for self-evaluation.
    let mut run = std::process::Command::new("cargo");
    run.arg("run")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--bin")
        .arg(bin_name)
        .args(cargo_args)
        .env("TEST_DEMO", "1")
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .stdin(std::process::Stdio::inherit());
    run.status()
}

/// f60 = triple_sims_run. Runs `run_once` 3 times. Returns true iff all pass.
/// Prints pass count and timing. Use from test binary: exit 0 when true.
pub async fn f60<F, Fut>(run_once: F) -> bool
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    const N: u32 = 3;
    let mut passed = 0u32;
    for i in 1..=N {
        let start = Instant::now();
        let ok = run_once().await;
        let ms = start.elapsed().as_millis();
        if ok {
            passed += 1;
            println!("TRIPLE SIMS pass {}/{} OK ({}ms)", i, N, ms);
        } else {
            eprintln!("TRIPLE SIMS pass {}/{} FAILED ({}ms)", i, N, ms);
            return false;
        }
    }
    println!("TRIPLE SIMS: {}/{} passes OK", passed, N);
    passed == N
}
