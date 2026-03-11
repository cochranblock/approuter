// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! HTTP API. kova serve. POST /api/intent, GET /ws/stream (WebSocket). Web GUI at /.
//! f114=serve_run

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::IntoResponse,
    response::Html,
    routing::{get, post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{broadcast, Mutex};
use tower_http::cors::CorsLayer;

/// t92=AppState. Shared state: pipeline broadcast receiver for WebSocket clients.
#[derive(Clone)]
pub struct AppState {
    #[cfg(feature = "inference")]
    pipeline_rx: Arc<Mutex<Option<broadcast::Receiver<Arc<str>>>>>,
    #[cfg(feature = "inference")]
    last_trace: Arc<Mutex<Option<crate::trace::LastTrace>>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            #[cfg(feature = "inference")]
            pipeline_rx: Arc::new(Mutex::new(None)),
            #[cfg(feature = "inference")]
            last_trace: Arc::new(Mutex::new(None)),
        }
    }
}

#[derive(Serialize)]
struct Status {
    status: &'static str,
}

#[derive(Deserialize)]
struct RecentQuery {
    project: Option<PathBuf>,
    #[serde(default = "default_minutes")]
    minutes: u64,
}

fn default_minutes() -> u64 {
    30
}

#[derive(Deserialize)]
struct BuildCommandQuery {
    project: String,
    #[serde(default)]
    release: bool,
}

#[derive(Serialize)]
struct BuildCommandResponse {
    command: String,
}

#[derive(Deserialize)]
struct FileQuery {
    hint: Option<String>,
}

#[derive(Deserialize)]
struct DiffBody {
    hint: Option<String>,
    #[serde(rename = "new_content")]
    new_content: Option<String>,
}

#[derive(Deserialize)]
struct ApplyBody {
    hint: Option<String>,
    content: Option<String>,
}

#[derive(Deserialize)]
struct BacklogRunBody {
    index: usize,
}

/// Validate hint is a safe filename: word.rs only. Returns hint or "lib.rs".
fn safe_hint(hint: Option<&str>) -> String {
    let h = hint.unwrap_or("lib.rs").trim();
    if let Some(stem) = h.strip_suffix(".rs") {
        if !stem.is_empty() && stem.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return h.to_string();
        }
    }
    "lib.rs".to_string()
}

async fn api_projects() -> Json<Vec<String>> {
    let projects = crate::discover_projects();
    Json(
        projects
            .iter()
            .filter_map(|p| p.to_str().map(String::from))
            .collect(),
    )
}

async fn api_prompts() -> impl IntoResponse {
    let system = crate::load_prompt("system");
    let persona = crate::load_prompt("persona");
    (StatusCode::OK, Json(serde_json::json!({"system": system, "persona": persona}))).into_response()
}

async fn recent_changes(Query(q): Query<RecentQuery>) -> String {
    let project = q
        .project
        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default());
    let changes = crate::recent_changes::f86(
        &project,
        std::time::Duration::from_secs(q.minutes * 60),
    );
    crate::recent_changes::f87(&changes)
}

async fn build_presets() -> Json<std::collections::HashMap<String, crate::BuildPreset>> {
    Json(crate::all_build_presets())
}

