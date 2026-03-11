// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Integration tests. f30 scope. No self-licking.

use kova::{load_backlog, t0, t3, t5, t6};
use std::path::PathBuf;
use std::process::Command;

#[test]
fn full_pipeline_on_temp_fixture() {
    let v0 = tempfile::TempDir::new().unwrap();
    std::fs::write(
        v0.path().join("Cargo.toml"),
        r#"[package]
name = "fixture"
version = "0.1.0"
edition = "2021"
"#,
    )
    .unwrap();
    std::fs::create_dir_all(v0.path().join("src")).unwrap();
    std::fs::write(
        v0.path().join("src/lib.rs"),
        r#"pub fn x() -> i32 { 42 }
#[cfg(test)]
mod tests { use super::*; #[test] fn t() { assert_eq!(x(), 42); } }
"#,
    )
    .unwrap();
    let v1 = t0::f20();
    let v2 = t3::f14(&v1, v0.path().to_path_buf(), None);
    let v3 = t6::default();
    let v4 = v3.f15(&v2).unwrap();
    assert!(
        v4.iter().all(|x| x.s11),
        "full pipeline on fixture must pass"
    );
}

#[test]
fn load_backlog_roundtrip() {
    let v0 = tempfile::TempDir::new().unwrap();
    let v1 = v0.path().join("b.json");
    std::fs::write(&v1, r#"{"items":[{"intent":"full-pipeline"}]}"#).unwrap();
    let v2 = load_backlog(&v1).unwrap();
    assert_eq!(v2.items.len(), 1);
    assert_eq!(v2.items[0].intent, "full-pipeline");
}

#[test]
#[ignore = "requires ~/rogue-repo; run with: cargo test --ignored full_pipeline_on_rogue_repo"]
fn full_pipeline_on_rogue_repo() {
    let v0 = PathBuf::from(env!("HOME")).join("rogue-repo");
    if !v0.exists() {
        eprintln!("skip: rogue-repo not found at {:?}", v0);
        return;
    }
    let v1 = t0::f20();
    let v2 = t3::f14(&v1, v0.clone(), None);
    let v3 = t6::default();
    let v4 = v3.f15(&v2).unwrap();
    let v5 = v4.iter().all(|x| x.s11);
    for x in &v4 {
        if !x.s11 && !x.s13.is_empty() {
            eprintln!("action {} failed: {}", x.s10, x.s13);
        }
    }
    assert!(v5, "full pipeline must pass on rogue-repo");
}

#[test]
fn tunnel_update_plan_has_approuter_action() {
    let v0 = t0::f21();
    let tmp = tempfile::TempDir::new().unwrap();
    let v2 = t3::f14(&v0, PathBuf::from("."), Some(tmp.path().to_path_buf()));
    assert_eq!(v2.s3.len(), 1);
    assert!(matches!(v2.s3[0].s6, t5::ApprouterUpdateTunnel));
}

#[test]
fn setup_roguerepo_plan_has_approuter_action() {
    let v0 = t0::f22();
    let tmp = tempfile::TempDir::new().unwrap();
    let v2 = t3::f14(&v0, PathBuf::from("."), Some(tmp.path().to_path_buf()));
    assert_eq!(v2.s3.len(), 1);
    assert!(matches!(v2.s3[0].s6, t5::ApprouterSetupRoguerepo));
}

#[test]
fn plan_carries_project_hint_from_intent() {
    let mut intent = t0::f20();
    intent.s1 = Some("oakilydokily".into());
    let tmp = tempfile::TempDir::new().unwrap();
    let plan = t3::f14(&intent, tmp.path().to_path_buf(), None);
    assert_eq!(plan.s7.as_deref(), Some("oakilydokily"));
}

/// Run `kova prompts` directly. Proves baked rules load in the real binary.
#[test]
fn kova_prompts_includes_baked() {
    let tmp = tempfile::TempDir::new().unwrap();
    let out = Command::new(env!("CARGO_BIN_EXE_kova"))
        .arg("prompts")
        .env("HOME", tmp.path())
        .env("KOVA_PROJECT", tmp.path())
        .output()
        .expect("spawn kova prompts");
    assert!(
        out.status.success(),
        "kova prompts failed: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    let stdout = String::from_utf8_lossy(&out.stdout);
    assert!(
        stdout.contains("Blocking Only") || stdout.contains("P20"),
        "baked blocking rule missing"
    );
    assert!(stdout.contains("f81"), "baked compression_map missing");
}
