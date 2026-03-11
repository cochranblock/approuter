// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! run::start_all — Native Rust replacement for start-sites-cloudflare.sh.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::thread;
use std::time::Duration;

use crate::cloudflare;
use approuter::setup;

/// Load KEY=VALUE from .env into env. Overrides existing vars.
fn load_env_into_process(path: &std::path::Path) {
    let Ok(content) = fs::read_to_string(path) else { return };
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let k = k.trim();
            let v = v.trim().trim_matches('"');
            if !k.is_empty() && !v.is_empty() {
                let _ = env::set_var(k, v);
            }
        }
    }
}

/// CF vars we own — approuter/.env only. Other project .env must not overwrite these.
const CF_KEYS: &[&str] = &["CF_TOKEN", "CLOUDFLARE_API_TOKEN", "CF_ACCOUNT_ID", "CLOUDFLARE_ACCOUNT_ID", "CF_TUNNEL_ID", "TUNNEL_TOKEN"];

/// Load only CF_* vars from path. Use for tunnel step so approuter/.env wins over ronin etc.
fn load_cf_env_from(path: &std::path::Path) {
    let Ok(content) = fs::read_to_string(path) else { return };
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            let k = k.trim();
            if CF_KEYS.iter().any(|&cf| cf == k) {
                let v = v.trim().trim_matches('"');
                if !v.is_empty() {
                    let _ = env::set_var(k, v);
                }
            }
        }
    }
}

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

/// Root for cochranblock/oakilydokily. COCHRANBLOCK_ROOT or current dir.
fn cb_root() -> PathBuf {
    env::var("COCHRANBLOCK_ROOT")
        .map(PathBuf::from)
        .unwrap_or_else(|_| setup::cb_root())
}

/// RONIN_ROOT with fallback to ronin-sites/ or ../ronin-sites relative to cb_root.
fn ronin_root() -> Option<PathBuf> {
    env::var("RONIN_ROOT")
        .map(PathBuf::from)
        .ok()
        .filter(|p| p.exists())
        .or_else(|| {
            let root = cb_root();
            for p in [
                root.join("ronin-sites"),
                root.parent().unwrap_or(&root).join("ronin-sites"),
            ] {
                if p.exists() {
                    return Some(p);
                }
            }
            None
        })
}

/// ROGUE_REPO_ROOT with fallback to rogue-repo/ or ../rogue-repo relative to cb_root.
fn rogue_repo_root() -> Option<PathBuf> {
    env::var("ROGUE_REPO_ROOT")
        .map(PathBuf::from)
        .ok()
        .filter(|p| p.exists())
        .or_else(|| {
            let root = cb_root();
            for p in [root.join("rogue-repo"), root.parent().unwrap_or(&root).join("rogue-repo")] {
                if p.exists() {
                    return Some(p);
                }
            }
            None
        })
}

fn spawn_detached(mut cmd: Command) -> Result<Child, Box<dyn std::error::Error + Send + Sync>> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    Ok(cmd.spawn()?)
}

/// Open URL in browser. Uses `open` on macOS, `xdg-open` on Linux.
fn open_browser(url: &str) {
    #[cfg(target_os = "macos")]
    let _ = Command::new("open").arg(url).output();
    #[cfg(not(target_os = "macos"))]
    let _ = Command::new("xdg-open").arg(url).output();
}

/// Workspace root when binary is at workspace/target/release/approuter (or debug).
fn workspace_root_from_exe() -> Option<PathBuf> {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.canonicalize().ok())
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))  // target/release -> target
        .and_then(|p| p.parent().map(|p| p.to_path_buf()))  // target -> workspace
}