#[cfg(feature = "inference")]
async fn api_route(Json(req): Json<RouteRequest>) -> impl IntoResponse {
    let router_path = match crate::f78(crate::ModelRole::Router) {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(RouteResponse {
                    classification: String::new(),
                    needs_clarification: None,
                    suggested_question: None,
                    choices: None,
                    enriched_message: None,
                    error: Some("No router model. Run: kova model install".into()),
                }),
            )
                .into_response()
        }
    };
    let to_route = if let Some(ref prior) = req.prior_message {
        let reply = req.message.trim();
        if reply.contains(".rs") {
            format!("{} in {}", prior, reply)
        } else {
            format!("{} {}", prior, reply)
        }
    } else {
        req.message.clone()
    };
    let path = router_path.clone();
    let msg = to_route.clone();
    let prior = req.prior_message.clone();
    let result = tokio::task::spawn_blocking(move || {
        let rx = crate::f79(&path, &msg);
        rx.recv().ok()
    })
    .await
    .ok()
    .flatten();
    let Some(result) = result else {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(RouteResponse {
                classification: String::new(),
                needs_clarification: None,
                suggested_question: None,
                choices: None,
                enriched_message: None,
                error: Some("Router did not respond".into()),
            }),
        )
            .into_response();
    };
    let (classification, needs_clarification, suggested_question, choices, _) =
        match &result {
            crate::RouterResult::CodeGen => ("code_gen".into(), None, None, None, None::<String>),
            crate::RouterResult::Refactor => ("refactor".into(), None, None, None, None::<String>),
            crate::RouterResult::Explain => ("explain".into(), None, None, None, None::<String>),
            crate::RouterResult::Fix => ("fix".into(), None, None, None, None::<String>),
            crate::RouterResult::Run => ("run".into(), None, None, None, None::<String>),
            crate::RouterResult::Custom => ("custom".into(), None, None, None, None::<String>),
            crate::RouterResult::NeedsClarification { .. } => {
                let q = result.clarification_question(&req.message);
                let ch = result.clarification_choices().map(|s| s.to_vec());
                (
                    "needs_clarification".into(),
                    Some(true),
                    Some(q),
                    ch,
                    None::<String>,
                )
            }
            crate::RouterResult::Error(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(RouteResponse {
                        classification: "error".into(),
                        needs_clarification: None,
                        suggested_question: None,
                        choices: None,
                        enriched_message: None,
                        error: Some(e.clone()),
                    }),
                )
                    .into_response();
            }
        };
    let enriched = if prior.is_some() && !matches!(&result, crate::RouterResult::NeedsClarification { .. }) {
        Some(to_route)
    } else {
        None
    };
    (
        StatusCode::OK,
        Json(RouteResponse {
            classification,
            needs_clarification,
            suggested_question,
            choices,
            enriched_message: enriched,
            error: None,
        }),
    )
        .into_response()
}

