#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

//! metrics_api — HTTP endpoints that surface the metrics catalog.
//!
//! Three endpoints, one store:
//! - `GET /approuter/metrics` — authenticated, full detail (JSON).
//! - `GET /approuter/metrics/public` — unauthenticated, redacted (JSON).
//! - `GET /approuter/metrics/prometheus` — authenticated, Prometheus text.
//!
//! Authentication uses the same `ROUTER_API_KEY` gate as the rest of the
//! approuter API (`api::f139`). When the env var is absent, endpoints that
//! expose sensitive data remain gated by *not existing* — they 200, but the
//! operator is responsible for not exposing approuter directly to the public.
//! The `/public` variant is deliberately safe to expose.

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Json,
};
use serde::Serialize;
use std::sync::Arc;

use crate::api::f139;
use crate::metrics_catalog::{MetricsCatalog, MetricsSnapshot, PublicMetricsSnapshot};
use crate::registry::t32;
use crate::selfcheck::{SelfCheckSnapshot, SelfCheckStore};

/// Shared state for the metrics endpoints. Kept as a single tuple alias so
/// the `with_state` call in `main.rs` stays legible.
pub type MetricsState = (
    Arc<MetricsCatalog>,
    Arc<SelfCheckStore>,
    Arc<t32>,
    String, // binary version (CARGO_PKG_VERSION)
    String, // binary_sha (env GIT_SHA, empty if unset)
);

/// Top-level authenticated response. One flat shape for grafana/JSON consumers.
#[derive(Serialize)]
pub struct FullMetricsResponse {
    pub process: ProcessInfo,
    pub ingress: std::collections::HashMap<String, u64>,
    pub per_route: Vec<crate::metrics_catalog::RouteSnapshot>,
    pub top_countries_24h: Vec<(String, u64)>,
    pub top_paths_24h: Vec<(String, u64)>,
    pub top_user_agents_24h: Vec<(String, u64)>,
    pub probe_paths_detected: Vec<crate::metrics_catalog::ProbeHit>,
    pub selfcheck: SelfCheckSnapshot,
    pub backends: Vec<BackendEntry>,
    pub dns: DnsSnapshot,
    pub hourly: Vec<crate::metrics_catalog::HourlyBucket>,
}

#[derive(Serialize)]
pub struct ProcessInfo {
    pub uptime_s: u64,
    pub started_at: u64,
    pub version: String,
    pub binary_sha: String,
    pub memory_rss_bytes: u64,
    pub total_requests: u64,
    pub total_bytes_out: u64,
    pub total_errors: u64,
    pub event_ring_len: usize,
    pub event_ring_capacity: usize,
}

#[derive(Serialize)]
pub struct BackendEntry {
    pub app_id: String,
    pub hostnames: Vec<String>,
    pub backend_url: String,
}

#[derive(Serialize)]
pub struct DnsSnapshot {
    pub last_known_external_ip: Option<String>,
}

/// GET /approuter/metrics — authenticated, full detail.
pub async fn metrics_full(
    State((catalog, selfcheck_store, registry, version, binary_sha)): State<MetricsState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(resp) = f139(&headers) {
        return resp.into_response();
    }
    let snap: MetricsSnapshot = catalog.snapshot();
    let selfcheck = selfcheck_store.snapshot();
    let backends: Vec<BackendEntry> = registry
        .list_apps()
        .into_iter()
        .map(|a| BackendEntry {
            app_id: a.s46,
            hostnames: a.s47,
            backend_url: a.s48,
        })
        .collect();

    let ingress_simple: std::collections::HashMap<String, u64> = snap
        .ingress
        .iter()
        .map(|(k, v)| (k.clone(), v.request_count))
        .collect();

    let body = FullMetricsResponse {
        process: ProcessInfo {
            uptime_s: snap.uptime_s,
            started_at: snap.started_at,
            version,
            binary_sha,
            memory_rss_bytes: process_rss_bytes(),
            total_requests: snap.total_requests,
            total_bytes_out: snap.total_bytes_out,
            total_errors: snap.total_errors,
            event_ring_len: snap.event_ring_len,
            event_ring_capacity: snap.event_ring_capacity,
        },
        ingress: ingress_simple,
        per_route: snap.per_route,
        top_countries_24h: snap.top_countries_24h,
        top_paths_24h: snap.top_paths_24h,
        top_user_agents_24h: snap.top_user_agents_24h,
        probe_paths_detected: snap.probe_paths_detected,
        selfcheck,
        backends,
        dns: DnsSnapshot {
            last_known_external_ip: selfcheck_store.external_ip(),
        },
        hourly: snap.hourly,
    };
    (StatusCode::OK, Json(body)).into_response()
}

