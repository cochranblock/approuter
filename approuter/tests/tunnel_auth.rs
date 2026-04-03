//! Security tests for backlog #1 (P23): tunnel start/stop/ensure endpoints must require API key auth.
//! Verifies the three previously-unguarded write endpoints now enforce Bearer auth.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Sonnet 4.6

use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::time::Duration;

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

fn approuter_bin() -> Option<std::path::PathBuf> {
    let exe = std::env::current_exe().ok()?;
    let bin = exe.parent()?.parent()?.join("approuter");
    if bin.exists() { Some(bin) } else { None }
}

fn spawn_with_key(port: u16, key: &str) -> Option<Child> {
    let bin = approuter_bin()?;
    Command::new(&bin)
        .env("ROUTER_PORT", port.to_string())
        .env("ROUTER_BIND", "127.0.0.1")
        .env("ROUTER_NO_TUNNEL", "true")
        .env("ROUTER_API_KEY", key)
        .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID")
        .stdout(Stdio::null()).stderr(Stdio::null())
        .spawn().ok()
}

async fn wait_ready(port: u16) -> bool {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/health", port);
    for _ in 0..30 {
        if client.get(&url).timeout(Duration::from_millis(400)).send().await.is_ok() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(400)).await;
    }
    false
}

// ── POST /approuter/tunnels/:provider/start ───────────────────────────────────

#[tokio::test]
async fn tunnel_start_rejects_without_key() {
    let port = free_port();
    let mut child = match spawn_with_key(port, "s3cr3t") {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/tunnels/cloudflare/start", port);

    // No auth → 401
    let res = client.post(&url).timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 with no auth, got {}", res.status());

    // Wrong key → 401
    let res = client.post(&url)
        .header("Authorization", "Bearer wrong")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 with wrong key, got {}", res.status());

    let _ = child.kill(); let _ = child.wait();
}

#[tokio::test]
async fn tunnel_start_accepts_correct_key() {
    let port = free_port();
    let mut child = match spawn_with_key(port, "s3cr3t") {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    // Correct key — auth passes. The provider spawn itself will fail (binary not installed)
    // but that returns 500, not 401. Either way: not 401 = auth passed.
    let res = client.post(format!("http://127.0.0.1:{}/approuter/tunnels/cloudflare/start", port))
        .header("Authorization", "Bearer s3cr3t")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_ne!(res.status().as_u16(), 401, "correct key should not return 401, got {}", res.status());

    let _ = child.kill(); let _ = child.wait();
}

// ── POST /approuter/tunnels/:provider/stop ────────────────────────────────────

#[tokio::test]
async fn tunnel_stop_rejects_without_key() {
    let port = free_port();
    let mut child = match spawn_with_key(port, "s3cr3t") {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/tunnels/cloudflare/stop", port);

    let res = client.post(&url).timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 with no auth on stop, got {}", res.status());

    let res = client.post(&url)
        .header("Authorization", "Bearer wrong")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 with wrong key on stop, got {}", res.status());

    let _ = child.kill(); let _ = child.wait();
}

#[tokio::test]
async fn tunnel_stop_accepts_correct_key() {
    let port = free_port();
    let mut child = match spawn_with_key(port, "s3cr3t") {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    let res = client.post(format!("http://127.0.0.1:{}/approuter/tunnels/cloudflare/stop", port))
        .header("Authorization", "Bearer s3cr3t")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_ne!(res.status().as_u16(), 401, "correct key should not return 401 on stop, got {}", res.status());

    let _ = child.kill(); let _ = child.wait();
}

// ── POST /approuter/tunnel/ensure ────────────────────────────────────────────

#[tokio::test]
async fn tunnel_ensure_rejects_without_key() {
    let port = free_port();
    let mut child = match spawn_with_key(port, "s3cr3t") {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/tunnel/ensure", port);

    let res = client.post(&url).timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 with no auth on ensure, got {}", res.status());

    let res = client.post(&url)
        .header("Authorization", "Bearer wrong")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 with wrong key on ensure, got {}", res.status());

    let _ = child.kill(); let _ = child.wait();
}

#[tokio::test]
async fn tunnel_ensure_accepts_correct_key() {
    let port = free_port();
    let mut child = match spawn_with_key(port, "s3cr3t") {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    // Correct key — auth passes. Ensure may succeed or fail depending on env, but not 401.
    let res = client.post(format!("http://127.0.0.1:{}/approuter/tunnel/ensure", port))
        .header("Authorization", "Bearer s3cr3t")
        .timeout(Duration::from_secs(10)).send().await.unwrap();
    assert_ne!(res.status().as_u16(), 401, "correct key should not return 401 on ensure, got {}", res.status());

    let _ = child.kill(); let _ = child.wait();
}

// ── Auth disabled when ROUTER_API_KEY unset ───────────────────────────────────

#[tokio::test]
async fn tunnel_endpoints_open_when_no_key_configured() {
    let port = free_port();
    let bin = match approuter_bin() {
        Some(b) => b,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    let mut child: Child = Command::new(&bin)
        .env("ROUTER_PORT", port.to_string())
        .env("ROUTER_BIND", "127.0.0.1")
        .env("ROUTER_NO_TUNNEL", "true")
        .env_remove("ROUTER_API_KEY")
        .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID")
        .stdout(Stdio::null()).stderr(Stdio::null())
        .spawn().expect("spawn");
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();

    // Without ROUTER_API_KEY set, endpoints are open (auth disabled)
    let res = client.post(format!("http://127.0.0.1:{}/approuter/tunnels/cloudflare/stop", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_ne!(res.status().as_u16(), 401, "expected open (no key set), got {}", res.status());

    let res = client.post(format!("http://127.0.0.1:{}/approuter/tunnel/ensure", port))
        .timeout(Duration::from_secs(10)).send().await.unwrap();
    assert_ne!(res.status().as_u16(), 401, "expected open (no key set) on ensure, got {}", res.status());

    let _ = child.kill(); let _ = child.wait();
}
