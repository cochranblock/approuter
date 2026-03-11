// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use axum::{routing::get, Router};
use axum::http::header::{HeaderName, HeaderValue};
use tower_http::{
    compression::CompressionLayer,
    set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};

use crate::t0;
use super::{pages, assets};

/// f1 = app_router. Why: Single router with compression, trace, security headers; state shared via Arc.
pub fn f1(p0: t0) -> Router {
    let p0: std::sync::Arc<t0> = std::sync::Arc::new(p0);
    let h1 = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-content-type-options"),
        HeaderValue::from_static("nosniff"),
    );
    let h2 = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("x-frame-options"),
        HeaderValue::from_static("SAMEORIGIN"),
    );
    let h3 = SetResponseHeaderLayer::overriding(
        HeaderName::from_static("referrer-policy"),
        HeaderValue::from_static("strict-origin-when-cross-origin"),
    );
    let r0 = Router::new()
        .route("/", get(pages::f2))
        .route("/services", get(pages::f11))
        .route("/about", get(pages::f12))
        .route("/contact", get(pages::f13))
        .route("/book", get(pages::f63))
        .route("/products", get(pages::f67))
        .route("/federal-partners", get(pages::f68))
        .route("/health", get(pages::f10))
        .route("/robots.txt", get(pages::f69))
        .route("/sitemap.xml", get(pages::f70))
        .route("/assets/*path", get(assets::f23));
    #[cfg(feature = "dev")]
    let r0 = r0
        .route("/dev/source", get(pages::f57))
        .route("/dev/exec-summary", get(pages::f58))
        .route("/dev/rules", get(pages::f59))
        .route("/dev/ai-orchestration", get(pages::f65))
        .route("/dev/prompts", get(pages::f66));
    r0
        .fallback(pages::f71)
        .layer(h1)
        .layer(h2)
        .layer(h3)
        .layer(CompressionLayer::new().zstd(true))
        .layer(TraceLayer::new_for_http())
        .with_state(p0)
}
