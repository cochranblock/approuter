// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]
//! f118 = serve_rogue_runner_download. Auth-gated binary delivery.

use axum::{
    extract::Query,
    http::{header, StatusCode},
    response::{IntoResponse, Redirect, Response},
};
use axum_extra::extract::CookieJar;

use crate::auth::{session_user_id, SESSION_COOKIE};
use crate::pwa::Assets;

#[derive(serde::Deserialize)]
pub struct t118 {
    pub platform: Option<String>,
}

/// f118 = serve_rogue_runner_download. GET /downloads/rogue-runner?platform=windows|android
pub async fn f118(jar: CookieJar, Query(q): Query<t118>) -> Response {
    let cookie_val = jar.get(SESSION_COOKIE).map(|c| c.value());
    let _user_id = match session_user_id(cookie_val) {
        Some(u) => u,
        None => return Redirect::to("/login?next=/downloads/rogue-runner").into_response(),
    };

    let (path, filename) = match q.platform.as_deref() {
        Some("windows") | Some("exe") => (
            "downloads/rogue-runner-windows-x64.exe",
            "rogue-runner-windows-x64.exe",
        ),
        Some("windows-msi") | Some("msi") => (
            "downloads/rogue-runner-windows-x64.msi",
            "rogue-runner-windows-x64.msi",
        ),
        Some("android") | Some("apk") => ("downloads/rogue-runner.apk", "rogue-runner.apk"),
        _ => {
            return (
                StatusCode::BAD_REQUEST,
                "?platform=windows|windows-msi|android required",
            )
                .into_response()
        }
    };

    let body = match Assets::get(path) {
        Some(f) => f.data.into_owned(),
        None => {
            return (StatusCode::NOT_FOUND, format!("Binary not found: {}", path)).into_response()
        }
    };

    (
        [
            (header::CONTENT_TYPE, "application/octet-stream"),
            (
                header::CONTENT_DISPOSITION,
                format!("attachment; filename=\"{}\"", filename).as_str(),
            ),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        body,
    )
        .into_response()
}
