// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Kova — augment engine. Core lib for GUI + serve.

pub mod backlog;
pub mod config;
pub mod cursor_prompts;
pub mod context;
pub mod trace;
pub mod compute;
pub mod plan;
pub mod storage;

pub use kova_core::{entry_to_intent, f62, intent_name, t0, t1, t2, Backlog, BacklogEntry};
pub use backlog::f25;
pub use config::{
    all_build_presets, backlog_path, bind_addr, bootstrap, cursor_prompts_enabled, default_project,
    discover_projects, f78, home, inference_model_path, infer_preset_name, kova_dir, load_build_preset,
    load_prompt, models_dir, orchestration_max_fix_retries, orchestration_router_resident,
    orchestration_run_clippy, orchestration_specialist_idle_unload_secs, prompts_dir,
    sled_path, workspace_root, BuildPreset, ModelRole,
};
pub use context::{f73, f74, Message};
pub use compute::{t6, t7};
pub use plan::{t3, t4, t5};
#[cfg(feature = "inference")]
pub use router::{f79, RouterResult};

#[cfg(feature = "gui")]
pub mod gui;
#[cfg(feature = "gui")]
pub mod theme;
#[cfg(feature = "gui")]
pub mod output;
#[cfg(feature = "serve")]
pub mod serve;
#[cfg(feature = "inference")]
pub mod inference;
#[cfg(feature = "inference")]
pub mod model;
#[cfg(feature = "inference")]
pub mod router;
#[cfg(feature = "inference")]
pub mod academy;
#[cfg(feature = "inference")]
pub mod pipeline;
#[cfg(feature = "inference")]
pub mod context_loader;
pub mod recent_changes;

#[cfg(feature = "autopilot")]
pub mod autopilot;

#[cfg(feature = "daemon")]
pub mod daemon;

pub mod c2;
pub mod elicitor;
pub mod inspect;
pub mod ssh_ca;

#[cfg(test)]
mod test_utils;

/// f25 alias for integration tests.
pub fn load_backlog(p: &std::path::Path) -> anyhow::Result<Backlog> {
    f25(p)
}

/// f90=run_test_suite. Deploy quality gate: clippy, TRIPLE SIMS, release build, smoke, baked demo.
#[cfg(feature = "tests")]
pub fn run_test_suite() -> anyhow::Result<()> {
    use std::path::Path;
    use std::process::Command;

    let project = Path::new(env!("CARGO_MANIFEST_DIR"));

    fn run_cargo(project: &Path, args: &[&str]) -> (bool, String) {
        match Command::new("cargo")
            .args(args)
            .current_dir(project)
            .output()
        {
            Ok(o) => {
                let stderr = String::from_utf8_lossy(&o.stderr).into_owned();
                (o.status.success(), stderr)
            }
            Err(e) => (false, e.to_string()),
        }
    }

    println!("kova test: cargo clippy...");
    let (ok, stderr) = run_cargo(project, &["clippy", "--", "-D", "warnings"]);
    if !ok {
        anyhow::bail!("clippy failed:\n{}", stderr);
    }

    println!("kova test: TRIPLE SIMS (cargo test -p kova 3x)...");
    let project_buf = project.to_path_buf();
    let (ok, stderr) = exopack::triple_sims::f61_with_args(
        &project_buf,
        3,
        &["-p", "kova", "--features", "serve,tests"],
    );
    if !ok {
        anyhow::bail!("{}", stderr);
    }

    println!("kova test: cargo build --release --features serve --target aarch64-apple-darwin...");
    let target_dir = project.join("target");
    let target_triple = "aarch64-apple-darwin";
    let (ok, stderr) = match Command::new("cargo")
        .args([
            "build",
            "--release",
            "--features",
            "serve",
            "--target",
            target_triple,
        ])
        .current_dir(project)
        .env("CARGO_TARGET_DIR", &target_dir)
        .output()
    {
        Ok(o) => {
            let s = String::from_utf8_lossy(&o.stderr).into_owned();
            (o.status.success(), s)
        }
        Err(e) => (false, e.to_string()),
    };
    if !ok {
        anyhow::bail!("release build failed:\n{}", stderr);
    }

    println!("kova test: release smoke (bootstrap + c2 nodes)...");
    let tmp = tempfile::TempDir::new()?;
    let home = tmp.path().to_path_buf();
    let kova_bin = project
        .join(format!("target/{}/release/kova", target_triple))
        .with_extension(std::env::consts::EXE_EXTENSION);
    if !kova_bin.exists() {
        anyhow::bail!("release binary not found: {:?}", kova_bin);
    }
    let out = Command::new(&kova_bin)
        .env("HOME", &home)
        .arg("bootstrap")
        .output()?;
    if !out.status.success() {
        anyhow::bail!(
            "kova bootstrap failed:\n{}",
            String::from_utf8_lossy(&out.stderr)
        );
    }
    let c2_out = Command::new(&kova_bin)
        .env("HOME", &home)
        .args(["c2", "nodes"])
        .output()?;
    if !c2_out.status.success() {
        anyhow::bail!(
            "kova c2 nodes failed:\n{}",
            String::from_utf8_lossy(&c2_out.stderr)
        );
    }
    let stdout = String::from_utf8_lossy(&c2_out.stdout);
    if !stdout.contains("lf") {
        anyhow::bail!("kova c2 nodes: expected lf in output, got:\n{}", stdout);
    }

    if std::env::var("KOVA_SKIP_BAKED_DEMO").is_ok() || std::env::var("KOVA_BAKED_DEMO").is_err() {
        println!("kova test: baked demo (skipped; set KOVA_BAKED_DEMO=1 to run)");
    } else {
        println!("kova test: baked demo (full intended usage, no user input)...");
        let port = 19402u16;
        let kova_bin = kova_bin.to_path_buf();
        let home = home.to_path_buf();
        std::thread::scope(|s| {
            let (tx, rx) = std::sync::mpsc::channel();
            s.spawn(move || {
                let rt = tokio::runtime::Builder::new_multi_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| anyhow::anyhow!("{}", e));
                let result = match rt {
                    Ok(r) => r.block_on(exopack::baked_demo::run_baked_demo(&kova_bin, &home, port))
                        .map_err(|e| anyhow::anyhow!("{}", e)),
                    Err(e) => Err(e),
                };
                let _ = tx.send(result);
            });
            rx.recv().unwrap()
        })?;
    }

    println!("kova test: all checks passed");
    Ok(())
}
