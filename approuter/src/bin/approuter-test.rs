// Unlicense — cochranblock.org
#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! f94=approuter_test. TRIPLE SIMS smoke. exopack only in -test binary.

use std::path::PathBuf;
use std::process::Command;
use std::time::Duration;

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();
    let ok = run_smoke().await;
    std::process::exit(if ok { 0 } else { 1 });
}

async fn run_smoke() -> bool {
    let client = match exopack::interface::http_client() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("exopack http_client: {}", e);
            return false;
        }
    };

    let exe = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("approuter-test"));
    let approuter_bin = exe.parent().unwrap().join("approuter");
    if !approuter_bin.exists() {
        eprintln!("approuter binary not found at {} (build with: cargo build -p approuter)", approuter_bin.display());
        return false;
    }

    let port = 19222u16;
    let child = Command::new(&approuter_bin)
        .env("ROUTER_PORT", port.to_string())
        .env("ROUTER_BIND", "127.0.0.1")
        .env("ROUTER_NO_TUNNEL", "1")
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .spawn();

    let mut child = match child {
        Ok(c) => c,
        Err(e) => {
            eprintln!("approuter spawn: {}", e);
            return false;
        }
    };

    tokio::time::sleep(Duration::from_secs(2)).await;

    let url = format!("http://127.0.0.1:{}/approuter/apps", port);
    let ok = match client.get(&url).timeout(Duration::from_secs(5)).send().await {
        Ok(r) => r.status().is_success(),
        Err(e) => {
            eprintln!("GET /approuter/apps: {}", e);
            false
        }
    };

    let _ = child.kill();
    let _ = child.wait();
    ok
}
