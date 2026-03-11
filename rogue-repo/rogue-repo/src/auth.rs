// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Auth: register, login, email verification. f97=register f98=login f99=verify_email

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use axum::{
    extract::State,
    response::{IntoResponse, Redirect, Response},
    Form, Json,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use base64::Engine;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use uuid::Uuid;

use crate::routes::{t0, t84};

pub const SESSION_COOKIE: &str = "rr_session";
const SESSION_MAX_AGE: i64 = 86400 * 7; // 7 days

/// t97 = RegisterForm
#[derive(Deserialize)]
pub struct t97 {
    pub email: String,
    pub password: String,
}

/// t98 = LoginForm
#[derive(Deserialize)]
pub struct t98 {
    pub email: String,
    pub password: String,
}

/// t99 = AuthRes
#[derive(Serialize)]
pub struct t99 {
    pub ok: bool,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Uuid>,
}

fn hash_password(p: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    Argon2::default()
        .hash_password(p.as_bytes(), &salt)
        .map_err(|e| e.to_string())
        .map(|h| h.to_string())
}

fn verify_password(hash: &str, password: &str) -> bool {
    let parsed = match PasswordHash::new(hash) {
        Ok(p) => p,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok()
}

fn make_token() -> String {
    use rand::Rng;
    let mut b = [0u8; 32];
    rand::thread_rng().fill(&mut b);
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, b)
}

fn sign_session(user_id: Uuid, secret: &[u8]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let exp = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64)
        + SESSION_MAX_AGE;
    let payload = format!("{}:{}", user_id, exp);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("hmac");
    mac.update(payload.as_bytes());
    let sig = mac.finalize().into_bytes();
    let sig_b64 = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, sig);
    format!("{}.{}", payload, sig_b64)
}

pub fn session_user_id(cookie_val: Option<&str>) -> Option<Uuid> {
    let val = cookie_val?;
    let secret =
        std::env::var("SESSION_SECRET").unwrap_or_else(|_| "dev-secret-change-in-prod".into());
    verify_session(val, secret.as_bytes())
}

fn verify_session(cookie_val: &str, secret: &[u8]) -> Option<Uuid> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut parts = cookie_val.splitn(2, '.');
    let payload = parts.next()?;
    let sig_b64 = parts.next()?;
    let sig = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(sig_b64)
        .ok()?;
    let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("hmac");
    mac.update(payload.as_bytes());
    mac.verify_slice(&sig).ok()?;
    let mut p = payload.splitn(2, ':');
    let user_id = p.next()?.parse::<Uuid>().ok()?;
    let exp: i64 = p.next()?.parse().ok()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    if exp < now {
        return None;
    }
    Some(user_id)
}

/// f97 = register, POST /register — redirects on success
pub async fn f97(
    State(p0): State<t0>,
    Form(form): Form<t97>,
) -> Result<Response, (axum::http::StatusCode, Redirect)> {
    let pool = p0.s0.as_ref().ok_or((
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        Redirect::to("/register?error=db"),
    ))?;

    let email = form.email.trim().to_lowercase();
    if email.is_empty() || form.password.len() < 8 {
        return Err((
            axum::http::StatusCode::BAD_REQUEST,
            Redirect::to("/register?error=invalid"),
        ));
    }

    let hash = hash_password(&form.password).map_err(|_| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Redirect::to("/register?error=hash"),
        )
    })?;

    let token = make_token();
    let expires = chrono::Utc::now() + chrono::Duration::hours(24);

    let row = sqlx::query_as::<_, (Uuid,)>(
        "INSERT INTO users (email, rogue_bucks_balance, password_hash, verification_token, verification_token_expires_at)
         VALUES ($1, 0, $2, $3, $4)
         ON CONFLICT (email) DO UPDATE SET
           password_hash = EXCLUDED.password_hash,
           verification_token = EXCLUDED.verification_token,
           verification_token_expires_at = EXCLUDED.verification_token_expires_at,
           email_verified_at = NULL
         RETURNING id",
    )
    .bind(&email)
    .bind(&hash)
    .bind(&token)
    .bind(expires)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        tracing::error!("register db: {}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Redirect::to("/register?error=db"),
        )
    })?;

    let (_user_id,) = row.ok_or((
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        Redirect::to("/register?error=db"),
    ))?;

    let base_url =
        std::env::var("REPO_BASE_URL").unwrap_or_else(|_| "http://localhost:3001".into());
    let verify_url = format!("{}/verify-email?token={}", base_url, token);

    if let Ok(api_key) = std::env::var("RESEND_API_KEY") {
        let client = reqwest::Client::new();
        let body = serde_json::json!({
            "from": std::env::var("RESEND_FROM").unwrap_or_else(|_| "Rogue Repo <onboarding@resend.dev>".into()),
            "to": [&email],
            "subject": "Verify your Rogue Repo email",
            "html": format!(
                r#"<p>Click to verify your email:</p><p><a href="{}">{}</a></p><p>Link expires in 24 hours.</p>"#,
                verify_url, verify_url
            )
        });
        match client
            .post("https://api.resend.com/emails")
            .header("Authorization", format!("Bearer {}", api_key))
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await
        {
            Ok(r) if r.status().is_success() => {
                tracing::info!("Verification email sent to {}", email)
            }
            Ok(r) => tracing::warn!("Resend failed {}: {:?}", r.status(), r.text().await.ok()),
            Err(e) => tracing::warn!("Resend request failed: {}", e),
        }
    } else {
        tracing::info!("Verification link (RESEND_API_KEY not set): {}", verify_url);
    }

    Ok(Redirect::to("/login?registered=1").into_response())
}