#[derive(Serialize)]
struct IntentResponse {
    accepted: bool,
    intent_name: String,
    /// When FullPipeline runs plan path (no inference), summary of results.
    #[serde(skip_serializing_if = "Option::is_none")]
    summary: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct RouteRequest {
    message: String,
    #[serde(rename = "prior_message")]
    prior_message: Option<String>,
    #[allow(dead_code)]
    project: Option<String>,
}

#[derive(Serialize)]
struct RouteResponse {
    classification: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    needs_clarification: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    suggested_question: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    choices: Option<Vec<String>>,
    /// When prior_message was provided and we routed on enriched context.
    #[serde(skip_serializing_if = "Option::is_none")]
    enriched_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<String>,
}

#[derive(Deserialize)]
struct IntentRequest {
    #[serde(flatten)]
    intent: kova_core::t0,
    /// Override project path (default: server default_project)
    project: Option<String>,
}

async fn api_intent(
    State(state): State<AppState>,
    Json(req): Json<IntentRequest>,
) -> Json<IntentResponse> {
    let intent = req.intent;
    let name = kova_core::intent_name(&intent.s0);
    let mut summary = None;
    if matches!(intent.s0, kova_core::t1::FullPipeline) {
        #[cfg(feature = "inference")]
        {
            if let (Some(coder), Some(fix)) = (
                crate::f78(crate::ModelRole::Coder),
                crate::f78(crate::ModelRole::Fix).or_else(|| crate::f78(crate::ModelRole::Coder)),
            ) {
                let project = req
                    .project
                    .as_ref()
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(crate::default_project);
                let system = crate::load_prompt("system");
                let persona = crate::load_prompt("persona");
                let cursor = crate::cursor_prompts::load_cursor_prompts(&project);
                let system_prompt = if cursor.is_empty() {
                    format!("{}\n\n{}", system, persona)
                } else {
                    format!("{}\n\n{}\n\n--- Cursor rules ---\n{}", system, persona, cursor)
                };
                let user_msg = intent.s1.as_deref().unwrap_or("Generate code.");
                let rx = crate::pipeline::f81(
                    &coder,
                    &fix,
                    &system_prompt,
                    user_msg,
                    &project,
                    crate::orchestration_max_fix_retries(),
                    crate::orchestration_run_clippy(),
                    Some(state.last_trace.clone()),
                );
                let mut guard = state.pipeline_rx.lock().await;
                *guard = Some(rx);
                return Json(IntentResponse {
                    accepted: true,
                    intent_name: name.to_string(),
                    summary: None,
                });
            }
        }
        // Fallback: no inference — run plan path (cargo check, cargo test)
        let project = req
            .project
            .as_ref()
            .map(std::path::PathBuf::from)
            .unwrap_or_else(crate::default_project);
        let approuter_dir = std::env::var("HOME")
            .ok()
            .map(|h| std::path::PathBuf::from(h).join("approuter"));
        let plan = crate::t3::f14(&intent, project, approuter_dir);
        let exec = crate::t6;
        if let Ok(results) = tokio::task::block_in_place(|| exec.f15(&plan)) {
            summary = Some(
                results
                    .iter()
                    .map(|r| format!("{} {} {}", if r.s11 { "✓" } else { "✗" }, r.s10, r.s13))
                    .collect(),
            );
        }
    }
    Json(IntentResponse {
        accepted: true,
        intent_name: name.to_string(),
        summary,
    })
}

async fn ws_stream(
    State(state): State<AppState>,
    ws: WebSocketUpgrade,
) -> axum::response::Response {
    ws.on_upgrade(move |socket| ws_handler(state, socket))
}

async fn ws_handler(state: AppState, mut socket: WebSocket) {
    #[cfg(feature = "inference")]
    {
        let mut rx_opt = state.pipeline_rx.lock().await;
        if let Some(mut rx) = rx_opt.take() {
            drop(rx_opt);
            loop {
                match rx.recv().await {
                    Ok(msg) => {
                        if socket.send(Message::Text((*msg).to_string())).await.is_err() {
                            return;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => continue,
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            return;
        }
    }
    for chunk in ["Mock ", "stream ", "data.\n"].iter() {
        if socket.send(Message::Text(chunk.to_string())).await.is_err() {
            return;
        }
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
}

#[cfg(feature = "inference")]
async fn api_explain(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.last_trace.lock().await;
    match guard.as_ref() {
        Some(t) => Json(serde_json::to_value(t).unwrap_or_default()).into_response(),
        None => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({"error":"no trace"})),
        )
            .into_response(),
    }
}

async fn api_file(Query(q): Query<FileQuery>) -> impl IntoResponse {
    let project = crate::default_project();
    let hint = safe_hint(q.hint.as_deref());
    let target = crate::output::f85(&project, Some(&hint));
    match tokio::fs::read_to_string(&target).await {
        Ok(s) => (StatusCode::OK, s).into_response(),
        Err(_) => (
            StatusCode::NOT_FOUND,
            format!("File not found: {}", target.display()),
        )
            .into_response(),
    }
}

async fn api_diff(Json(body): Json<DiffBody>) -> impl IntoResponse {
    let project = crate::default_project();
    let hint = safe_hint(body.hint.as_deref());
    let target = crate::output::f85(&project, Some(&hint));
    let new_content = body.new_content.as_deref().unwrap_or("");
    let current = tokio::fs::read_to_string(&target).await.unwrap_or_default();
    let diff = crate::output::f84(&current, new_content);
    (StatusCode::OK, diff).into_response()
}

async fn api_apply(Json(body): Json<ApplyBody>) -> impl IntoResponse {
    let project = crate::default_project();
    let hint = safe_hint(body.hint.as_deref());
    let target = crate::output::f85(&project, Some(&hint));
    let content = body.content.as_deref().unwrap_or("");
    if let Err(e) = tokio::fs::create_dir_all(target.parent().unwrap_or(&project)).await {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("create dir: {}", e)})),
        )
            .into_response();
    }
    match tokio::fs::write(&target, content).await {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({"ok": true, "path": target.display().to_string()})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": format!("write: {}", e)})),
        )
            .into_response(),
    }
}

