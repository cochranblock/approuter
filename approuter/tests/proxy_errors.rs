//! Integration tests for backlog #6: proxy error handling.
//! Verify upstream unreachable returns 502, analytics recorded.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

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

async fn wait_ready(port: u16) -> bool {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/health", port);
    for _ in 0..20 {
        if client.get(&url).timeout(Duration::from_millis(500)).send().await.is_ok() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    false
}

#[tokio::test]
async fn unreachable_backend_returns_502() {
    let port = free_port();
    let dead_port = free_port(); // nothing listening here
    let bin = match approuter_bin() {
        Some(b) => b,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };

    // Point cochranblock (default backend) at a dead port
    let mut child: Child = Command::new(&bin)
        .env("ROUTER_PORT", port.to_string())
        .env("ROUTER_BIND", "127.0.0.1")
        .env("ROUTER_NO_TUNNEL", "true")
        .env("ROUTER_COCHRANBLOCK_URL", format!("http://127.0.0.1:{}", dead_port))
        .env_remove("CF_TOKEN")
        .env_remove("CF_ACCOUNT_ID")
        .env_remove("ROUTER_API_KEY")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .expect("spawn approuter");

    if !wait_ready(port).await {
        let _ = child.kill();
        panic!("approuter did not start");
    }

    let client = reqwest::Client::new();

    // Request to default backend (cochranblock) which is unreachable -> 502
    let res = client
        .get(format!("http://127.0.0.1:{}/", port))
        .timeout(Duration::from_secs(10))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status().as_u16(), 502, "expected 502 for unreachable backend");

    // Verify analytics recorded the 502
    let analytics = client
        .get(format!("http://127.0.0.1:{}/approuter/analytics/recent?limit=5", port))
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .unwrap();
    assert!(analytics.status().is_success());
    let events: Vec<serde_json::Value> = analytics.json().await.unwrap();
    let has_502 = events.iter().any(|e| e["status"].as_u64() == Some(502));
    assert!(has_502, "analytics should contain a 502 event, got: {:?}", events);

    let _ = child.kill();
    let _ = child.wait();
}
