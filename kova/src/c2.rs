// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! kova c2 — Tokenized orchestration. f18–f23 local or broadcast.

#![allow(non_camel_case_types)]

use clap::ValueEnum;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Clone, Copy, ValueEnum)]
pub enum Token {
    #[value(name = "f18")]
    F18,
    #[value(name = "f19")]
    F19,
    #[value(name = "f20")]
    F20,
    #[value(name = "f21")]
    F21,
    #[value(name = "f22")]
    F22,
    #[value(name = "f23")]
    F23,
}

impl Token {
    pub fn to_intent(self, release: bool) -> kova_core::t0 {
        match self {
            Token::F18 => kova_core::t0::f18(release),
            Token::F19 => kova_core::t0::f19(),
            Token::F20 => kova_core::t0::f20(),
            Token::F21 => kova_core::t0::f21(),
            Token::F22 => kova_core::t0::f22(),
            Token::F23 => kova_core::t0::f23(),
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Token::F18 => "f18",
            Token::F19 => "f19",
            Token::F20 => "f20",
            Token::F21 => "f21",
            Token::F22 => "f22",
            Token::F23 => "f23",
        }
    }

    pub fn is_local_only(&self) -> bool {
        matches!(self, Token::F21 | Token::F22 | Token::F23)
    }
}

pub fn default_nodes() -> Vec<&'static str> {
    vec!["lf", "gd", "bt", "st"]
}

pub fn resolve_project(project: Option<PathBuf>) -> PathBuf {
    project
        .or_else(|| std::env::var("KOVA_PROJECT").ok().map(PathBuf::from))
        .filter(|p| p.exists())
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")))
}

fn is_under_hive_vault(p: &Path) -> bool {
    if let Ok(home) = std::env::var("HOME") {
        let hive_vault = format!("{}/hive-vault", home);
        return p.to_string_lossy().starts_with(&hive_vault);
    }
    false
}

fn to_worker_path(p: &Path) -> PathBuf {
    let s = p.to_string_lossy();
    if let Ok(home) = std::env::var("HOME") {
        let hive_vault = format!("{}/hive-vault", home);
        if s.starts_with(&hive_vault) {
            let rest = s.strip_prefix(&hive_vault).unwrap_or("");
            return PathBuf::from("/mnt/hive").join(rest.trim_start_matches('/'));
        }
    }
    p.to_path_buf()
}

fn run_local(plan: &crate::plan::t3) -> anyhow::Result<()> {
    let exec = crate::compute::t6;
    let results = exec.f15(plan)?;
    for r in &results {
        let mark = if r.s11 { "✓" } else { "✗" };
        eprintln!("{} {} {}", mark, r.s10, if r.s13.is_empty() { "" } else { &r.s13 });
    }
    let all_ok = results.iter().all(|r| r.s11);
    if !all_ok {
        anyhow::bail!("One or more actions failed");
    }
    Ok(())
}

/// f120=kova_c2_broadcast. SSH broadcast to workers.
fn run_broadcast(plan: &crate::plan::t3, nodes: &[&str]) -> anyhow::Result<()> {
    let worker_path = to_worker_path(&plan.s4);
    for step in &plan.s3 {
        let cmd = match &step.s6 {
            crate::plan::t5::CargoCheck => "cargo check",
            crate::plan::t5::CargoTest => "cargo test",
            crate::plan::t5::CargoBuild { release } => {
                if *release {
                    "cargo build --release"
                } else {
                    "cargo build"
                }
            }
            _ => continue,
        };
        for node in nodes {
            eprintln!("[ ❯ ] {} → {}", node, cmd);
            let status = Command::new("ssh")
                .arg(*node)
                .arg(format!("cd {} && {}", worker_path.display(), cmd))
                .status()?;
            if !status.success() {
                anyhow::bail!("{} failed on {}", cmd, node);
            }
        }
    }
    eprintln!("[ ✔ ] Broadcast complete");
    Ok(())
}

