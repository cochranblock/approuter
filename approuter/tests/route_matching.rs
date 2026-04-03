//! Unit tests for route matching (f56/f57) and health check endpoint.
//! Covers: host-based routing, path-prefix routing, wildcard hostnames, /health.

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
    for _ in 0..30 {
        if client.get(&url).timeout(Duration::from_millis(400)).send().await.is_ok() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(400)).await;
    }
    false
}

// ── Health check endpoint ─────────────────────────────────────────────────────

#[tokio::test]
async fn health_returns_ok() {
    let port = free_port();
    let mut child: Child = match approuter_bin() {
        Some(bin) => Command::new(&bin)
            .env("ROUTER_PORT", port.to_string())
            .env("ROUTER_BIND", "127.0.0.1")
            .env("ROUTER_NO_TUNNEL", "true")
            .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID").env_remove("ROUTER_API_KEY")
            .stdout(Stdio::null()).stderr(Stdio::null())
            .spawn().expect("spawn"),
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/health", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["status"], "ok");
    assert_eq!(body["service"], "approuter");

    // Also via /approuter/health alias
    let res2 = client.get(format!("http://127.0.0.1:{}/approuter/health", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res2.status().as_u16(), 200);

    let _ = child.kill(); let _ = child.wait();
}

// ── Host-based routing ────────────────────────────────────────────────────────

/// Request with a host that matches a registered app routes to that app's backend.
/// We register a mock backend (wiremock) and verify the request arrives there.
#[tokio::test]
async fn registered_host_routes_to_backend() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::any;

    let mock = MockServer::start().await;
    Mock::given(any()).respond_with(ResponseTemplate::new(200).set_body_string("from-backend")).mount(&mock).await;

    let port = free_port();
    let mut child: Child = match approuter_bin() {
        Some(bin) => Command::new(&bin)
            .env("ROUTER_PORT", port.to_string())
            .env("ROUTER_BIND", "127.0.0.1")
            .env("ROUTER_NO_TUNNEL", "true")
            .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID").env_remove("ROUTER_API_KEY")
            .stdout(Stdio::null()).stderr(Stdio::null())
            .spawn().expect("spawn"),
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();

    // Register app pointing at mock backend
    let reg = client.post(format!("http://127.0.0.1:{}/approuter/register", port))
        .json(&serde_json::json!({
            "app_id": "mock-app",
            "hostnames": ["mock.local"],
            "backend_url": mock.uri()
        }))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert!(reg.status().is_success(), "register failed: {}", reg.status());

    // Request with Host: mock.local — should hit the mock backend
    let res = client.get(format!("http://127.0.0.1:{}/", port))
        .header("Host", "mock.local")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body = res.text().await.unwrap();
    assert_eq!(body, "from-backend");

    let _ = child.kill(); let _ = child.wait();
}

/// Request with an unregistered host falls through to default cochranblock backend.
#[tokio::test]
async fn unknown_host_routes_to_default_backend() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::any;

    let default_mock = MockServer::start().await;
    Mock::given(any()).respond_with(ResponseTemplate::new(200).set_body_string("default")).mount(&default_mock).await;

    let port = free_port();
    let mut child: Child = match approuter_bin() {
        Some(bin) => Command::new(&bin)
            .env("ROUTER_PORT", port.to_string())
            .env("ROUTER_BIND", "127.0.0.1")
            .env("ROUTER_NO_TUNNEL", "true")
            .env("ROUTER_COCHRANBLOCK_URL", default_mock.uri())
            .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID").env_remove("ROUTER_API_KEY")
            .stdout(Stdio::null()).stderr(Stdio::null())
            .spawn().expect("spawn"),
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/some/path", port))
        .header("Host", "no-match.example.com")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.text().await.unwrap(), "default");

    let _ = child.kill(); let _ = child.wait();
}

// ── Wildcard hostname routing ─────────────────────────────────────────────────

#[tokio::test]
async fn wildcard_host_routes_correctly() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::any;

    let mock = MockServer::start().await;
    Mock::given(any()).respond_with(ResponseTemplate::new(200).set_body_string("wildcard-backend")).mount(&mock).await;

    let port = free_port();
    let mut child: Child = match approuter_bin() {
        Some(bin) => Command::new(&bin)
            .env("ROUTER_PORT", port.to_string())
            .env("ROUTER_BIND", "127.0.0.1")
            .env("ROUTER_NO_TUNNEL", "true")
            .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID").env_remove("ROUTER_API_KEY")
            .stdout(Stdio::null()).stderr(Stdio::null())
            .spawn().expect("spawn"),
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    client.post(format!("http://127.0.0.1:{}/approuter/register", port))
        .json(&serde_json::json!({
            "app_id": "wildcard-app",
            "hostnames": ["*.wildcard.test"],
            "backend_url": mock.uri()
        }))
        .timeout(Duration::from_secs(5)).send().await.unwrap();

    // sub.wildcard.test should match *.wildcard.test
    let res = client.get(format!("http://127.0.0.1:{}/", port))
        .header("Host", "sub.wildcard.test")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.text().await.unwrap(), "wildcard-backend");

    let _ = child.kill(); let _ = child.wait();
}

// ── Path-prefix routing (ROUTER_OAKILYDOKILY_PATH) ────────────────────────────

#[tokio::test]
async fn path_prefix_routes_to_correct_backend() {
    use wiremock::{MockServer, Mock, ResponseTemplate};
    use wiremock::matchers::any;

    let oak_mock = MockServer::start().await;
    Mock::given(any()).respond_with(ResponseTemplate::new(200).set_body_string("oakilydokily")).mount(&oak_mock).await;

    let port = free_port();
    let mut child: Child = match approuter_bin() {
        Some(bin) => Command::new(&bin)
            .env("ROUTER_PORT", port.to_string())
            .env("ROUTER_BIND", "127.0.0.1")
            .env("ROUTER_NO_TUNNEL", "true")
            .env("ROUTER_OAKILYDOKILY_URL", oak_mock.uri())
            .env("ROUTER_OAKILYDOKILY_PATH", "oak")
            .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID").env_remove("ROUTER_API_KEY")
            .stdout(Stdio::null()).stderr(Stdio::null())
            .spawn().expect("spawn"),
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("approuter did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/oak/page", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.text().await.unwrap(), "oakilydokily");

    let _ = child.kill(); let _ = child.wait();
}
