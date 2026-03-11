// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::http::StatusCode;
use include_packed::include_packed;

/// f23 = serve_static — include_packed assets (zstd)
fn t15_get(p0: &str) -> Option<Vec<u8>> {
    let v0 = match p0 {
        "css/main.css" => include_packed!("assets/css/main.css"),
        "js/booking.js" => include_packed!("assets/js/booking.js"),
        "js/calendar.js" => include_packed!("assets/js/calendar.js"),
        "js/main.js" => include_packed!("assets/js/main.js"),
        "favicon.svg" => include_packed!("assets/favicon.svg"),
        "cochranblock-logo.svg" => include_packed!("assets/cochranblock-logo.svg"),
        "cochranblock-hero-logo.svg" => include_packed!("assets/cochranblock-hero-logo.svg"),
        "img/kova.png" => include_packed!("assets/img/kova.png"),
        "img/rogue-repo.png" => include_packed!("assets/img/rogue-repo.png"),
        "img/ronin-sites.png" => include_packed!("assets/img/ronin-sites.png"),
        "resume.pdf" => include_packed!("assets/resume.pdf"),
        _ => return None,
    };
    Some(v0)
}

pub async fn f23(Path(p1): Path<String>) -> impl IntoResponse {
    let v0 = p1.trim_start_matches('/');
    match t15_get(v0) {
        Some(v1) => {
            let v2 = mime_guess::from_path(v0).first_or_octet_stream();
            ([(axum::http::header::CONTENT_TYPE, v2.as_ref())], v1).into_response()
        }
        None => StatusCode::NOT_FOUND.into_response(),
    }
}
