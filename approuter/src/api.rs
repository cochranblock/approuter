// Unlicense — cochranblock.org
// Contributors: mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
//! f98 f99 f100 f101 f103 f104 f105 f106 f107 f108 — approuter API. register, list, unregister, dns_update_a, openapi, tunnel status/stop/ensure/restart/fix. t35=ApiState.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html, IntoResponse},
    Json,
};
use std::path::PathBuf;
use std::process::Child;
use std::sync::{Arc, Mutex};

use crate::cloudflare;
use crate::registry::{t30, t32};
use crate::tunnel;

#[derive(serde::Deserialize)]
pub struct t34 {
    #[serde(rename = "zone_id")]
    pub s6: String,
    #[serde(rename = "record_id")]
    pub s7: String,
    #[serde(rename = "content")]
    pub s8: String,
}

#[derive(serde::Deserialize)]
pub struct t33 {
    #[serde(rename = "app_id")]
    pub s46: String,
    #[serde(rename = "hostnames")]
    pub s47: Vec<String>,
    #[serde(rename = "backend_url")]
    pub s48: String,
}

/// t35 = ApiState. (registry, port, tunnel_handle, config_base_dir). Tunnel handle is None when --no-tunnel.
pub type ApiState = (Arc<t32>, u16, Arc<Mutex<Option<Child>>>, PathBuf);

/// Regenerate tunnel config and restart cloudflared if it was running.
fn restart_tunnel_if_running(p0: &t32, p1: u16, p2: &Arc<Mutex<Option<Child>>>, p3: &std::path::Path) {
    if let Ok(mut guard) = p2.lock() {
        let had_child = if let Some(mut child) = guard.take() {
            let _ = child.kill();
            true
        } else {
            false
        };
        if let Err(e) = tunnel::f91_gen(p3, p0, p1) {
            tracing::warn!("Regenerate tunnel config failed: {}", e);
            return;
        }
        if had_child {
            match tunnel::f92(p3, p0, p1) {
                Ok(child) => {
                    *guard = Some(child);
                    tracing::info!("Tunnel restarted with updated registry");
                }
                Err(e) => tracing::warn!("Tunnel restart failed: {}", e),
            }
        }
    }
}

/// f98 = register_handler. POST /approuter/register.
pub async fn f98(
    State((p0, p1, p2, p3)): State<ApiState>,
    Json(p4): Json<t33>,
) -> impl IntoResponse {
    let v0 = p4.s47.clone();
    let app = t30 {
        s46: p4.s46,
        s47: p4.s47,
        s48: p4.s48,
    };
    match p0.register(app) {
        Ok(()) => {
            if let Err(e) = cloudflare::f96(p0.as_ref(), p1).await {
                tracing::warn!("Tunnel update failed (app registered): {}", e);
            }
            restart_tunnel_if_running(p0.as_ref(), p1, &p2, &p3);
            for h in &v0 {
                if let Err(e) = cloudflare::f95(h, cloudflare::c91()).await {
                    tracing::warn!("DNS CNAME {} failed (non-fatal): {}", h, e);
                }
            }
            (StatusCode::OK, Json(serde_json::json!({"ok": true})))
        }
        Err(e) => (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({"error": e})),
        ),
    }
}

/// f99 = list_apps_handler. GET /approuter/apps.
pub async fn f99(State((p0, _, _, _)): State<ApiState>) -> impl IntoResponse {
    let apps = p0.list_apps();
    (StatusCode::OK, Json(serde_json::json!(apps)))
}

/// f101 = dns_update_a_handler. POST /approuter/dns/update-a.
pub async fn f101(Json(p0): Json<t34>) -> impl IntoResponse {
    match cloudflare::f97(&p0.s6, &p0.s7, &p0.s8).await {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))),
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": e.to_string()})),
        ),
    }
}

/// f109 = dashboard_handler. GET /approuter/ or /approuter. HTML UI.
pub async fn f109() -> impl IntoResponse {
    Html(include_str!("../dashboard.html"))
}

/// f110 = google_apis_handler. GET /approuter/google/apis. Discovery API (free, no auth). ?free_only=1&preferred=1
#[derive(serde::Deserialize)]
pub struct t36 {
    #[serde(default)]
    pub free_only: bool,
    #[serde(default)]
    pub preferred: bool,
}

pub async fn f110(Query(q): Query<t36>) -> impl IntoResponse {
    let url = if q.preferred {
        "https://discovery.googleapis.com/discovery/v1/apis?preferred=true"
    } else {
        "https://discovery.googleapis.com/discovery/v1/apis"
    };
    match reqwest::Client::new()
        .get(url)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
    {
        Ok(res) if res.status().is_success() => {
            match res.json::<serde_json::Value>().await {
                Ok(mut j) => {
                    if q.free_only {
                        let free_names: std::collections::HashSet<&str> = [
                            "discovery", "admin", "gmail", "calendar", "drive", "vision",
                            "language", "speech", "videointelligence", "storage", "searchconsole",
                            "firestore", "pubsub", "secretmanager", "cloudbuild", "bigquery",
                            "appengine", "run", "functions", "recaptchaenterprise", "workflows",
                            "webrisk", "logging", "monitoring",
                        ]
                        .into_iter()
                        .collect();
                        if let Some(items) = j.get_mut("items").and_then(|v| v.as_array_mut()) {
                            items.retain(|item| {
                                item.get("name")
                                    .and_then(|n| n.as_str())
                                    .map(|n| free_names.contains(n))
                                    .unwrap_or(false)
                            });
                        }
                    }
                    (StatusCode::OK, Json(j))
                }
                Err(e) => (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e.to_string()})),
                ),
            }
        }
        Ok(res) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": format!("Discovery API returned {}", res.status())})),
        ),
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(serde_json::json!({"error": e.to_string()})),
        ),
    }
}

