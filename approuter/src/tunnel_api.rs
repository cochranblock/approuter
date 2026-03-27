//! tunnel_api — Multi-tunnel API endpoints. Status, start/stop, health check, metrics comparison.

// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use std::sync::Arc;

use crate::registry::t32;
use crate::tunnel_metrics::t50;
use crate::tunnel_provider::{t44, t47};

/// t51 = TunnelApiState.
pub type TunnelApiState = (Arc<t47>, Arc<t50>, Arc<t32>);

/// GET /approuter/tunnels — All providers status.
pub async fn tunnels_status(State((mgr, _, _)): State<TunnelApiState>) -> impl IntoResponse {
    let status = mgr.status_all();
    (StatusCode::OK, Json(serde_json::json!({ "tunnels": status })))
}

/// POST /approuter/tunnels/:provider/start — Start a specific provider.
pub async fn tunnel_start(
    State((mgr, _, reg)): State<TunnelApiState>,
    Path(provider): Path<String>,
) -> impl IntoResponse {
    let kind = match t44::from_str(&provider) {
        Some(k) => k,
        None => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({
            "error": format!("unknown provider: {}. valid: cloudflare, ngrok, tailscale, bore, localtunnel", provider)
        }))),
    };

    let cfg = mgr.configs().iter().find(|c| c.kind == kind);
    let cfg = match cfg {
        Some(c) => c.clone(),
        None => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error": "provider not configured"}))),
    };

    match mgr.spawn_provider(&cfg, reg.as_ref()) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true, "provider": kind.name()}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))),
    }
}

/// POST /approuter/tunnels/:provider/stop — Stop a specific provider.
pub async fn tunnel_stop(
    State((mgr, _, _)): State<TunnelApiState>,
    Path(provider): Path<String>,
) -> impl IntoResponse {
    let kind = match t44::from_str(&provider) {
        Some(k) => k,
        None => return (StatusCode::BAD_REQUEST, Json(serde_json::json!({"error": "unknown provider"}))),
    };
    mgr.stop_provider(&kind);
    (StatusCode::OK, Json(serde_json::json!({"ok": true, "stopped": kind.name()})))
}

/// GET /approuter/tunnels/health — Health check all running providers.
pub async fn tunnels_health(State((mgr, _, _)): State<TunnelApiState>) -> impl IntoResponse {
    let results = mgr.health_check_all().await;
    (StatusCode::OK, Json(serde_json::json!({ "health": results })))
}

#[derive(serde::Deserialize)]
pub struct MetricsQuery {
    #[serde(default)]
    pub hours: Option<u64>,
    #[serde(default)]
    pub provider: Option<String>,
}

/// GET /approuter/tunnels/metrics — Comparison metrics across all providers.
pub async fn tunnels_metrics(
    State((_, metrics, _)): State<TunnelApiState>,
    Query(q): Query<MetricsQuery>,
) -> impl IntoResponse {
    if let Some(ref p) = q.provider {
        if let Some(kind) = t44::from_str(p) {
            let stats = metrics.provider_stats(&kind, q.hours);
            return (StatusCode::OK, Json(serde_json::json!(stats)));
        }
    }
    let comparison = metrics.comparison(q.hours);
    (StatusCode::OK, Json(serde_json::json!({ "comparison": comparison })))
}

/// GET /approuter/tunnels/metrics/probes — Recent raw probe data.
pub async fn tunnels_probes(
    State((_, metrics, _)): State<TunnelApiState>,
    Query(q): Query<MetricsQuery>,
) -> impl IntoResponse {
    if let Some(ref p) = q.provider {
        if let Some(kind) = t44::from_str(p) {
            let probes = metrics.probe_history(&kind, 200);
            return (StatusCode::OK, Json(serde_json::json!({ "probes": probes })));
        }
    }
    let probes = metrics.recent_probes(200);
    (StatusCode::OK, Json(serde_json::json!({ "probes": probes })))
}

/// GET /approuter/tunnels/compete — Dashboard HTML.
pub async fn tunnels_dashboard() -> impl IntoResponse {
    Html(include_str!("../tunnels.html"))
}
