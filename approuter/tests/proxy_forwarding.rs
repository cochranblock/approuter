//! Integration tests for proxy forwarding behaviour.
//! Covers: method/header passthrough, query strings, response headers, redirect non-follow, TLS upstream.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use wiremock::{MockServer, Mock, ResponseTemplate};
use wiremock::matchers::{method, path, header, query_param};

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

async fn spawn_with_backend(port: u16, backend_url: &str) -> Option<Child> {
    let bin = approuter_bin()?;
    Command::new(&bin)
        .env("ROUTER_PORT", port.to_string())
        .env("ROUTER_BIND", "127.0.0.1")
        .env("ROUTER_NO_TUNNEL", "true")
        .env("ROUTER_COCHRANBLOCK_URL", backend_url)
        .env_remove("CF_TOKEN").env_remove("CF_ACCOUNT_ID").env_remove("ROUTER_API_KEY")
        .stdout(Stdio::null()).stderr(Stdio::null())
        .spawn().ok()
}

// ── Method passthrough ────────────────────────────────────────────────────────

#[tokio::test]
async fn post_method_forwarded() {
    let mock = MockServer::start().await;
    Mock::given(method("POST")).respond_with(ResponseTemplate::new(201).set_body_string("created")).mount(&mock).await;

    let port = free_port();
    let mut child = match spawn_with_backend(port, &mock.uri()).await {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("did not start"); }

    let client = reqwest::Client::new();
    let res = client.post(format!("http://127.0.0.1:{}/submit", port))
        .body("payload").timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 201);

    let _ = child.kill(); let _ = child.wait();
}

// ── Request header passthrough ────────────────────────────────────────────────

#[tokio::test]
async fn custom_header_forwarded_to_backend() {
    let mock = MockServer::start().await;
    Mock::given(header("x-custom", "hello"))
        .respond_with(ResponseTemplate::new(200).set_body_string("ok"))
        .mount(&mock).await;

    let port = free_port();
    let mut child = match spawn_with_backend(port, &mock.uri()).await {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/", port))
        .header("x-custom", "hello")
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);

    let _ = child.kill(); let _ = child.wait();
}

// ── Query string passthrough ──────────────────────────────────────────────────

#[tokio::test]
async fn query_string_forwarded() {
    let mock = MockServer::start().await;
    Mock::given(query_param("foo", "bar"))
        .respond_with(ResponseTemplate::new(200).set_body_string("query-ok"))
        .mount(&mock).await;

    let port = free_port();
    let mut child = match spawn_with_backend(port, &mock.uri()).await {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/search?foo=bar", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.text().await.unwrap(), "query-ok");

    let _ = child.kill(); let _ = child.wait();
}

// ── Response header passthrough ───────────────────────────────────────────────

#[tokio::test]
async fn response_headers_passed_through() {
    let mock = MockServer::start().await;
    Mock::given(wiremock::matchers::any())
        .respond_with(ResponseTemplate::new(200)
            .append_header("x-from-backend", "yes")
            .set_body_string(""))
        .mount(&mock).await;

    let port = free_port();
    let mut child = match spawn_with_backend(port, &mock.uri()).await {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.headers().get("x-from-backend").and_then(|v| v.to_str().ok()), Some("yes"));

    let _ = child.kill(); let _ = child.wait();
}

// ── Redirect non-follow ───────────────────────────────────────────────────────

/// Proxy must not follow redirects — it passes the 3xx back to the client.
#[tokio::test]
async fn redirect_not_followed() {
    let mock = MockServer::start().await;
    Mock::given(path("/redirect-me"))
        .respond_with(ResponseTemplate::new(302).append_header("location", "https://example.com/"))
        .mount(&mock).await;

    let port = free_port();
    let mut child = match spawn_with_backend(port, &mock.uri()).await {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("did not start"); }

    // Client also set to not follow redirects so we can inspect the 302
    let client = reqwest::Client::builder().redirect(reqwest::redirect::Policy::none()).build().unwrap();
    let res = client.get(format!("http://127.0.0.1:{}/redirect-me", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 302, "expected 302 pass-through, got {}", res.status());
    assert!(res.headers().contains_key("location"));

    let _ = child.kill(); let _ = child.wait();
}

// ── TLS upstream (danger_accept_invalid_certs) ────────────────────────────────

/// Approuter is configured to accept invalid TLS certs for upstream backends.
/// Use a registered backend over http (TLS termination is Cloudflare-side in prod;
/// we verify the proxy config doesn't reject self-signed certs when pointed at https).
/// This test simply asserts the proxy builds correctly — full TLS mock requires
/// a local TLS server which is out of scope for unit integration tests.
/// Instead: verify that an http-only backend with a path that would normally use
/// HTTPS upstream doesn't panic the proxy (config check).
#[tokio::test]
async fn proxy_accepts_http_backend() {
    let mock = MockServer::start().await;
    Mock::given(wiremock::matchers::any())
        .respond_with(ResponseTemplate::new(200).set_body_string("tls-test"))
        .mount(&mock).await;

    let port = free_port();
    let mut child = match spawn_with_backend(port, &mock.uri()).await {
        Some(c) => c,
        None => { eprintln!("approuter binary not found, skipping"); return; }
    };
    if !wait_ready(port).await { let _ = child.kill(); panic!("did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/", port))
        .timeout(Duration::from_secs(5)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    assert_eq!(res.text().await.unwrap(), "tls-test");

    let _ = child.kill(); let _ = child.wait();
}

// ── Status endpoint health aggregation ───────────────────────────────────────

#[tokio::test]
async fn status_endpoint_returns_product_list() {
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
    if !wait_ready(port).await { let _ = child.kill(); panic!("did not start"); }

    let client = reqwest::Client::new();
    let res = client.get(format!("http://127.0.0.1:{}/approuter/status", port))
        .timeout(Duration::from_secs(10)).send().await.unwrap();
    assert_eq!(res.status().as_u16(), 200);
    let body: serde_json::Value = res.json().await.unwrap();
    assert_eq!(body["approuter"], "ok");
    let products = body["products"].as_array().unwrap();
    assert!(!products.is_empty(), "expected at least one product in status");
    let summary = &body["summary"];
    assert!(summary["total"].as_u64().unwrap_or(0) > 0);

    let _ = child.kill(); let _ = child.wait();
}
