#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use axum::{body::Body, extract::State, http::{Request, Response, StatusCode}};
use std::sync::Arc;
use std::time::Instant;

use crate::analytics;
use crate::metrics_catalog::{self, IngressPath, MetricsCatalog, RequestEvent, UaClass};
use crate::registry::t32;

#[derive(Clone)]
pub struct t29 {
    pub s35: String,
    pub s36: String,
    pub s37: Vec<String>,
    pub s38: Option<String>,
    pub s42: String,
    pub s43: Vec<String>,
    pub s49: String,
    pub s50: Vec<String>,
    pub s51: Option<String>,
}

/// f56 = resolve_origin
fn f56<'a>(p0: &'a t29, p1: Option<&str>, p2: &str) -> &'a str {
    let v0 = p1.map(|v1| v1.split(':').next().unwrap_or(v1).trim());
    if let Some(v2) = v0 {
        if let Some(ref suf) = p0.s51 {
            let s = suf.trim();
            if !s.is_empty() && (v2.eq_ignore_ascii_case(s.trim_start_matches('.')) || v2.ends_with(s)) {
                return &p0.s49;
            }
        }
        for v3 in &p0.s50 {
            if v2.eq_ignore_ascii_case(v3) {
                return &p0.s49;
            }
        }
        for v3 in &p0.s43 {
            if v2.eq_ignore_ascii_case(v3) {
                return &p0.s42;
            }
        }
        for v3 in &p0.s37 {
            if v2.eq_ignore_ascii_case(v3) {
                return &p0.s36;
            }
        }
    }
    if let Some(ref v4) = p0.s38 {
        let v5 = v4.trim_end_matches('/');
        if !v5.is_empty() {
            let v6 = format!("/{}", v5);
            if p2 == v6 || p2 == format!("{}/", v6) || p2.starts_with(&format!("{}/", v6)) {
                return &p0.s36;
            }
        }
    }
    &p0.s35
}

/// f57 = build_proxy_url
fn f57(p0: &str, p1: &str, p2: Option<&str>) -> String {
    let v0 = if p1.is_empty() { "/" } else { p1 };
    let v1 = p2.map(|v2| format!("?{}", v2)).unwrap_or_default();
    format!("{}{}{}", p0, v0, v1)
}

/// f55 = proxy_router. Registry takes precedence over legacy t29.
pub fn f55(
    p0: Arc<t29>,
    registry: Option<Arc<t32>>,
    analytics: Option<Arc<analytics::t42>>,
    catalog: Option<Arc<MetricsCatalog>>,
) -> axum::Router {
    let v0 = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .timeout(std::time::Duration::from_secs(30))
        .connect_timeout(std::time::Duration::from_secs(5))
        .build()
        .expect("reqwest client");
    axum::Router::new()
        .fallback(f58)
        .with_state((p0, registry, v0, analytics, catalog))
}

type ProxyState = (
    Arc<t29>,
    Option<Arc<t32>>,
    reqwest::Client,
    Option<Arc<analytics::t42>>,
    Option<Arc<MetricsCatalog>>,
);

/// Resolve `(backend_app, backend_base_url, proxy_url)` for an inbound request.
/// Registry wins; falls back to legacy `t29` hostnames. `backend_app` is empty
/// when the legacy path serves the request.
fn resolve_backend(
    p0: &t29,
    registry: Option<&Arc<t32>>,
    host: Option<&str>,
    path: &str,
    query: Option<&str>,
) -> (String, String, String) {
    if let Some(reg) = registry {
        if let Some((app_id, base)) = reg.resolve_app(host) {
            let url = f57(&base, path, query);
            return (app_id, base, url);
        }
    }
    let base = f56(p0, host, path).to_string();
    let url = if let Some(ref suf) = p0.s38 {
        let strip = suf.trim_end_matches('/');
        let root = format!("/{}", strip);
        if !strip.is_empty() && (path == root || path == format!("{}/", root)) {
            f57(&base, "/", query)
        } else if path.starts_with(&format!("{}/", root)) {
            f57(&base, &path[root.len()..], query)
        } else {
            f57(&base, path, query)
        }
    } else {
        f57(&base, path, query)
    };
    (String::new(), base, url)
}

/// Extract `tls_version` from the `cf-visitor` header (set by CF when the
/// request arrived over HTTPS at the edge). Returns `None` on non-CF ingress.
fn tls_from_cf_visitor(headers: &axum::http::HeaderMap) -> Option<String> {
    let raw = headers.get("cf-visitor")?.to_str().ok()?;
    let j: serde_json::Value = serde_json::from_str(raw).ok()?;
    j.get("scheme").and_then(|s| s.as_str()).map(|s| {
        if s.eq_ignore_ascii_case("https") {
            "TLS (CF-edge)".to_string()
        } else {
            "none".to_string()
        }
    })
}

fn header_str(headers: &axum::http::HeaderMap, name: &str) -> String {
    headers
        .get(name)
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string()
}