/// f119=kova_c2_run. CLI orchestration. Local or broadcast.
pub fn run_command(
    token: Token,
    project: Option<PathBuf>,
    broadcast: bool,
    release: bool,
    nodes_override: Option<String>,
) -> anyhow::Result<()> {
    let project_path = resolve_project(project);
    let intent = token.to_intent(release);
    let approuter_dir = std::env::var("HOME")
        .ok()
        .map(|h| PathBuf::from(h).join("approuter"));
    let plan = crate::plan::t3::f14(&intent, project_path.clone(), approuter_dir);

    if plan.s3.is_empty() {
        eprintln!("No actions for {}", token.name());
        return Ok(());
    }

    if token.is_local_only() {
        run_local(&plan)
    } else if broadcast {
        // Pre-flight: project must be under hive-vault for broadcast
        if !is_under_hive_vault(&project_path) {
            anyhow::bail!(
                "Project must be under ~/hive-vault for broadcast (workers see /mnt/hive).\n\
                 Run: kova c2 sync\n\
                 Then: kova c2 run {} --project ~/hive-vault/projects/workspace/... --broadcast",
                token.name()
            );
        }

        let nodes: Vec<String> = if let Some(s) = nodes_override {
            s.split(',').map(|x| x.trim().to_string()).filter(|x| !x.is_empty()).collect()
        } else {
            let hosts = crate::inspect::run_inspect();
            hosts
                .iter()
                .filter(|h| h.id != "c2-core" && !h.unreachable)
                .map(|h| h.id.clone())
                .collect()
        };

        if nodes.is_empty() {
            anyhow::bail!("No reachable workers. Run: kova c2 inspect");
        }

        // Pre-flight: hive must be synced on workers
        let first = &nodes[0];
        let worker_path = to_worker_path(&plan.s4);
        let preflight = Command::new("ssh")
            .args(["-o", "ConnectTimeout=5", first])
            .arg(format!("test -d {}", worker_path.display()))
            .status();
        if let Ok(status) = preflight {
            if !status.success() {
                anyhow::bail!(
                    "Hive not synced on {}. Run: kova c2 sync",
                    first
                );
            }
        }

        let node_refs: Vec<&str> = nodes.iter().map(|s| s.as_str()).collect();
        run_broadcast(&plan, &node_refs)
    } else {
        run_local(&plan)
    }
}

pub fn run_nodes() {
    for n in default_nodes() {
        println!("{}", n);
    }
}

/// Workspace crates to sync (per KOVA_PROJECT_PLACEMENT).
const WORKSPACE_CRATES: &[&str] = &[
    "approuter", "cochranblock", "oakilydokily", "kova", "kova-core", "kova-web",
    "exopack", "whyyoulying", "wowasticker", "vendor",
];

fn kova_root() -> PathBuf {
    std::env::var("KOVA_ROOT")
        .ok()
        .map(PathBuf::from)
        .or_else(|| std::env::var("HOME").ok().map(PathBuf::from))
        .filter(|p| p.exists())
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Sync workspace from c2-core to workers. Replaces sync-hive.sh.
pub fn run_sync(dry_run: bool, target: &str) -> anyhow::Result<()> {
    let root = kova_root();
    let hive_workspace = "/mnt/hive/projects/workspace";
    let hive_projects = "/mnt/hive/projects";

    // 1. Ensure hive dir exists on target
    let check = Command::new("ssh")
        .args(["-o", "ConnectTimeout=5", target])
        .arg(format!("test -d {}", hive_workspace))
        .status();
    if let Ok(status) = check {
        if !status.success() {
            anyhow::bail!(
                "Hive not ready. Run on target:\n  ssh {} \"sudo mkdir -p {} {}/ronin-sites {}/rogue-repo && sudo chown -R $(whoami):$(whoami) {}\"",
                target, hive_workspace, hive_projects, hive_projects, hive_projects
            );
        }
    } else {
        anyhow::bail!("Cannot reach {}. Check SSH.", target);
    }

    let mut rsync_args = vec!["-avz", "--exclude", "target", "--exclude", "node_modules"];
    if dry_run {
        rsync_args.push("--dry-run");
    }

    // 2. Rsync workspace crates
    eprintln!("[sync] Syncing workspace to {}:{}/", target, hive_workspace);
    for crate_name in WORKSPACE_CRATES {
        let src = root.join(crate_name);
        if src.is_dir() {
            let status = Command::new("rsync")
                .args(&rsync_args)
                .arg(&src)
                .arg(format!("{}:{}/", target, hive_workspace))
                .status()?;
            if !status.success() {
                anyhow::bail!("rsync {} failed", crate_name);
            }
        }
    }
    let cargo_toml = root.join("Cargo.toml");
    if cargo_toml.is_file() {
        let status = Command::new("rsync")
            .args(&rsync_args)
            .arg(&cargo_toml)
            .arg(format!("{}:{}/", target, hive_workspace))
            .status()?;
        if !status.success() {
            anyhow::bail!("rsync Cargo.toml failed");
        }
    }

    // 3. Rsync ronin-sites, rogue-repo (outside workspace)
    eprintln!("[sync] Syncing ronin-sites, rogue-repo to {}:{}/", target, hive_projects);
    for dir in ["ronin-sites", "rogue-repo"] {
        let src = root.join(dir);
        if src.is_dir() {
            let status = Command::new("rsync")
                .args(&rsync_args)
                .arg(&src)
                .arg(format!("{}:{}/", target, hive_projects))
                .status()?;
            if !status.success() {
                anyhow::bail!("rsync {} failed", dir);
            }
        }
    }

    eprintln!("[sync] Done.");
    Ok(())
}