/// f103 = openapi_handler. GET /approuter/openapi.json. Point others here.
pub async fn f103() -> impl IntoResponse {
    (
        [("content-type", "application/json")],
        include_str!("../openapi.json"),
    )
}

/// f100 = unregister_handler. DELETE /approuter/apps/:id.
pub async fn f100(
    State((p0, p1, p2, p3)): State<ApiState>,
    Path(p4): Path<String>,
) -> impl IntoResponse {
    match p0.unregister(&p4) {
        Ok(true) => {
            if let Err(e) = cloudflare::f96(p0.as_ref(), p1).await {
                tracing::warn!("Tunnel update failed (app unregistered): {}", e);
            }
            restart_tunnel_if_running(p0.as_ref(), p1, &p2, &p3);
            (StatusCode::OK, Json(serde_json::json!({"ok": true})))
        }
        Ok(false) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error": "app not found"})),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        ),
    }
}

/// f104 = tunnel_status_handler. GET /approuter/tunnel. Returns this approuter's cloudflared instance.
pub async fn f104(State((_, _, p2, _)): State<ApiState>) -> impl IntoResponse {
    let mut guard = match p2.lock() {
        Ok(g) => g,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "lock failed"})),
            )
        }
    };
    let out = if let Some(ref mut child) = *guard {
        let pid = child.id();
        let running = child.try_wait().ok().flatten().is_none();
        serde_json::json!({
            "tunnel_id": tunnel::tunnel_id(),
            "pid": pid,
            "running": running,
        })
    } else {
        serde_json::json!({
            "tunnel_id": tunnel::tunnel_id(),
            "pid": null,
            "running": false,
        })
    };
    (StatusCode::OK, Json(out))
}

/// f105 = tunnel_stop_handler. POST /approuter/tunnel/stop. Kills only this approuter's cloudflared child.
pub async fn f105(State((_, _, p2, _)): State<ApiState>) -> impl IntoResponse {
    let mut guard = match p2.lock() {
        Ok(g) => g,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "lock failed"})),
            )
        }
    };
    if let Some(mut child) = guard.take() {
        let _ = child.kill();
        tracing::info!("Tunnel (cloudflared) stopped via API");
        (StatusCode::OK, Json(serde_json::json!({"ok": true, "stopped": true})))
    } else {
        (StatusCode::OK, Json(serde_json::json!({"ok": true, "stopped": false})))
    }
}

/// f106 = tunnel_ensure_handler. POST /approuter/tunnel/ensure. Downloads cloudflared to base/bin/ if missing.
pub async fn f106(State((_, _, _, p3)): State<ApiState>) -> impl IntoResponse {
    match tunnel::f109(&p3).await {
        Ok(_) => (StatusCode::OK, Json(serde_json::json!({"ok": true, "message": "cloudflared ready"}))),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        ),
    }
}

/// f107 = tunnel_restart_handler. POST /approuter/tunnel/restart. Stops cloudflared and spawns a fresh instance.
pub async fn f107(State((p0, p1, p2, p3)): State<ApiState>) -> impl IntoResponse {
    let mut guard = match p2.lock() {
        Ok(g) => g,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "lock failed"})),
            )
        }
    };
    if let Some(mut child) = guard.take() {
        let _ = child.kill();
    }
    match tunnel::f92(&p3, p0.as_ref(), p1) {
        Ok(child) => {
            *guard = Some(child);
            tracing::info!("Tunnel restarted via API");
            (StatusCode::OK, Json(serde_json::json!({"ok": true, "restarted": true})))
        }
        Err(e) => {
            tracing::warn!("Tunnel restart failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string(), "restarted": false})),
            )
        }
    }
}

/// f108 = tunnel_fix_handler. POST /approuter/tunnel/fix. Ensures cloudflared binary exists, stops old instance, spawns fresh. Fix for 1033 etc.
pub async fn f108(State((p0, p1, p2, p3)): State<ApiState>) -> impl IntoResponse {
    if let Err(e) = tunnel::f109(&p3).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("ensure cloudflared failed: {}", e)})),
        );
    }
    let mut guard = match p2.lock() {
        Ok(g) => g,
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": "lock failed"})),
            )
        }
    };
    if let Some(mut child) = guard.take() {
        let _ = child.kill();
    }
    match tunnel::f92(&p3, p0.as_ref(), p1) {
        Ok(child) => {
            *guard = Some(child);
            tracing::info!("Tunnel fixed via API (ensure + restart)");
            (StatusCode::OK, Json(serde_json::json!({"ok": true, "fixed": true})))
        }
        Err(e) => {
            tracing::warn!("Tunnel fix spawn failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string(), "fixed": false})),
            )
        }
    }
}
