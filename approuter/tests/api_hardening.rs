//! Integration tests for backlog #1-3: API key auth, hostname collision, /approuter/status.

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

fn spawn_approuter(port: u16, api_key: Option<&str>) -> Option<Child> {
    let bin = approuter_bin()?;
    let mut cmd = Command::new(&bin);
    cmd.env("ROUTER_PORT", port.to_string())
        .env("ROUTER_BIND", "127.0.0.1")
        .env("ROUTER_NO_TUNNEL", "true")
        .env_remove("CF_TOKEN")
        .env_remove("CF_ACCOUNT_ID")
        .stdout(Stdio::null())
        .stderr(Stdio::null());
    if let Some(key) = api_key {
        cmd.env("ROUTER_API_KEY", key);
    } else {
        cmd.env_remove("ROUTER_API_KEY");
    }
    cmd.spawn().ok()
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

// --- Backlog #1: API key auth ---

#[tokio::test]
async fn api_key_rejects_without_header() {
    let port = free_port();
    let mut child = match spawn_approuter(port, Some("test-secret")) {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await {
        let _ = child.kill();
        panic!("approuter did not start");
    }

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/register", port);
    let body = serde_json::json!({
        "app_id": "x", "hostnames": ["x.test"], "backend_url": "http://127.0.0.1:1"
    });

    // No auth header -> 401
    let res = client.post(&url).json(&body).timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 without auth header");

    // Wrong key -> 401
    let res = client.post(&url).json(&body)
        .header("Authorization", "Bearer wrong-key")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 401, "expected 401 with wrong key");

    // Correct key -> 200
    let res = client.post(&url).json(&body)
        .header("Authorization", "Bearer test-secret")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert!(res.status().is_success(), "expected 200 with correct key, got {}", res.status());

    let _ = child.kill();
    let _ = child.wait();
}

#[tokio::test]
async fn api_key_disabled_when_unset() {
    let port = free_port();
    let mut child = match spawn_approuter(port, None) {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await {
        let _ = child.kill();
        panic!("approuter did not start");
    }

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/register", port);
    let body = serde_json::json!({
        "app_id": "noauth", "hostnames": ["noauth.test"], "backend_url": "http://127.0.0.1:2"
    });

    // No ROUTER_API_KEY set -> no auth required -> 200
    let res = client.post(&url).json(&body).timeout(Duration::from_secs(5)).send().await.unwrap();
    assert!(res.status().is_success(), "expected 200 when API key is unset, got {}", res.status());

    let _ = child.kill();
    let _ = child.wait();
}

// --- Backlog #2: Hostname collision ---

#[tokio::test]
async fn hostname_collision_returns_409() {
    let port = free_port();
    let mut child = match spawn_approuter(port, None) {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await {
        let _ = child.kill();
        panic!("approuter did not start");
    }

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/register", port);

    // Register app-a with shared.example.com
    let res = client.post(&url)
        .json(&serde_json::json!({"app_id": "app-a", "hostnames": ["shared.example.com"], "backend_url": "http://127.0.0.1:3000"}))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert!(res.status().is_success(), "app-a registration failed");

    // Register app-b with same hostname -> 409
    let res = client.post(&url)
        .json(&serde_json::json!({"app_id": "app-b", "hostnames": ["shared.example.com"], "backend_url": "http://127.0.0.1:4000"}))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 409, "expected 409 Conflict for duplicate hostname");

    // Self-update app-a with same hostname -> 200
    let res = client.post(&url)
        .json(&serde_json::json!({"app_id": "app-a", "hostnames": ["shared.example.com", "new.example.com"], "backend_url": "http://127.0.0.1:3000"}))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert!(res.status().is_success(), "self-update should succeed, got {}", res.status());

    let _ = child.kill();
    let _ = child.wait();
}

// --- Backlog #3: /approuter/status ---

#[tokio::test]
async fn status_returns_all_products() {
    let port = free_port();
    let mut child = match spawn_approuter(port, None) {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await {
        let _ = child.kill();
        panic!("approuter did not start");
    }

    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/status", port);
    let res = client.get(&url).timeout(Duration::from_secs(10)).send().await.unwrap();
    assert!(res.status().is_success());

    let j: serde_json::Value = res.json().await.unwrap();

    // approuter field present
    assert_eq!(j["approuter"], "ok");

    // products array has the 4 legacy products
    let products = j["products"].as_array().expect("products is an array");
    let names: Vec<&str> = products.iter().filter_map(|p| p["product"].as_str()).collect();
    assert!(names.contains(&"cochranblock"), "missing cochranblock in {:?}", names);
    assert!(names.contains(&"oakilydokily"), "missing oakilydokily in {:?}", names);
    assert!(names.contains(&"rogue-repo"), "missing rogue-repo in {:?}", names);
    assert!(names.contains(&"ronin-sites"), "missing ronin-sites in {:?}", names);

    // Each product has required fields
    for p in products {
        assert!(p.get("backend").is_some(), "missing backend field");
        assert!(p.get("hostnames").is_some(), "missing hostnames field");
        assert!(p.get("healthy").is_some(), "missing healthy field");
        assert!(p.get("status_code").is_some(), "missing status_code field");
        assert!(p.get("latency_ms").is_some(), "missing latency_ms field");
    }

    // summary present
    assert!(j["summary"]["total"].as_u64().unwrap() >= 4);

    // Backends aren't running, so all should be unhealthy
    let unhealthy = j["summary"]["unhealthy"].as_u64().unwrap();
    assert_eq!(unhealthy, j["summary"]["total"].as_u64().unwrap(), "all backends should be unhealthy since none are running");

    let _ = child.kill();
    let _ = child.wait();
}
