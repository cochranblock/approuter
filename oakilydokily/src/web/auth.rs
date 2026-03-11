// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Sign in with Google + Facebook OAuth + manual (email/password). f82/f83=Google f98/f99=Facebook f100/f101=manual f84=logout

#![allow(non_camel_case_types, non_snake_case, dead_code)]

use axum::extract::{Form, Query, State};
use axum::response::{Html, IntoResponse, Redirect};
use axum_extra::extract::cookie::{Cookie, CookieJar, SameSite};
use std::sync::Arc;

use super::head;
use crate::AppState;
use base64::Engine;
use serde::Deserialize;
use sha2::{Digest, Sha256};

const SESSION_COOKIE: &str = "od_session";
const STATE_COOKIE: &str = "od_oauth_state";
const REDIRECT_COOKIE: &str = "od_redirect";
const SESSION_MAX_AGE: i64 = 86400 * 7; // 7 days

/// f95=safe_redirect_path. Validates redirect param (starts with /, no protocol).
fn f95(redirect: &str) -> Option<String> {
    let r = redirect.trim();
    if r.starts_with('/') && !r.starts_with("//") && !r.contains("://") {
        Some(r.to_string())
    } else {
        None
    }
}

/// f94 = is_https. OD_BASE_URL starts with https://.
fn f94() -> bool {
    std::env::var("OD_BASE_URL")
        .unwrap_or_default()
        .trim()
        .to_lowercase()
        .starts_with("https://")
}

/// t82=GoogleUser. From userinfo.
#[derive(Debug, Deserialize)]
pub struct t82 {
    pub email: String,
    pub name: Option<String>,
}

/// f85 = oauth_state. Random 24-byte base64 for CSRF.
fn f85() -> String {
    use rand::Rng;
    let mut b = [0u8; 24];
    rand::thread_rng().fill(&mut b);
    base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, b)
}

/// f86 = session_cookie_value. HMAC-SHA256 payload: email|name|exp.
fn f86(email: &str, name: &str, secret: &[u8]) -> String {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let exp = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64)
        + SESSION_MAX_AGE;
    let payload = format!("{}|{}|{}", email, name.replace('|', "_"), exp);
    let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("hmac");
    mac.update(payload.as_bytes());
    let sig = mac.finalize().into_bytes();
    let sig_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(sig);
    format!("{}.{}", payload, sig_b64)
}

/// f87 = parse_session. Verify HMAC, return (email, name) or None.
fn f87(cookie_val: &str, secret: &[u8]) -> Option<(String, String)> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let (payload, sig_b64) = cookie_val.split_once('.')?;
    let sig = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(sig_b64)
        .ok()?;
    let mut mac = Hmac::<Sha256>::new_from_slice(secret).expect("hmac");
    mac.update(payload.as_bytes());
    mac.verify_slice(&sig).ok()?;
    let mut p = payload.splitn(3, '|');
    let email = p.next()?.to_string();
    let name = p.next()?.replace('_', "|");
    let exp: i64 = p.next()?.parse().ok()?;
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;
    if exp < now {
        return None;
    }
    Some((email, name))
}