async fn f58(
    State((p0, registry, v0, analytics_store, catalog)): State<ProxyState>,
    p1: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    let start = Instant::now();
    let req_headers = p1.headers().clone();
    let req_method = p1.method().to_string();
    let http_version = format!("{:?}", p1.version());
    let ingress_path = p1
        .extensions()
        .get::<IngressPath>()
        .copied()
        .unwrap_or(IngressPath::CfTunnel);

    // Block Tor exit nodes (CF-IPCountry: T1)
    if p1.headers().get("cf-ipcountry").and_then(|v| v.to_str().ok()) == Some("T1") {
        return Ok(Response::builder()
            .status(StatusCode::FORBIDDEN)
            .body(Body::from("403 Forbidden"))
            .unwrap());
    }

    let v1 = p1.headers().get("host").and_then(|v2| v2.to_str().ok());
    let v2 = p1.uri().path();
    let v3 = p1.uri().query();
    let req_path = v2.to_string();
    let req_host = v1
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_string();

    let (backend_app, backend_base, v5) = resolve_backend(&p0, registry.as_ref(), v1, v2, v3);
    let (v6, v7) = p1.into_parts();
    let v8 = axum::body::to_bytes(v7, 10 * 1024 * 1024)
        .await
        .map_err(|_| StatusCode::BAD_REQUEST)?;
    let v9: reqwest::Method = v6.method.as_str().parse().map_err(|_| StatusCode::BAD_GATEWAY)?;
    let mut v10 = v0.request(v9, &v5);
    for (v11, v12) in v6.headers.iter() {
        if v11.as_str().eq_ignore_ascii_case("transfer-encoding") || v11.as_str().eq_ignore_ascii_case("connection") {
            continue;
        }
        if let (Ok(v13), Ok(v14)) = (
            v11.as_str().parse::<reqwest::header::HeaderName>(),
            reqwest::header::HeaderValue::from_bytes(v12.as_bytes()),
        ) {
            v10 = v10.header(v13, v14);
        }
    }
    v10 = v10.body(v8);

    let backend_call_start = Instant::now();
    let v15 = match v10.send().await {
        Ok(r) => r,
        Err(v16) => {
            tracing::warn!("proxy upstream error: {}", v16);
            let backend_ms = backend_call_start.elapsed().as_millis() as u64;
            let duration_ms = start.elapsed().as_millis() as u64;
            if let Some(ref store) = analytics_store {
                let event = analytics::extract_event(&req_headers, &req_method, &req_path, 502, duration_ms);
                store.record(event);
            }
            record_catalog_event(
                catalog.as_ref(),
                &req_headers,
                &req_method,
                &req_path,
                &req_host,
                &backend_app,
                &backend_base,
                502,
                0,
                duration_ms,
                backend_ms,
                &http_version,
                ingress_path,
            );
            return Err(StatusCode::BAD_GATEWAY);
        }
    };
    let v17 = v15.status().as_u16();
    let v18 = v15.headers().clone();
    let backend_ms = backend_call_start.elapsed().as_millis() as u64;
    let v19 = v15.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
    let response_bytes = v19.len() as u64;
    let mut v20 = Response::builder().status(v17);
    for (v11, v12) in v18.iter() {
        if v11.as_str().eq_ignore_ascii_case("transfer-encoding") || v11.as_str().eq_ignore_ascii_case("connection") {
            continue;
        }
        if let (Ok(v13), Ok(v14)) = (
            v11.as_str().parse::<axum::http::header::HeaderName>(),
            axum::http::header::HeaderValue::from_bytes(v12.as_bytes()),
        ) {
            v20 = v20.header(v13, v14);
        }
    }
    let duration_ms = start.elapsed().as_millis() as u64;
    if let Some(ref store) = analytics_store {
        let event = analytics::extract_event(&req_headers, &req_method, &req_path, v17, duration_ms);
        store.record(event);
    }
    record_catalog_event(
        catalog.as_ref(),
        &req_headers,
        &req_method,
        &req_path,
        &req_host,
        &backend_app,
        &backend_base,
        v17,
        response_bytes,
        duration_ms,
        backend_ms,
        &http_version,
        ingress_path,
    );

    v20.body(Body::from(v19)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

#[allow(clippy::too_many_arguments)]
fn record_catalog_event(
    catalog: Option<&Arc<MetricsCatalog>>,
    req_headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
    host: &str,
    backend_app: &str,
    backend_url: &str,
    status: u16,
    response_bytes: u64,
    duration_ms: u64,
    backend_ms: u64,
    http_version: &str,
    ingress_path: IngressPath,
) {
    let Some(catalog) = catalog else { return };
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    let country = header_str(req_headers, "cf-ipcountry");
    let raw_ip = header_str(req_headers, "cf-connecting-ip");
    let ua = header_str(req_headers, "user-agent");
    let tls_version = tls_from_cf_visitor(req_headers);
    let event = RequestEvent {
        ts: now,
        method: method.to_string(),
        path: path.to_string(),
        host: host.to_string(),
        status_code: status,
        response_bytes,
        response_time_ms: duration_ms,
        client_ip_trunc: metrics_catalog::truncate_ip(&raw_ip),
        country,
        ua_class: UaClass::from_ua(&ua),
        ingress_path,
        tls_version,
        http_version: http_version.to_string(),
        cache_hit: false,
        backend_app: backend_app.to_string(),
        backend_url: backend_url.to_string(),
        backend_latency_ms: backend_ms,
        error_type: metrics_catalog::ErrorType::from_status(status),
    };
    catalog.record(event);
}