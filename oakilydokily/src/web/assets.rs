// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

/// f109 = serve. GET /assets/*path. Static files via RustEmbed.
pub async fn serve(Path(path): Path<String>) -> impl IntoResponse {
    let path = path.trim_start_matches('/');
    match Assets::get(path) {
        Some(file) => {
            let mime = if path.ends_with(".svg") {
                axum::http::header::HeaderValue::from_static("image/svg+xml")
            } else if path.ends_with(".wasm") {
                axum::http::header::HeaderValue::from_static("application/wasm")
            } else {
                axum::http::header::HeaderValue::from_str(
                    mime_guess::from_path(path).first_or_octet_stream().as_ref(),
                )
                .unwrap_or(axum::http::header::HeaderValue::from_static(
                    "application/octet-stream",
                ))
            };
            (
                [(axum::http::header::CONTENT_TYPE, mime)],
                file.data.into_owned(),
            )
                .into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
