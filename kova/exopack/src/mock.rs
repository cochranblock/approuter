// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Mock APIs on demand. wiremock wrapper.

use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

/// Start a mock HTTP server on a random port. Returns (server, base_uri).
pub async fn start_mock_server() -> (MockServer, String) {
    let server = MockServer::start().await;
    let uri = server.uri();
    (server, uri)
}

/// Mount a simple GET handler returning 200 + body.
pub async fn mount_get(server: &MockServer, path_pattern: &str, body: &str) {
    Mock::given(method("GET"))
        .and(path(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_string(body.to_string()))
        .mount(server)
        .await;
}

/// Mount a simple GET handler returning JSON.
pub async fn mount_get_json(server: &MockServer, path_pattern: &str, json: serde_json::Value) {
    Mock::given(method("GET"))
        .and(path(path_pattern))
        .respond_with(ResponseTemplate::new(200).set_body_json(json))
        .mount(server)
        .await;
}
