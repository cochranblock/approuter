// Unlicense — cochranblock.org
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use axum::{body::Body, extract::State, http::{Request, Response, StatusCode}};
use std::sync::Arc;

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
pub fn f55(p0: Arc<t29>, registry: Option<Arc<t32>>) -> axum::Router {
    let v0 = reqwest::Client::builder()
        .danger_accept_invalid_certs(true)
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .expect("reqwest client");
    axum::Router::new().fallback(f58).with_state((p0, registry, v0))
}

async fn f58(
    State((p0, registry, v0)): State<(Arc<t29>, Option<Arc<t32>>, reqwest::Client)>,
    p1: Request<Body>,
) -> Result<Response<Body>, StatusCode> {
    let v1 = p1.headers().get("host").and_then(|v2| v2.to_str().ok());
    let v2 = p1.uri().path();
    let v3 = p1.uri().query();

    let v5 = if let Some(ref reg) = registry {
        if let Some(base) = reg.get_backend(v1, v2) {
            f57(&base, v2, v3)
        } else {
            let v4 = f56(&p0, v1, v2);
            if let Some(ref v6) = p0.s38 {
                let v7 = v6.trim_end_matches('/');
                let v8 = format!("/{}", v7);
                if !v7.is_empty() && (v2 == v8 || v2 == format!("{}/", v8)) {
                    f57(v4, "/", v3)
                } else if v2.starts_with(&format!("{}/", v8)) {
                    f57(v4, &v2[v8.len()..], v3)
                } else {
                    f57(v4, v2, v3)
                }
            } else {
                f57(v4, v2, v3)
            }
        }
    } else {
        let v4 = f56(&p0, v1, v2);
        if let Some(ref v6) = p0.s38 {
            let v7 = v6.trim_end_matches('/');
            let v8 = format!("/{}", v7);
            if !v7.is_empty() && (v2 == v8 || v2 == format!("{}/", v8)) {
                f57(v4, "/", v3)
            } else if v2.starts_with(&format!("{}/", v8)) {
                f57(v4, &v2[v8.len()..], v3)
            } else {
                f57(v4, v2, v3)
            }
        } else {
            f57(v4, v2, v3)
        }
    };
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
    let v15 = v10.send().await.map_err(|v16| {
        tracing::warn!("proxy upstream error: {}", v16);
        StatusCode::BAD_GATEWAY
    })?;
    let v17 = v15.status().as_u16();
    let v18 = v15.headers().clone();
    let v19 = v15.bytes().await.map_err(|_| StatusCode::BAD_GATEWAY)?;
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
    v20.body(Body::from(v19)).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
