// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use axum::{routing::get, Router};
use axum::response::Redirect;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

use super::{assets, auth, pages, waiver};
use crate::AppState;

/// f1 = router. Why: Single entry for all OD routes; state shared via Arc.
pub fn f1(state: AppState) -> Router {
    let state = std::sync::Arc::new(state);
    Router::new()
        .route("/", get(pages::home))
        .route("/about", get(pages::about))
        .route("/contact", get(pages::contact))
        .route("/waiver", get(waiver::f74).post(waiver::f75))
        .route("/waiver/confirmed", get(waiver::confirmed))
        .route("/auth/google", get(auth::f82))
        .route("/auth/google/callback", get(auth::f83))
        .route("/auth/facebook", get(auth::f98))
        .route("/auth/facebook/callback", get(auth::f99))
        .route("/auth/apple", get(auth::f91))
        .route("/auth/apple/callback", get(auth::f92))
        .route("/auth/login", get(auth::f100).post(auth::f101))
        .route("/auth/logout", get(auth::f84))
        .route("/health", get(pages::health))
        .route("/favicon.ico", get(|| async { Redirect::permanent("/assets/favicon.svg") }))
        .route("/sitemap.xml", get(pages::sitemap))
        .route("/assets/*path", get(assets::serve))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}
pub use f1 as router;