async fn api_backlog_get() -> impl IntoResponse {
    let path = crate::backlog_path();
    match crate::f25(&path) {
        Ok(b) => (StatusCode::OK, Json(b)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn api_backlog_run(
    State(state): State<AppState>,
    Json(body): Json<BacklogRunBody>,
) -> impl IntoResponse {
    let path = crate::backlog_path();
    let backlog = match crate::f25(&path) {
        Ok(b) => b,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    };
    let entry = match backlog.items.get(body.index) {
        Some(e) => e.clone(),
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error": "index out of range"})),
            )
                .into_response()
        }
    };
    let intent = match kova_core::entry_to_intent(&entry) {
        Some(i) => i,
        None => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": "unsupported intent"})),
            )
                .into_response()
        }
    };
    let project = entry
        .project
        .as_ref()
        .map(std::path::PathBuf::from)
        .unwrap_or_else(crate::default_project);
    let approuter_dir = entry
        .approuter_dir
        .as_ref()
        .map(std::path::PathBuf::from)
        .or_else(|| std::env::var("HOME").ok().map(|h| std::path::PathBuf::from(h).join("approuter")));

    if matches!(intent.s0, kova_core::t1::FullPipeline) {
        #[cfg(feature = "inference")]
        {
            if let (Some(coder), Some(fix)) = (
                crate::f78(crate::ModelRole::Coder),
                crate::f78(crate::ModelRole::Fix).or_else(|| crate::f78(crate::ModelRole::Coder)),
            ) {
                let system = crate::load_prompt("system");
                let persona = crate::load_prompt("persona");
                let cursor = crate::cursor_prompts::load_cursor_prompts(&project);
                let system_prompt = if cursor.is_empty() {
                    format!("{}\n\n{}", system, persona)
                } else {
                    format!("{}\n\n{}\n\n--- Cursor rules ---\n{}", system, persona, cursor)
                };
                let user_msg = intent.s1.as_deref().unwrap_or("Generate code.");
                let rx = crate::pipeline::f81(
                    &coder,
                    &fix,
                    &system_prompt,
                    user_msg,
                    &project,
                    crate::orchestration_max_fix_retries(),
                    crate::orchestration_run_clippy(),
                    Some(state.last_trace.clone()),
                );
                {
                    let mut guard = state.pipeline_rx.lock().await;
                    *guard = Some(rx);
                }
                return (StatusCode::OK, Json(serde_json::json!({"message": "Pipeline started.", "stream": true}))).into_response();
            }
        }
        // Fallback: no inference — run plan path (cargo check, cargo test)
        let plan = crate::t3::f14(&intent, project.clone(), approuter_dir);
        let exec = crate::t6;
        match tokio::task::block_in_place(|| exec.f15(&plan)) {
            Ok(results) => {
                let summary: Vec<String> = results
                    .iter()
                    .map(|r| format!("{} {} {}", if r.s11 { "✓" } else { "✗" }, r.s10, r.s13))
                    .collect();
                return (StatusCode::OK, Json(serde_json::json!({"message": "Done.", "summary": summary}))).into_response();
            }
            Err(e) => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(serde_json::json!({"error": e.to_string()})),
                )
                    .into_response();
            }
        }
    }
    let plan = crate::t3::f14(&intent, project.clone(), approuter_dir);
    let exec = crate::t6;
    match exec.f15(&plan) {
        Ok(results) => {
            let summary: Vec<String> = results
                .iter()
                .map(|r| format!("{} {} {}", if r.s11 { "✓" } else { "✗" }, r.s10, r.s13))
                .collect();
            (StatusCode::OK, Json(serde_json::json!({"message": "Done.", "summary": summary}))).into_response()
        }
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

async fn api_backlog_post(Json(entry): Json<kova_core::BacklogEntry>) -> impl IntoResponse {
    let path = crate::backlog_path();
    let mut backlog = crate::f25(&path).unwrap_or_default();
    backlog.items.push(entry);
    let json = match serde_json::to_string_pretty(&backlog) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    };
    match std::fs::write(&path, json) {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"ok": true}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

#[cfg(feature = "inference")]
async fn api_explain_run(State(state): State<AppState>) -> impl IntoResponse {
    let trace = state.last_trace.lock().await.clone();
    let trace = match trace {
        Some(t) => t,
        None => {
            return (
                StatusCode::NOT_FOUND,
                Json(serde_json::json!({"error":"No trace. Run a pipeline first."})),
            )
                .into_response()
        }
    };
    let model = match crate::f78(crate::ModelRole::Coder) {
        Some(p) => p,
        None => {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(serde_json::json!({"error":"no model"})),
            )
                .into_response()
        }
    };
    match crate::academy::explain_trace(&trace, &model).await {
        Ok(s) => (StatusCode::OK, Json(serde_json::json!({"explanation": s}))).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e})),
        )
            .into_response(),
    }
}

