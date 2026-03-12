// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! Restart subcommands: pkill + cargo build + exec.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use std::env;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

use approuter::setup;

fn pkill(pattern: &str) {
    let _ = Command::new("pkill")
        .args(["-f", pattern])
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status();
}

fn env_or(key: &str, default: &str) -> String {
    env::var(key).unwrap_or_else(|_| default.to_string())
}

fn env_opt(key: &str) -> Option<String> {
    env::var(key).ok().filter(|s| !s.is_empty())
}

pub fn f126() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = setup::cb_root();
    let port = env_or("ROUTER_PORT", "8080");

    let client = reqwest::blocking::Client::new();
    let _ = client
        .post(format!("http://127.0.0.1:{}/approuter/tunnel/stop", port))
        .timeout(Duration::from_secs(2))
        .send();

    pkill("approuter");
    pkill("router");
    thread::sleep(Duration::from_secs(1));

    let status = Command::new("cargo")
        .current_dir(&root)
        .args(["build", "--release", "-p", "approuter"])
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    let bin = root.join("target/release/approuter");
    let mut cmd = Command::new(&bin);
    cmd.env("ROUTER_PORT", &port)
        .env("ROUTER_NO_TUNNEL", "1")
        .env("ROUTER_CONFIG_DIR", &root)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if let Some(v) = env_opt("ROUTER_OAKILYDOKILY_HOST") {
        cmd.env("ROUTER_OAKILYDOKILY_HOST", v);
    }
    if let Some(v) = env_opt("ROUTER_ROGUEREPO_HOST") {
        cmd.env("ROUTER_ROGUEREPO_HOST", v);
    }
    if let Some(v) = env_opt("ROUTER_ROGUEREPO_URL") {
        cmd.env("ROUTER_ROGUEREPO_URL", v);
    }
    if let Some(v) = env_opt("ROUTER_RONIN_URL") {
        cmd.env("ROUTER_RONIN_URL", v);
    }
    if let Some(v) = env_opt("ROUTER_RONIN_SUFFIX") {
        cmd.env("ROUTER_RONIN_SUFFIX", v);
    }

    println!("Starting approuter (blocking)...");
    let status = cmd.status()?;
    std::process::exit(status.code().unwrap_or(1));
}

pub fn f127() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = setup::cb_root();
    pkill("oakilydokily");
    thread::sleep(Duration::from_secs(1));

    let status = Command::new("cargo")
        .current_dir(&root)
        .args(["build", "--release", "-p", "oakilydokily"])
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    let bin = root.join("target/release/oakilydokily");
    let od_dir = root.join("oakilydokily");
    let mut cmd = Command::new(&bin);
    cmd.current_dir(&od_dir)
        .stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if let Some(v) = env_opt("PORT") {
        cmd.env("PORT", v);
    }
    if let Some(v) = env_opt("BIND") {
        cmd.env("BIND", v);
    }
    if let Some(v) = env_opt("ROUTER") {
        cmd.env("ROUTER", v);
    }

    println!("Starting oakilydokily (blocking)...");
    let status = cmd.status()?;
    std::process::exit(status.code().unwrap_or(1));
}

pub fn f128() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = setup::cb_root();
    pkill("cochranblock");
    thread::sleep(Duration::from_secs(1));

    let status = Command::new("cargo")
        .current_dir(&root)
        .args(["build", "--release", "-p", "cochranblock"])
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    let bin = root.join("target/release/cochranblock");
    let mut cmd = Command::new(&bin);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if let Some(v) = env_opt("PORT") {
        cmd.env("PORT", v);
    }
    if let Some(v) = env_opt("BIND") {
        cmd.env("BIND", v);
    }
    if let Some(v) = env_opt("ROUTER") {
        cmd.env("ROUTER", v);
    }
    if let Some(v) = env_opt("CB_HOSTNAMES") {
        cmd.env("CB_HOSTNAMES", v);
    }
    if let Some(v) = env_opt("CB_BACKEND_URL") {
        cmd.env("CB_BACKEND_URL", v);
    }

    println!("Starting cochranblock (blocking)...");
    let status = cmd.status()?;
    std::process::exit(status.code().unwrap_or(1))
}

pub fn f129() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let ronin_root: PathBuf = env::var("RONIN_ROOT").map_err(|_| "RONIN_ROOT required")?.into();

    pkill("ronin-sites");
    thread::sleep(Duration::from_secs(1));

    let status = Command::new("cargo")
        .current_dir(&ronin_root)
        .args(["build", "--release", "--manifest-path", "rs/Cargo.toml"])
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    let bin = ronin_root.join("rs/target/release/ronin-sites");
    let mut cmd = Command::new(&bin);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if let Some(v) = env_opt("BIND_ADDR") {
        cmd.env("BIND_ADDR", v);
    }
    if let Some(v) = env_opt("STORAGE_PUBLIC_BASE") {
        cmd.env("STORAGE_PUBLIC_BASE", v);
    }
    if let Some(v) = env_opt("ROUTER") {
        cmd.env("ROUTER", v);
    }

    println!("Starting ronin-sites (blocking)...");
    let status = cmd.status()?;
    std::process::exit(status.code().unwrap_or(1));
}

pub fn f130() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let rr: PathBuf = env::var("ROGUE_REPO_ROOT").map_err(|_| "ROGUE_REPO_ROOT required")?.into();

    pkill("rogue-repo");
    thread::sleep(Duration::from_secs(1));

    let status = Command::new("cargo")
        .current_dir(&rr)
        .args(["build", "--release", "-p", "rogue-repo"])
        .status()?;
    if !status.success() {
        return Err("cargo build failed".into());
    }

    let bin = rr.join("target/release/rogue-repo");
    let mut cmd = Command::new(&bin);
    cmd.stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    if let Some(v) = env_opt("ROUTER") {
        cmd.env("ROUTER", v);
    }

    println!("Starting rogue-repo (blocking)...");
    let status = cmd.status()?;
    std::process::exit(status.code().unwrap_or(1));
}

pub fn f131(timeout_secs: u64) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let url = env_or("VERIFY_ORIGIN_URL", "https://127.0.0.1:443/");
    let client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true)
        .build()?;
    let interval = Duration::from_secs(3);
    let mut elapsed = Duration::ZERO;

    while elapsed.as_secs() < timeout_secs {
        if let Ok(r) = client.get(&url).timeout(Duration::from_secs(5)).send() {
            if r.status().as_u16() == 200 {
                println!("Origin OK (HTTP 200)");
                return Ok(());
            }
        }
        thread::sleep(interval);
        elapsed += interval;
    }
    Err(format!("Timeout: origin not responding after {}s", timeout_secs).into())
}