/// cf_token_check — Evaluate each token's capabilities (verify, tunnel token).
pub fn cf_token_check(root: &PathBuf) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    load_cf_env_from(&root.join("approuter").join(".env"));
    let account_id = env_opt("CF_ACCOUNT_ID").or_else(|| env_opt("CLOUDFLARE_ACCOUNT_ID"))
        .unwrap_or_else(|| "aabaf34b42d0d042e3e570903b117b08".into());
    let tunnel_id = env_opt("CF_TUNNEL_ID").unwrap_or_else(|| "b12525df-6971-4c47-9a0d-61ee57a5cbd5".into());

    let approuter_token = env_opt("CF_TOKEN").or_else(|| env_opt("CLOUDFLARE_API_TOKEN"));
    let ronin_env = ronin_root().map(|r| r.join(".env"));
    let ronin_token = ronin_env.as_ref()
        .filter(|p| p.exists())
        .and_then(|p| {
            let content = fs::read_to_string(p).ok()?;
            for line in content.lines() {
                let line = line.trim();
                if let Some((k, v)) = line.split_once('=') {
                    if k.trim() == "CF_TOKEN" || k.trim() == "CLOUDFLARE_API_TOKEN" {
                        let v = v.trim().trim_matches('"');
                        if !v.is_empty() {
                            return Some(v.to_string());
                        }
                    }
                }
            }
            None
        });

    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build()?;
    println!("{:-<60}", "");
    println!("{:^60}", "Cloudflare token capabilities");
    println!("{:-<60}", "");
    println!("{:<20} {:<12} {:<12} {:<16}", "Source", "Verify", "Tunnel API", "Token ID (masked)");
    println!("{:-<60}", "");

    for (name, token_opt) in [
        ("approuter/.env", approuter_token.as_deref()),
        ("ronin-sites/.env", ronin_token.as_deref()),
    ] {
        let Some(token) = token_opt else {
            println!("{:<20} {:<12} {:<12} {:<16}", name, "—", "—", "(not set)");
            continue;
        };
        let verify = rt.block_on(cloudflare::verify_token(token));
        let verify_ok = verify.as_ref().map(|v| v.ok && v.status == "active").unwrap_or(false);
        let verify_str = if let Ok(v) = &verify {
            format!("{} ({})", if verify_ok { "✓" } else { "✗" }, v.status)
        } else {
            "✗ (error)".into()
        };
        let tunnel = rt.block_on(cloudflare::can_get_tunnel_token(token, &account_id, &tunnel_id));
        let tunnel_str = match &tunnel {
            Ok(true) => "✓",
            Ok(false) => "✗ 403/401",
            Err(_) => "✗ error",
        };
        let id = verify.as_ref().map(|v| v.id.as_str()).unwrap_or("—");
        let id_short = if id.len() > 12 { format!("{}...", &id[..8]) } else { id.into() };
        println!("{:<20} {:<12} {:<12} {:<16}", name, verify_str, tunnel_str, id_short);
    }
    println!("{:-<60}", "");
    println!("Tunnel API = GET /accounts/.../cfd_tunnel/.../token (Cloudflare Tunnel Write)");
    Ok(())
}