async fn api_demo_record(Json(payload): Json<serde_json::Value>) -> impl IntoResponse {
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("unnamed");
    let safe = name.replace(['/', '\\', ':', ' '], "_");
    let demo_dir = std::env::var("KOVA_DEMO_DIR")
        .ok()
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".kova")
                .join("demos")
        });
    if let Err(e) = std::fs::create_dir_all(&demo_dir) {
        return (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response();
    }
    let path = demo_dir.join(format!("{}.json", safe));
    let json = match serde_json::to_string_pretty(&payload) {
        Ok(s) => s,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({"error": e.to_string()})),
            )
                .into_response()
        }
    };
    match std::fs::write(&path, json) {
        Ok(()) => (
            StatusCode::OK,
            Json(serde_json::json!({"ok": true, "path": path.display().to_string()})),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({"error": e.to_string()})),
        )
            .into_response(),
    }
}

fn app_router() -> Router<AppState> {
    let r = Router::new()
        .route("/", get(|| async { Html(include_str!("../assets/app.html")) }))
        .route("/api/status", get(|| async { Json(Status { status: "ok" }) }))
        .route("/api/project", get(|| async { Json(serde_json::json!({"project": crate::default_project().display().to_string()})) }))
        .route("/api/projects", get(api_projects))
        .route("/api/prompts", get(api_prompts))
        .route("/api/intent", post(api_intent))
        .route("/ws/stream", get(ws_stream))
        .route("/context/recent", get(recent_changes))
        .route("/build/presets", get(build_presets))
        .route("/build/command", get(build_command))
        .route("/api/file", get(api_file))
        .route("/api/diff", post(api_diff))
        .route("/api/apply", post(api_apply))
        .route("/api/backlog", get(api_backlog_get).post(api_backlog_post))
        .route("/api/backlog/run", post(api_backlog_run))
        .route("/api/demo/record", post(api_demo_record));
    #[cfg(feature = "inference")]
    let r = r
        .route("/api/route", post(api_route))
        .route("/api/explain", get(api_explain))
        .route("/api/explain/run", post(api_explain_run));
    r
}

async fn build_command(Query(q): Query<BuildCommandQuery>) -> Json<BuildCommandResponse> {
    let preset = crate::load_build_preset(&q.project);
    let project_path = crate::default_project();
    let root = crate::workspace_root(&project_path);
    let command = if let Some(p) = preset {
        let mut args = vec!["cargo", "build", "-p", &p.package];
        if q.release {
            args.push("--release");
        }
        if let Some(t) = &p.target {
            args.push("--target");
            args.push(t);
        }
        for f in &p.features {
            args.push("--features");
            args.push(f);
        }
        format!(
            "cd {} && {}",
            root.display(),
            args.join(" ")
        )
    } else {
        let mut args = vec!["cargo", "build"];
        if q.release {
            args.push("--release");
        }
        format!(
            "cd {} && {}",
            project_path.display(),
            args.join(" ")
        )
    };
    Json(BuildCommandResponse { command })
}

pub async fn run(addr: SocketAddr) -> anyhow::Result<()> {
    let state = AppState::default();
    let app = app_router().layer(CorsLayer::permissive()).with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Kova API at http://{}", addr);
    axum::serve(listener, app).await?;
    Ok(())
}

/// Run server and open browser after bind.
pub async fn run_with_open(addr: SocketAddr, url: &str) -> anyhow::Result<()> {
    let state = AppState::default();
    let app = app_router().layer(CorsLayer::permissive()).with_state(state);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    tracing::info!("Kova API at {}", url);

    let (cmd, args): (&str, Vec<&str>) = if cfg!(target_os = "macos") {
        ("open", vec![url])
    } else if cfg!(target_os = "windows") {
        ("cmd", vec!["/C", "start", url])
    } else {
        ("xdg-open", vec![url])
    };
    if let Err(e) = std::process::Command::new(cmd).args(&args).spawn() {
        tracing::warn!("Could not open browser: {}", e);
    }

    axum::serve(listener, app).await?;
    Ok(())
}
