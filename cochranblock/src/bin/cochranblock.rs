// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

//! CochranBlock server. Registers with approuter when `approuter` feature enabled. Serves cochranblock.org.

use cochranblock::t0;
use cochranblock::web::router;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Load .env from deterministic path. Why: cwd may differ when run via approuter.
    let _ = dotenvy::dotenv();
    if let Ok(root) = std::env::var("COCHRANBLOCK_ROOT") {
        let p = std::path::Path::new(&root).join(".env");
        let _ = dotenvy::from_path_override(&p);
    }
    let _ = dotenvy::from_path_override("cochranblock/.env");
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")))
        .init();

    let port: u16 = std::env::var("PORT").ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8081);
    let bind = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0".into());
    let addr = format!("{}:{}", bind, port);

    let app = router::f1(t0 {});
    #[cfg(feature = "approuter")]
    approuter_client::f116(approuter_client::RegisterConfig {
        app_id: "cochranblock",
        hostnames: std::env::var("CB_HOSTNAMES")
            .unwrap_or_else(|_| "cochranblock.org,www.cochranblock.org".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        backend_url: std::env::var("CB_BACKEND_URL")
            .unwrap_or_else(|_| format!("http://127.0.0.1:{}", port)),
    })
    .await;

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("cochranblock listening on http://{}", addr);

    let shutdown = async {
        tokio::signal::ctrl_c().await.ok();
        tracing::info!("shutting down");
    };
    axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown)
        .await?;
    Ok(())
}