/// start_all — pkill, spawn approuter (--no-tunnel), cochranblock, oakilydokily, rogue-repo, ronin-sites,
/// get tunnel token, spawn cloudflared. Optionally open browser.
pub fn start_all(open_browser_flag: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let root = cb_root();
    let env_path = root.join("approuter").join(".env");
    let to_load = if env_path.exists() { env_path } else { root.join(".env") };
    if let Err(e) = dotenvy::from_path_override(&to_load) {
        eprintln!("Could not load .env from {}: {}", to_load.display(), e);
    }
    let port = env_or("ROUTER_PORT", "8080");
    let approuter_url = env_or("APPROUTER_URL", &format!("http://127.0.0.1:{}", port));

    // 1. pkill existing processes
    pkill("cloudflared");
    pkill("approuter");
    pkill("router");
    pkill("cochranblock");
    pkill("oakilydokily");
    pkill("rogue-repo");
    pkill("ronin-sites");
    thread::sleep(Duration::from_secs(1));

    // 2. Spawn approuter with --no-tunnel
    let approuter_bin = std::env::current_exe()
        .unwrap_or_else(|_| root.join("target/release/approuter"));
    let mut approuter_cmd = Command::new(&approuter_bin);
    approuter_cmd
        .arg("--no-tunnel")
        .env("ROUTER_PORT", &port)
        .env("ROUTER_NO_TUNNEL", "true")
        .env("ROUTER_CONFIG_DIR", &root);
    if let Some(v) = env_opt("ROUTER_OAKILYDOKILY_HOST") {
        approuter_cmd.env("ROUTER_OAKILYDOKILY_HOST", v);
    }
    if let Some(v) = env_opt("ROUTER_ROGUEREPO_HOST") {
        approuter_cmd.env("ROUTER_ROGUEREPO_HOST", v);
    }
    if let Some(v) = env_opt("ROUTER_ROGUEREPO_URL") {
        approuter_cmd.env("ROUTER_ROGUEREPO_URL", v);
    }
    if let Some(v) = env_opt("ROUTER_RONIN_URL") {
        approuter_cmd.env("ROUTER_RONIN_URL", v);
    }
    if let Some(v) = env_opt("ROUTER_RONIN_SUFFIX") {
        approuter_cmd.env("ROUTER_RONIN_SUFFIX", v);
    }
    spawn_detached(approuter_cmd)?;
    println!("Spawned approuter on port {}", port);
    thread::sleep(Duration::from_secs(2));

    // 3. Spawn cochranblock
    let cb_bin = root.join("target/release/cochranblock");
    if cb_bin.exists() {
        let mut cmd = Command::new(&cb_bin);
        cmd.env("APPROUTER_URL", &approuter_url)
            .env("ROUTER", &approuter_url);
        if let Some(v) = env_opt("PORT") {
            cmd.env("PORT", v);
        }
        if let Some(v) = env_opt("BIND") {
            cmd.env("BIND", v);
        }
        if let Some(v) = env_opt("CB_HOSTNAMES") {
            cmd.env("CB_HOSTNAMES", v);
        }
        if let Some(v) = env_opt("CB_BACKEND_URL") {
            cmd.env("CB_BACKEND_URL", v);
        }
        spawn_detached(cmd)?;
        println!("Spawned cochranblock");
    } else {
        println!("cochranblock binary not found at {} (run cargo build -p cochranblock)", cb_bin.display());
    }

    // 4. Spawn oakilydokily
    let od_bin = root.join("target/release/oakilydokily");
    let od_dir = root.join("oakilydokily");
    if od_bin.exists() {
        let mut cmd = Command::new(&od_bin);
        cmd.current_dir(&od_dir)
            .env("APPROUTER_URL", &approuter_url)
            .env("ROUTER", &approuter_url);
        if let Some(v) = env_opt("PORT") {
            cmd.env("PORT", v);
        }
        if let Some(v) = env_opt("BIND") {
            cmd.env("BIND", v);
        }
        spawn_detached(cmd)?;
        println!("Spawned oakilydokily");
    } else {
        println!("oakilydokily binary not found at {}", od_bin.display());
    }

    // 5. Spawn rogue-repo
    if let Some(rr) = rogue_repo_root() {
        load_env_into_process(&rr.join(".env"));
        let rr_bin = rr.join("target/release/rogue-repo");
        if rr_bin.exists() {
            let mut cmd = Command::new(&rr_bin);
            cmd.current_dir(&rr)
                .env("APPROUTER_URL", &approuter_url)
                .env("ROUTER", &approuter_url);
            if let Some(v) = env_opt("DATABASE_URL") {
                cmd.env("DATABASE_URL", v);
            }
            spawn_detached(cmd)?;
            println!("Spawned rogue-repo");
        } else {
            println!("rogue-repo binary not found at {}", rr_bin.display());
        }
    } else {
        println!("ROGUE_REPO_ROOT not set and rogue-repo not found in fallback paths");
    }

    // 6. Spawn ronin-sites
    if let Some(ronin) = ronin_root() {
        load_env_into_process(&ronin.join(".env"));
        let ronin_bin = ronin.join("rs/target/release/ronin-sites");
        if ronin_bin.exists() {
            let mut cmd = Command::new(&ronin_bin);
            cmd.current_dir(&ronin)
                .env("APPROUTER_URL", &approuter_url)
                .env("ROUTER", &approuter_url);
            if env_opt("SK").or_else(|| env_opt("SECRET_KEY")).is_none() {
                cmd.env("SK", "dev-secret-key-not-for-production");
                eprintln!("ronin-sites: SK/SECRET_KEY not set, using dev fallback");
            }
            if let Some(v) = env_opt("BIND_ADDR") {
                cmd.env("BIND_ADDR", v);
            }
            if let Some(v) = env_opt("STORAGE_PUBLIC_BASE") {
                cmd.env("STORAGE_PUBLIC_BASE", v);
            }
            spawn_detached(cmd)?;
            println!("Spawned ronin-sites");
            thread::sleep(Duration::from_secs(5));
        } else {
            println!("ronin-sites binary not found at {}", ronin_bin.display());
        }
    } else {
        println!("RONIN_ROOT not set and ronin not found in fallback paths");
    }

    // 7. Get tunnel token and spawn cloudflared
    // CF vars come from approuter/.env only (not ronin etc). That token is the one used.
    let approuter_env = root.join("approuter").join(".env");
    load_cf_env_from(&approuter_env);
    if let Some(ws) = workspace_root_from_exe() {
        let p = ws.join("approuter").join(".env");
        if p.exists() && p != approuter_env {
            load_cf_env_from(&p);
        }
    }
    // Prefer API (CF_TOKEN + CF_ACCOUNT_ID) when configured; fallback to TUNNEL_TOKEN
    let api_ready = env_opt("CF_TOKEN").or_else(|| env_opt("CLOUDFLARE_API_TOKEN")).is_some()
        && env_opt("CF_ACCOUNT_ID").or_else(|| env_opt("CLOUDFLARE_ACCOUNT_ID")).is_some();
    let token = api_ready
        .then(|| {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_all()
                .build()
                .ok()?;
            rt.block_on(crate::cloudflare::get_tunnel_token()).ok()
        })
        .flatten()
        .map(|t| (t, "Cloudflare API (approuter/.env)"))
        .or_else(|| env_opt("TUNNEL_TOKEN").map(|t| (t, "TUNNEL_TOKEN (approuter/.env)")));
    match token {
    Some((token, source)) => {
        println!("Tunnel token from {}", source);
        let cloudflared = if cfg!(target_os = "windows") {
            "cloudflared.exe"
        } else {
            "cloudflared"
        };
        let mut cmd = Command::new(cloudflared);
        cmd.args(["tunnel", "run", "--token", &token]);
        if let Ok(c) = spawn_detached(cmd) {
            println!("Spawned cloudflared with tunnel token");
            drop(c);
        } else {
            println!("cloudflared not found or failed to spawn (install from https://developers.cloudflare.com/cloudflare-one/connections/connect-apps/install-and-setup/installation/)");
        }
    }
    None => {
        eprintln!("Could not get tunnel token. Set CF_TOKEN+CF_ACCOUNT_ID in approuter/.env (API path), or TUNNEL_TOKEN (eyJ... from dashboard). Skipping cloudflared.");
    }
    }

    // 8. Optionally open browser to sites
    if open_browser_flag {
        thread::sleep(Duration::from_secs(1));
        for url in [
            "https://cochranblock.org",
            "https://oakilydokily.com",
            "https://roguerepo.io",
            "https://ronin-sites.pro",
        ] {
            open_browser(url);
            thread::sleep(Duration::from_millis(300));
        }
    }

    println!("start-all complete. Approuter at {}", approuter_url);
    Ok(())
}