/// f98 = login, POST /login
pub async fn f98(
    State(p0): State<t0>,
    Form(form): Form<t98>,
) -> Result<Response, (axum::http::StatusCode, Json<t99>)> {
    let pool = p0.s0.as_ref().ok_or((
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        Json(t99 {
            ok: false,
            message: "Database not configured".into(),
            user_id: None,
        }),
    ))?;

    let email = form.email.trim().to_lowercase();
    let row = sqlx::query_as::<_, (Uuid, String, Option<chrono::DateTime<chrono::Utc>>)>(
        "SELECT id, password_hash, email_verified_at FROM users WHERE email = $1",
    )
    .bind(&email)
    .fetch_optional(pool)
    .await
    .map_err(|e| {
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(t99 {
                ok: false,
                message: format!("DB: {}", e),
                user_id: None,
            }),
        )
    })?;

    let (user_id, hash, verified) = row.ok_or((
        axum::http::StatusCode::UNAUTHORIZED,
        Json(t99 {
            ok: false,
            message: "Invalid email or password".into(),
            user_id: None,
        }),
    ))?;

    if hash.is_empty() || !verify_password(&hash, &form.password) {
        return Err((
            axum::http::StatusCode::UNAUTHORIZED,
            Json(t99 {
                ok: false,
                message: "Invalid email or password".into(),
                user_id: None,
            }),
        ));
    }

    if verified.is_none() {
        return Err((
            axum::http::StatusCode::FORBIDDEN,
            Json(t99 {
                ok: false,
                message: "Please verify your email first".into(),
                user_id: None,
            }),
        ));
    }

    let secret =
        std::env::var("SESSION_SECRET").unwrap_or_else(|_| "dev-secret-change-in-prod".into());
    let session = sign_session(user_id, secret.as_bytes());

    let cookie = Cookie::build((SESSION_COOKIE, session))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::seconds(SESSION_MAX_AGE))
        .build();

    Ok(([("Set-Cookie", cookie.to_string())], Redirect::to("/")).into_response())
}

/// f100 = verify_email, GET /verify-email?token=...
pub async fn f100(
    State(p0): State<t0>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Response {
    let pool = match &p0.s0 {
        Some(p) => p,
        None => return Redirect::to("/login?error=db").into_response(),
    };

    let token = match params.get("token") {
        Some(t) if !t.is_empty() => t.clone(),
        _ => return Redirect::to("/login?error=missing_token").into_response(),
    };

    let row = sqlx::query_as::<_, (Uuid,)>(
        "UPDATE users SET email_verified_at = NOW(), verification_token = NULL, verification_token_expires_at = NULL
         WHERE verification_token = $1 AND (verification_token_expires_at IS NULL OR verification_token_expires_at > NOW())
         RETURNING id",
    )
    .bind(&token)
    .fetch_optional(pool)
    .await;

    match row {
        Ok(Some(_)) => Redirect::to("/login?verified=1").into_response(),
        _ => Redirect::to("/login?error=invalid_token").into_response(),
    }
}

/// f101 = logout, POST /logout (GET also accepted for link convenience)
pub async fn f101() -> Response {
    let cookie = Cookie::build((SESSION_COOKIE, ""))
        .path("/")
        .http_only(true)
        .max_age(time::Duration::seconds(0))
        .build();
    ([("Set-Cookie", cookie.to_string())], Redirect::to("/")).into_response()
}