/// GET /approuter/metrics/public — unauthenticated, aggregate-only.
pub async fn metrics_public(
    State((catalog, selfcheck_store, _, _, _)): State<MetricsState>,
) -> impl IntoResponse {
    let snap: PublicMetricsSnapshot = catalog.snapshot_public();
    let selfcheck = selfcheck_store.snapshot();
    let body = serde_json::json!({
        "uptime_s": snap.uptime_s,
        "total_requests": snap.total_requests,
        "total_errors": snap.total_errors,
        "total_bytes_out": snap.total_bytes_out,
        "ingress": snap.ingress,
        "per_country": snap.per_country,
        "hourly": snap.hourly,
        "probe_paths_detected_count": snap.probe_paths_detected_count,
        "selfcheck_summary": {
            "last_check_ts": selfcheck.last_check_ts,
            "cf_consecutive_failures": selfcheck.cf.consecutive_failures,
            "direct_consecutive_failures": selfcheck.direct.consecutive_failures,
            "cf_p95_latency_ms": selfcheck.cf.p95_latency_ms,
            "direct_p95_latency_ms": selfcheck.direct.p95_latency_ms,
        }
    });
    (StatusCode::OK, Json(body))
}

/// GET /approuter/metrics/prometheus — authenticated, Prometheus text format.
pub async fn metrics_prometheus(
    State((catalog, selfcheck_store, _, _, _)): State<MetricsState>,
    headers: HeaderMap,
) -> impl IntoResponse {
    if let Some(resp) = f139(&headers) {
        return resp.into_response();
    }
    let mut text = catalog.prometheus_text();
    let snap = selfcheck_store.snapshot();
    text.push_str("# HELP approuter_selfcheck_consecutive_failures Consecutive self-check failures per path.\n");
    text.push_str("# TYPE approuter_selfcheck_consecutive_failures gauge\n");
    text.push_str(&format!(
        "approuter_selfcheck_consecutive_failures{{path=\"cf\"}} {}\n",
        snap.cf.consecutive_failures
    ));
    text.push_str(&format!(
        "approuter_selfcheck_consecutive_failures{{path=\"direct\"}} {}\n",
        snap.direct.consecutive_failures
    ));
    text.push_str("# HELP approuter_selfcheck_p95_latency_ms Self-check p95 latency per path.\n");
    text.push_str("# TYPE approuter_selfcheck_p95_latency_ms gauge\n");
    text.push_str(&format!(
        "approuter_selfcheck_p95_latency_ms{{path=\"cf\"}} {}\n",
        snap.cf.p95_latency_ms
    ));
    text.push_str(&format!(
        "approuter_selfcheck_p95_latency_ms{{path=\"direct\"}} {}\n",
        snap.direct.p95_latency_ms
    ));
    text.push_str("# HELP approuter_process_memory_rss_bytes Resident set size of the proxy.\n");
    text.push_str("# TYPE approuter_process_memory_rss_bytes gauge\n");
    text.push_str(&format!(
        "approuter_process_memory_rss_bytes {}\n",
        process_rss_bytes()
    ));
    (
        StatusCode::OK,
        [("content-type", "text/plain; version=0.0.4")],
        text,
    )
        .into_response()
}

// ---------- process stats ----------

/// Return the current process RSS in bytes. Uses `getrusage(RUSAGE_SELF)` on
/// Unix (macOS reports bytes, Linux reports kilobytes — normalized here).
/// Returns 0 on non-Unix platforms or on error.
pub fn process_rss_bytes() -> u64 {
    #[cfg(unix)]
    {
        unsafe {
            let mut usage: libc::rusage = std::mem::zeroed();
            if libc::getrusage(libc::RUSAGE_SELF, &mut usage) != 0 {
                return 0;
            }
            let raw = usage.ru_maxrss as u64;
            if cfg!(target_os = "macos") {
                raw
            } else {
                raw.saturating_mul(1024)
            }
        }
    }
    #[cfg(not(unix))]
    {
        0
    }
}
