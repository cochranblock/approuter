// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]
//! rogue-repo: f0=main, f1=buy_bucks, f2=provision_app, f3=add_device

use rogue_repo::{auth, downloads, pwa, routes, vault};
use approuter::{f116, RegisterConfig};
use axum::{
    routing::{get, post},
    Router,
};
use std::net::SocketAddr;
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let pool = std::env::var("DATABASE_URL").ok().and_then(|url| {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                sqlx::postgres::PgPoolOptions::new()
                    .max_connections(5)
                    .connect(&url)
                    .await
                    .ok()
            })
        })
    });

    if pool.is_none() {
        tracing::warn!("DATABASE_URL not set or invalid; auth disabled");
    }

    let p0 = routes::t0 { s0: pool };
    let app = Router::new()
        .route("/", get(routes::f4))
        .route("/login", get(pwa::f102).post(auth::f98))
        .route("/register", get(pwa::f103).post(auth::f97))
        .route("/verify-email", get(auth::f100))
        .route("/logout", axum::routing::post(auth::f101).get(auth::f101))
        .route("/manifest.json", get(pwa::f92))
        .route("/sw.js", get(pwa::f93))
        .route("/assets/*path", get(pwa::f91))
        .route("/apps/rogue-runner", get(pwa::f94))
        .route("/apps/rogue-runner-wasm", get(pwa::f95))
        .route("/apps/null-terminal", get(pwa::f117))
        .route("/health", get(routes::f5))
        .route("/buy-bucks", post(routes::f87))
        .route("/provision-app", post(routes::f88))
        .route("/add-device", post(routes::f89))
        .route("/downloads/rogue-runner", get(downloads::f118))
        .with_state(p0)
        .layer(CompressionLayer::new())
        .layer(CorsLayer::new().allow_origin(Any).allow_methods(Any));

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    f116(RegisterConfig {
        app_id: "roguerepo",
        hostnames: std::env::var("REPO_HOSTNAMES")
            .unwrap_or_else(|_| "roguerepo.io,www.roguerepo.io".into())
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        backend_url: std::env::var("REPO_BACKEND_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:3001".into()),
    })
    .await;
    tracing::info!("Rogue Repo API on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