/// f82 = google_redirect (GET /auth/google)
pub async fn f82(jar: CookieJar) -> impl IntoResponse {
    let client_id = match std::env::var("GOOGLE_CLIENT_ID") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            tracing::warn!("GOOGLE_CLIENT_ID not set, redirecting to /");
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let base_url = std::env::var("OD_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let redirect_uri = format!("{}/auth/google/callback", base_url.trim_end_matches('/'));
    let state = f85();
    let auth_url = format!(
        "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=openid%20email%20profile&state={}&access_type=offline&prompt=consent",
        urlencoding::encode(&client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&state)
    );
    let mut state_cookie = Cookie::build((STATE_COOKIE, state.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::minutes(10));
    if f94() {
        state_cookie = state_cookie.secure(true);
    }
    let jar = jar.add(state_cookie.build());
    (jar, Redirect::temporary(&auth_url)).into_response()
}

/// t83=OAuthCallbackQuery. code, state, error.
#[derive(Deserialize)]
pub struct t83 {
    pub code: Option<String>,
    pub state: Option<String>,
    pub error: Option<String>,
}

/// f83 = google_callback (GET /auth/google/callback)
pub async fn f83(jar: CookieJar, Query(q): Query<t83>) -> impl IntoResponse {
    if let Some(err) = &q.error {
        tracing::warn!("Google OAuth error: {}", err);
        let jar = jar.remove(Cookie::from(STATE_COOKIE));
        return (jar, Redirect::to("/")).into_response();
    }
    let (code, state) = match (&q.code, &q.state) {
        (Some(c), Some(s)) => (c.clone(), s.clone()),
        _ => {
            let jar = jar.remove(Cookie::from(STATE_COOKIE));
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let stored_state = jar.get(STATE_COOKIE).map(|c| c.value().to_string());
    let jar = jar.remove(Cookie::from(STATE_COOKIE));
    if stored_state.as_deref() != Some(&state) {
        tracing::warn!("OAuth state mismatch");
        return (jar, Redirect::to("/")).into_response();
    }
    let user: t82 = if std::env::var("OD_TEST_MOCK_OAUTH")
        .map(|v| v.trim() == "1")
        .unwrap_or(false)
        && code == "__mock__"
    {
        t82 {
            email: "oauth-test@example.com".into(),
            name: Some("OAuth Test User".into()),
        }
    } else {
        let client_id = std::env::var("GOOGLE_CLIENT_ID").unwrap_or_default();
        let client_secret = std::env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default();
        let base_url =
            std::env::var("OD_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
        let redirect_uri = format!("{}/auth/google/callback", base_url.trim_end_matches('/'));
        if client_id.is_empty() || client_secret.is_empty() {
            tracing::warn!("Google OAuth not configured");
            return (jar, Redirect::to("/")).into_response();
        }
        let token_url = std::env::var("OD_OAUTH2_TOKEN_URL")
            .unwrap_or_else(|_| "https://oauth2.googleapis.com/token".into());
        let token_resp = reqwest::blocking::Client::new()
            .post(&token_url)
            .form(&[
                ("code", code.as_str()),
                ("client_id", &client_id),
                ("client_secret", &client_secret),
                ("redirect_uri", &redirect_uri),
                ("grant_type", "authorization_code"),
            ])
            .timeout(std::time::Duration::from_secs(10))
            .send();
        let token_body = match token_resp {
            Ok(r) if r.status().is_success() => match r.json::<serde_json::Value>() {
                Ok(j) => j,
                Err(e) => {
                    tracing::warn!("token parse: {}", e);
                    return (jar, Redirect::to("/")).into_response();
                }
            },
            Ok(r) => {
                tracing::warn!("token exchange failed {}: {:?}", r.status(), r.text().ok());
                return (jar, Redirect::to("/")).into_response();
            }
            Err(e) => {
                tracing::warn!("token request: {}", e);
                return (jar, Redirect::to("/")).into_response();
            }
        };
        let access_token = match token_body.get("access_token").and_then(|v| v.as_str()) {
            Some(t) => t.to_string(),
            None => {
                tracing::warn!("no access_token in response");
                return (jar, Redirect::to("/")).into_response();
            }
        };
        let userinfo_url = std::env::var("OD_OPENID_USERINFO_URL")
            .unwrap_or_else(|_| "https://openidconnect.googleapis.com/v1/userinfo".into());
        let user_resp = reqwest::blocking::Client::new()
            .get(&userinfo_url)
            .bearer_auth(&access_token)
            .timeout(std::time::Duration::from_secs(5))
            .send();
        match user_resp {
            Ok(r) if r.status().is_success() => match r.json() {
                Ok(u) => u,
                Err(e) => {
                    tracing::warn!("userinfo parse: {}", e);
                    return (jar, Redirect::to("/")).into_response();
                }
            },
            _ => {
                tracing::warn!("userinfo request failed");
                return (jar, Redirect::to("/")).into_response();
            }
        }
    };
    let email = user.email.trim().to_lowercase();
    let name = user.name.as_deref().unwrap_or("").trim().to_string();
    if email.is_empty() {
        tracing::warn!("Google user has no email");
        return (jar, Redirect::to("/")).into_response();
    }
    let secret = std::env::var("SESSION_SECRET").unwrap_or_default();
    if secret.len() < 32 {
        tracing::warn!("SESSION_SECRET too short (need 32+ chars)");
        return (jar, Redirect::to("/")).into_response();
    }
    let session_val = f86(&email, &name, secret.as_bytes());
    let mut session_cookie = Cookie::build((SESSION_COOKIE, session_val))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(7));
    if f94() {
        session_cookie = session_cookie.secure(true);
    }
    let jar = jar.add(session_cookie.build());
    let (redirect, jar) = f96(jar);
    (jar, redirect).into_response()
}

/// f98 = facebook_redirect (GET /auth/facebook)
pub async fn f98(jar: CookieJar) -> impl IntoResponse {
    let app_id = match std::env::var("FB_APP_ID") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            tracing::warn!("FB_APP_ID not set, redirecting to /");
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let base_url = std::env::var("OD_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let redirect_uri = format!("{}/auth/facebook/callback", base_url.trim_end_matches('/'));
    let state = f85();
    let auth_url = format!(
        "https://www.facebook.com/v21.0/dialog/oauth?client_id={}&redirect_uri={}&state={}&scope=email,public_profile",
        urlencoding::encode(&app_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&state)
    );
    let mut state_cookie = Cookie::build((STATE_COOKIE, state.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::minutes(10));
    if f94() {
        state_cookie = state_cookie.secure(true);
    }
    let jar = jar.add(state_cookie.build());
    (jar, Redirect::temporary(&auth_url)).into_response()
}

/// f99 = facebook_callback (GET /auth/facebook/callback)
pub async fn f99(jar: CookieJar, Query(q): Query<t83>) -> impl IntoResponse {
    if let Some(err) = &q.error {
        tracing::warn!("Facebook OAuth error: {}", err);
        let jar = jar.remove(Cookie::from(STATE_COOKIE));
        return (jar, Redirect::to("/")).into_response();
    }
    let (code, state) = match (&q.code, &q.state) {
        (Some(c), Some(s)) => (c.clone(), s.clone()),
        _ => {
            let jar = jar.remove(Cookie::from(STATE_COOKIE));
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let stored_state = jar.get(STATE_COOKIE).map(|c| c.value().to_string());
    let jar = jar.remove(Cookie::from(STATE_COOKIE));
    if stored_state.as_deref() != Some(&state) {
        tracing::warn!("Facebook OAuth state mismatch");
        return (jar, Redirect::to("/")).into_response();
    }
    let app_id = std::env::var("FB_APP_ID").unwrap_or_default();
    let app_secret = std::env::var("FB_APP_SECRET").unwrap_or_default();
    let base_url = std::env::var("OD_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let redirect_uri = format!("{}/auth/facebook/callback", base_url.trim_end_matches('/'));
    if app_id.is_empty() || app_secret.is_empty() {
        tracing::warn!("Facebook OAuth not configured");
        return (jar, Redirect::to("/")).into_response();
    }
    let token_url = format!(
        "https://graph.facebook.com/v21.0/oauth/access_token?client_id={}&redirect_uri={}&client_secret={}&code={}",
        urlencoding::encode(&app_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&app_secret),
        urlencoding::encode(&code)
    );
    let token_resp = reqwest::blocking::Client::new()
        .get(&token_url)
        .timeout(std::time::Duration::from_secs(10))
        .send();
    let token_body = match token_resp {
        Ok(r) if r.status().is_success() => match r.json::<serde_json::Value>() {
            Ok(j) => j,
            Err(e) => {
                tracing::warn!("Facebook token parse: {}", e);
                return (jar, Redirect::to("/")).into_response();
            }
        },
        Ok(r) => {
            tracing::warn!(
                "Facebook token exchange failed {}: {:?}",
                r.status(),
                r.text().ok()
            );
            return (jar, Redirect::to("/")).into_response();
        }
        Err(e) => {
            tracing::warn!("Facebook token request: {}", e);
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let access_token = match token_body.get("access_token").and_then(|v| v.as_str()) {
        Some(t) => t.to_string(),
        None => {
            tracing::warn!("Facebook response missing access_token");
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let me_url = format!(
        "https://graph.facebook.com/me?fields=id,name,email&access_token={}",
        urlencoding::encode(&access_token)
    );
    let user_resp = reqwest::blocking::Client::new()
        .get(&me_url)
        .timeout(std::time::Duration::from_secs(5))
        .send();
    let user: t82 = match user_resp {
        Ok(r) if r.status().is_success() => match r.json() {
            Ok(u) => u,
            Err(e) => {
                tracing::warn!("Facebook userinfo parse: {}", e);
                return (jar, Redirect::to("/")).into_response();
            }
        },
        _ => {
            tracing::warn!("Facebook userinfo request failed");
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let email = user.email.trim().to_lowercase();
    let name = user.name.as_deref().unwrap_or("").trim().to_string();
    if email.is_empty() {
        tracing::warn!("Facebook user has no email");
        return (jar, Redirect::to("/")).into_response();
    }
    let secret = std::env::var("SESSION_SECRET").unwrap_or_default();
    if secret.len() < 32 {
        tracing::warn!("SESSION_SECRET too short (need 32+ chars)");
        return (jar, Redirect::to("/")).into_response();
    }
    let session_val = f86(&email, &name, secret.as_bytes());
    let mut session_cookie = Cookie::build((SESSION_COOKIE, session_val))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(7));
    if f94() {
        session_cookie = session_cookie.secure(true);
    }
    let jar = jar.add(session_cookie.build());
    let (redirect, jar) = f96(jar);
    (jar, redirect).into_response()
}

/// f91 = apple_redirect (GET /auth/apple) — kept for backwards compat, deprecated
pub async fn f91(jar: CookieJar) -> impl IntoResponse {
    let client_id = match std::env::var("APPLE_CLIENT_ID") {
        Ok(k) if !k.is_empty() => k,
        _ => {
            tracing::warn!("APPLE_CLIENT_ID not set, redirecting to /");
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let base_url = std::env::var("OD_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let redirect_uri = format!("{}/auth/apple/callback", base_url.trim_end_matches('/'));
    let state = f85();
    let auth_url = format!(
        "https://appleid.apple.com/auth/authorize?client_id={}&redirect_uri={}&response_type=code&response_mode=query&scope=name%20email&state={}",
        urlencoding::encode(&client_id),
        urlencoding::encode(&redirect_uri),
        urlencoding::encode(&state)
    );
    let mut state_cookie = Cookie::build((STATE_COOKIE, state.clone()))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::minutes(10));
    if f94() {
        state_cookie = state_cookie.secure(true);
    }
    let jar = jar.add(state_cookie.build());
    (jar, Redirect::temporary(&auth_url)).into_response()
}

/// f92 = apple_callback (GET /auth/apple/callback)
pub async fn f92(jar: CookieJar, Query(q): Query<t83>) -> impl IntoResponse {
    if let Some(err) = &q.error {
        tracing::warn!("Apple OAuth error: {}", err);
        let jar = jar.remove(Cookie::from(STATE_COOKIE));
        return (jar, Redirect::to("/")).into_response();
    }
    let (code, state) = match (&q.code, &q.state) {
        (Some(c), Some(s)) => (c.clone(), s.clone()),
        _ => {
            let jar = jar.remove(Cookie::from(STATE_COOKIE));
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let stored_state = jar.get(STATE_COOKIE).map(|c| c.value().to_string());
    let jar = jar.remove(Cookie::from(STATE_COOKIE));
    if stored_state.as_deref() != Some(&state) {
        tracing::warn!("Apple OAuth state mismatch");
        return (jar, Redirect::to("/")).into_response();
    }
    let client_id = std::env::var("APPLE_CLIENT_ID").unwrap_or_default();
    let client_secret = std::env::var("APPLE_CLIENT_SECRET").unwrap_or_default();
    let base_url = std::env::var("OD_BASE_URL").unwrap_or_else(|_| "http://127.0.0.1:3000".into());
    let redirect_uri = format!("{}/auth/apple/callback", base_url.trim_end_matches('/'));
    if client_id.is_empty() || client_secret.is_empty() {
        tracing::warn!("Apple OAuth not configured");
        return (jar, Redirect::to("/")).into_response();
    }
    let token_resp = reqwest::blocking::Client::new()
        .post("https://appleid.apple.com/auth/token")
        .form(&[
            ("code", code.as_str()),
            ("client_id", &client_id),
            ("client_secret", &client_secret),
            ("redirect_uri", &redirect_uri),
            ("grant_type", "authorization_code"),
        ])
        .header("Content-Type", "application/x-www-form-urlencoded")
        .timeout(std::time::Duration::from_secs(10))
        .send();
    let token_body = match token_resp {
        Ok(r) if r.status().is_success() => match r.json::<serde_json::Value>() {
            Ok(j) => j,
            Err(e) => {
                tracing::warn!("Apple token parse: {}", e);
                return (jar, Redirect::to("/")).into_response();
            }
        },
        Ok(r) => {
            tracing::warn!(
                "Apple token exchange failed {}: {:?}",
                r.status(),
                r.text().ok()
            );
            return (jar, Redirect::to("/")).into_response();
        }
        Err(e) => {
            tracing::warn!("Apple token request: {}", e);
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let id_token = match token_body.get("id_token").and_then(|v| v.as_str()) {
        Some(t) => t,
        None => {
            tracing::warn!("Apple response missing id_token");
            return (jar, Redirect::to("/")).into_response();
        }
    };
    let (email, name) = match f93(id_token) {
        Some((e, n)) => (e, n),
        None => {
            tracing::warn!("Apple id_token decode failed");
            return (jar, Redirect::to("/")).into_response();
        }
    };
    if email.is_empty() {
        tracing::warn!("Apple user has no email in id_token");
        return (jar, Redirect::to("/")).into_response();
    }
    let secret = std::env::var("SESSION_SECRET").unwrap_or_default();
    if secret.len() < 32 {
        tracing::warn!("SESSION_SECRET too short (need 32+ chars)");
        return (jar, Redirect::to("/")).into_response();
    }
    let session_val = f86(&email, &name, secret.as_bytes());
    let mut session_cookie = Cookie::build((SESSION_COOKIE, session_val))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(7));
    if f94() {
        session_cookie = session_cookie.secure(true);
    }
    let jar = jar.add(session_cookie.build());
    (jar, Redirect::to("/")).into_response()
}

/// f93 = decode_apple_id_token. Returns (email, name). Name may be empty.
fn f93(id_token: &str) -> Option<(String, String)> {
    let parts: Vec<&str> = id_token.splitn(3, '.').collect();
    if parts.len() != 3 {
        return None;
    }
    let payload_b64 = parts[1];
    let payload_bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .decode(payload_b64)
        .ok()?;
    let payload: serde_json::Value = serde_json::from_slice(&payload_bytes).ok()?;
    let sub = payload.get("sub")?.as_str()?.to_string();
    let email = payload
        .get("email")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_lowercase();
    let name = payload
        .get("name")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .trim()
        .to_string();
    let email = if email.is_empty() {
        let s: &str = sub.as_ref();
        format!("{}@privaterelay.appleid.com", &s[..s.len().min(16)])
    } else {
        email
    };
    Some((
        email,
        if name.is_empty() {
            "Apple user".into()
        } else {
            name
        },
    ))
}

/// f100=login_page. GET /auth/login. OAuth + manual form. Create account → /auth/google.
pub async fn f100(
    jar: CookieJar,
    Query(q): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    if f88(&jar).is_some() {
        let (redirect, jar) = f96(jar);
        return (jar, redirect).into_response();
    }
    let mut jar = jar;
    if let Some(redirect) = q.get("redirect").and_then(|r| f95(r)) {
        let mut c = Cookie::build((REDIRECT_COOKIE, redirect))
            .path("/")
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(time::Duration::minutes(10));
        if f94() {
            c = c.secure(true);
        }
        jar = jar.add(c.build());
    }
    let google_ok = std::env::var("GOOGLE_CLIENT_ID")
        .map(|k| !k.trim().is_empty())
        .unwrap_or(false);
    let fb_ok = std::env::var("FB_APP_ID")
        .map(|k| !k.trim().is_empty())
        .unwrap_or(false);
    let manual_ok = std::env::var("OD_MANUAL_USERS")
        .map(|k| !k.trim().is_empty())
        .unwrap_or(false);
    let primary_btn = if google_ok {
        r#"<a href="/auth/google" class="lg-btn lg-btn-google" title="Sign in with Google">Sign in with Google</a>"#
    } else if fb_ok {
        r#"<a href="/auth/facebook" class="lg-btn lg-btn-facebook">Sign in with Facebook</a>"#
    } else {
        ""
    };
    let manual_form = if manual_ok {
        r#"<form method="post" action="/auth/login" class="lg-form"><label for="lg-email">Email</label><input type="email" id="lg-email" name="email" required autocomplete="email" placeholder="you@example.com"><label for="lg-pw">Password</label><input type="password" id="lg-pw" name="password" required autocomplete="current-password" placeholder="••••••••" minlength="8"><button type="submit" class="lg-btn lg-btn-submit">Continue</button></form>"#
    } else {
        ""
    };
    let sep = if !primary_btn.is_empty() && manual_ok {
        r#"<div class="lg-divider">or</div>"#
    } else {
        ""
    };
    let content = format!(
        r#"<section class="lg"><div class="lg-box"><h1 class="lg-h1">Sign in</h1><p class="lg-p">One click to access your waiver and account.</p><div class="lg-btns">{}{}{}</div></div></section>"#,
        primary_btn,
        sep,
        manual_form
    );
    let auth_link = head::f89(None);
    let page = format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><title>Sign in | OakilyDokily</title><link rel="icon" type="image/svg+xml" href="/assets/favicon.svg"><link rel="stylesheet" href="/assets/css/main.css"></head><body data-page="login">{}{}{}</body></html>"#,
        head::f90(&auth_link),
        content,
        r#"</main><footer class="footer footer-login"><a href="/">← Home</a></footer><script>(function(){var t=document.querySelector('.nav-toggle');var n=document.getElementById('nav-links');if(t&&n){t.onclick=function(){var o=n.classList.toggle('nav-open');t.setAttribute('aria-expanded',o);}}}());</script>"#
    );
    (jar, Html(page)).into_response()
}

/// f96=redirect_after_login. Clears redirect cookie, returns Redirect to target.
fn f96(mut jar: CookieJar) -> (Redirect, CookieJar) {
    let target = jar
        .get(REDIRECT_COOKIE)
        .and_then(|c| f95(c.value()))
        .unwrap_or_else(|| "/".into());
    jar = jar.remove(Cookie::from(REDIRECT_COOKIE));
    (Redirect::to(&target), jar)
}

/// t84=ManualLoginForm. s84=email s85=password.
#[derive(Deserialize)]
pub struct t84 {
    pub email: String,
    pub password: String,
}

/// t85=SignupForm. email, password, name.
#[derive(Deserialize)]
pub struct t85 {
    pub email: String,
    pub password: String,
    pub name: String,
}

/// f118 = hash_email. SHA256(trimmed lowercase email) as hex. OD_MANUAL_USERS lookup.
pub fn hash_email(email: &str) -> String {
    let normalized = email.trim().to_lowercase();
    let mut hasher = Sha256::new();
    hasher.update(normalized.as_bytes());
    hasher
        .finalize()
        .iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn f101_set_session(mut jar: CookieJar, email: &str, name: &str) -> Result<CookieJar, CookieJar> {
    let secret = match std::env::var("SESSION_SECRET") {
        Ok(s) if s.len() >= 32 => s,
        _ => return Err(jar),
    };
    let session_val = f86(email, name, secret.as_bytes());
    let mut c = Cookie::build((SESSION_COOKIE, session_val))
        .path("/")
        .http_only(true)
        .same_site(SameSite::Lax)
        .max_age(time::Duration::days(7));
    if f94() {
        c = c.secure(true);
    }
    jar = jar.add(c.build());
    Ok(jar)
}

/// f101=login_post. Checks users table, then OD_MANUAL_USERS.
pub async fn f101(
    State(s): State<Arc<AppState>>,
    jar: CookieJar,
    Form(f): Form<t84>,
) -> impl IntoResponse {
    if f88(&jar).is_some() {
        let (redirect, jar) = f96(jar);
        return (jar, redirect).into_response();
    }
    let email = f.email.trim().to_lowercase();
    let password = f.password.as_str();
    if email.is_empty() || password.is_empty() {
        return (jar, Redirect::to("/auth/login")).into_response();
    }
    let user = if let Some(ref d1) = s.s1 {
        crate::d1_auth::f78(d1, &email).await.ok().flatten()
    } else {
        crate::waiver::user_get(&s.s0, &email).await.ok().flatten()
    };
    if let Some((name, pwhash)) = user {
        if bcrypt::verify(password, &pwhash).unwrap_or(false) {
            match f101_set_session(jar, &email, &name) {
                Ok(jar) => {
                    let (redirect, jar) = f96(jar);
                    return (jar, redirect).into_response();
                }
                Err(jar) => return (jar, Redirect::to("/auth/login")).into_response(),
            }
        }
        return (jar, Redirect::to("/auth/login")).into_response();
    }
    let users = std::env::var("OD_MANUAL_USERS").unwrap_or_default();
    let email_hash = hash_email(&email);
    let mut found = false;
    let mut name = String::new();
    for pair in users.split(',') {
        let pair = pair.trim();
        if let Some((key, pwhash)) = pair.split_once(':') {
            let key = key.trim();
            let pwhash = pwhash.trim();
            let matches = key == email_hash || (key.contains('@') && key.to_lowercase() == email);
            if matches {
                if bcrypt::verify(password, pwhash).unwrap_or(false) {
                    found = true;
                    name = email.split('@').next().unwrap_or("User").to_string();
                    break;
                }
                return (jar, Redirect::to("/auth/login")).into_response();
            }
        }
    }
    if found {
        let name = if name.is_empty() {
            email.split('@').next().unwrap_or("User").to_string()
        } else {
            name
        };
        match f101_set_session(jar, &email, &name) {
            Ok(jar) => {
                let (redirect, jar) = f96(jar);
                return (jar, redirect).into_response();
            }
            Err(jar) => return (jar, Redirect::to("/auth/login")).into_response(),
        }
    }
    /* Not found: create account and sign in (one-click flow) */
    if password.len() < 8 {
        return (jar, Redirect::to("/auth/login")).into_response();
    }
    let name = email.split('@').next().unwrap_or("User").to_string();
    let pwhash = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        _ => return (jar, Redirect::to("/auth/login")).into_response(),
    };
    let created = if let Some(ref d1) = s.s1 {
        crate::d1_auth::f79(d1, &email, &pwhash, &name).await.is_ok()
    } else {
        crate::waiver::user_create(&s.s0, &email, &pwhash, &name)
            .await
            .is_ok()
    };
    if !created {
        return (jar, Redirect::to("/auth/login")).into_response();
    }
    match f101_set_session(jar, &email, &name) {
        Ok(jar) => {
            let (redirect, jar) = f96(jar);
            (jar, redirect).into_response()
        }
        Err(jar) => (jar, Redirect::to("/auth/login")).into_response(),
    }
}

/// f102=signup_page. GET /auth/signup → redirect to combined auth page.
pub async fn f102(jar: CookieJar) -> impl IntoResponse {
    if f88(&jar).is_some() {
        let (redirect, jar) = f96(jar);
        return (jar, redirect).into_response();
    }
    (jar, Redirect::to("/auth/login")).into_response()
}

/// f103=signup_post. Creates user, logs in, redirects.
pub async fn f103(
    State(s): State<Arc<AppState>>,
    jar: CookieJar,
    Form(f): Form<t85>,
) -> impl IntoResponse {
    if f88(&jar).is_some() {
        let (redirect, jar) = f96(jar);
        return (jar, redirect).into_response();
    }
    let email = f.email.trim().to_lowercase();
    let name = f.name.trim();
    let password = f.password.as_str();
    if email.is_empty() || name.is_empty() || password.len() < 8 {
        return (jar, Redirect::to("/auth/signup")).into_response();
    }
    if email.len() > 254 || name.len() > 100 {
        return (jar, Redirect::to("/auth/signup")).into_response();
    }
    let pwhash = match bcrypt::hash(password, bcrypt::DEFAULT_COST) {
        Ok(h) => h,
        _ => return (jar, Redirect::to("/auth/signup")).into_response(),
    };
    let created = if let Some(ref d1) = s.s1 {
        crate::d1_auth::f79(d1, &email, &pwhash, name).await.is_ok()
    } else {
        crate::waiver::user_create(&s.s0, &email, &pwhash, name)
            .await
            .is_ok()
    };
    if !created {
        return (jar, Redirect::to("/auth/signup")).into_response();
    }
    match f101_set_session(jar, &email, name) {
        Ok(jar) => {
            let (redirect, jar) = f96(jar);
            (jar, redirect).into_response()
        }
        Err(jar) => (jar, Redirect::to("/auth/signup")).into_response(),
    }
}

/// f84 = logout (GET /auth/logout)
pub async fn f84(jar: CookieJar) -> impl IntoResponse {
    let jar = jar.remove(Cookie::from(SESSION_COOKIE));
    (jar, Redirect::to("/")).into_response()
}

/// f88 = get_session. Returns (email, name) if logged in.
pub fn f88(jar: &CookieJar) -> Option<(String, String)> {
    let val = jar.get(SESSION_COOKIE)?.value();
    let secret = std::env::var("SESSION_SECRET").ok()?;
    if secret.len() < 32 {
        return None;
    }
    f87(val, secret.as_bytes())
}
