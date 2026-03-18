//! Mock Cloudflare API with wiremock. Test registration flow when CF returns success.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::net::TcpListener;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use wiremock::matchers::{method, path_regex};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

async fn wait_for_server(port: u16, timeout_secs: u64) -> bool {
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/approuter/apps", port);
    for _ in 0..timeout_secs * 2 {
        if client.get(&url).timeout(Duration::from_millis(500)).send().await.is_ok() {
            return true;
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
    false
}

#[tokio::test]
async fn registration_flow_with_mocked_cloudflare() {
    let mock_server = MockServer::start().await;
    let mock_uri = mock_server.uri();

    // Mock GET /client/v4/zones (f94 extracts zone from hostname; path excludes query)
    Mock::given(method("GET"))
        .and(path_regex(r"^/client/v4/zones$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": [{
                "id": "zone123",
                "name": "example.com",
                "account": { "id": "acc123" }
            }]
        })))
        .mount(&mock_server)
        .await;

    // Mock GET /client/v4/zones/zone123/dns_records (for CNAME check)
    Mock::given(method("GET"))
        .and(path_regex(r"^/client/v4/zones/zone123/dns_records"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": []
        })))
        .mount(&mock_server)
        .await;

    // Mock POST /client/v4/zones/zone123/dns_records (create CNAME)
    Mock::given(method("POST"))
        .and(path_regex(r"^/client/v4/zones/zone123/dns_records$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": { "id": "rec123" }
        })))
        .mount(&mock_server)
        .await;

    // Mock PUT /client/v4/accounts/acc123/cfd_tunnel/.*/configurations
    Mock::given(method("PUT"))
        .and(path_regex(r"^/client/v4/accounts/[^/]+/cfd_tunnel/[^/]+/configurations$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": {}
        })))
        .mount(&mock_server)
        .await;

    // Mock GET /client/v4/accounts/acc123/cfd_tunnel/.*/token
    Mock::given(method("GET"))
        .and(path_regex(r"^/client/v4/accounts/[^/]+/cfd_tunnel/[^/]+/token$"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "result": { "token": "eyJ0ZXN0Ijp0cnVlfQ.mock-token" }
        })))
        .mount(&mock_server)
        .await;

    let port = free_port();
    let exe = std::env::current_exe().unwrap();
    let approuter_bin = exe
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("approuter");
    if !approuter_bin.exists() {
        eprintln!(
            "approuter binary not found at {} (run: cargo build -p approuter)",
            approuter_bin.display()
        );
        return;
    }

    let child = Command::new(&approuter_bin)
        .env("ROUTER_PORT", port.to_string())
        .env("ROUTER_BIND", "127.0.0.1")
        .env("ROUTER_NO_TUNNEL", "true")
        .env("CF_API_BASE_URL", &mock_uri)
        .env("CF_TOKEN", "test-token")
        .env("CF_ACCOUNT_ID", "acc123")
        .env("CF_TUNNEL_ID", "tunnel456")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    let mut child: Child = match child {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to spawn approuter: {}", e);
            return;
        }
    };

    if !wait_for_server(port, 10).await {
        let _ = child.kill();
        panic!("approuter did not start within 10s");
    }

    let url = format!("http://127.0.0.1:{}/approuter/register", port);
    let body = serde_json::json!({
        "app_id": "test-website",
        "hostnames": ["test.example.com"],
        "backend_url": "http://127.0.0.1:9999"
    });

    let client = reqwest::Client::new();
    let register_res = client
        .post(&url)
        .json(&body)
        .timeout(Duration::from_secs(5))
        .send()
        .await;

    let register_res = match register_res {
        Ok(r) => r,
        Err(e) => {
            let _ = child.kill();
            panic!("POST /approuter/register failed: {}", e);
        }
    };

    assert!(
        register_res.status().is_success(),
        "register failed: {}",
        register_res.text().await.unwrap_or_default()
    );

    let apps_url = format!("http://127.0.0.1:{}/approuter/apps", port);
    let apps_res = client
        .get(&apps_url)
        .timeout(Duration::from_secs(5))
        .send()
        .await
        .expect("GET /approuter/apps");

    assert!(apps_res.status().is_success());
    let apps: Vec<serde_json::Value> = apps_res.json().await.expect("parse apps json");
    let test_website = apps
        .iter()
        .find(|a| a.get("app_id").and_then(|v| v.as_str()) == Some("test-website"));

    let _ = child.kill();
    let _ = child.wait();

    assert!(
        test_website.is_some(),
        "test-website not found in apps: {:?}",
        apps
    );
}