// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Create test interfaces: spawn HTTP server, HTTP client with cookie store.


/// Bind to 127.0.0.1:0, return listener and base URL.
pub async fn bind_random() -> Result<(tokio::net::TcpListener, String), String> {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .map_err(|e| format!("bind failed: {}", e))?;
    let addr = listener.local_addr().map_err(|e| format!("local_addr: {}", e))?;
    let base = format!("http://{}", addr);
    Ok((listener, base))
}

/// HTTP test client: cookie store, no redirect follow.
pub fn http_client() -> Result<reqwest::Client, String> {
    reqwest::Client::builder()
        .cookie_store(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| e.to_string())
}
