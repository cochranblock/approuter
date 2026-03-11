// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use axum::extract::{ConnectInfo, Query, State};
use axum::http::{HeaderMap, StatusCode};
use axum::response::{Html, IntoResponse, Redirect};
use axum::Form;
use axum_extra::extract::CookieJar;
use std::net::SocketAddr;
use std::sync::Arc;

use super::{auth, email, head};
use crate::waiver;
use crate::AppState;

/// f72 = client_ip
fn f72(addr: SocketAddr, headers: &HeaderMap) -> String {
    headers
        .get("x-forwarded-for")
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.split(',').next())
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|| addr.ip().to_string())
}

/// f70 = head. p3 = optional extra head (e.g. Turnstile script)
fn f70(p0: &str, p1: &str, p2: &str, p3: &str) -> String {
    format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="{}"><title>{}</title><link rel="icon" type="image/svg+xml" href="/assets/favicon.svg" sizes="32x32"><link rel="stylesheet" href="/assets/css/main.css">{}</head><body data-page="{}">"#,
        p1, p0, p3, p2
    )
}

const FOOTER: &str = r##"</main><footer class="footer"><nav class="footer-nav"><a href="/">Home</a><a href="/about">About</a><a href="/contact">Contact</a><a href="/waiver">Waiver</a></nav><p>&copy; 2026 OakilyDokily</p><p class="footer-cta"><a href="mailto:byrdkaylie34@gmail.com?subject=OakilyDokily%20Inquiry" class="btn btn-primary">Get in Touch</a></p></footer><script>(function(){var t=document.querySelector('.nav-toggle');var n=document.getElementById('nav-links');if(t&&n){t.onclick=function(){var o=n.classList.toggle('nav-open');t.setAttribute('aria-expanded',o);}}}());</script></body></html>"##;

fn f74_test_bypass() -> bool {
    std::env::var("OD_TEST_WAIVER_BYPASS")
        .map(|v| v.trim() == "1")
        .unwrap_or(false)
}

/// f74 = get_waiver. Why: Single handler for waiver page; Turnstile optional when key set. Requires auth.
pub async fn f74(State(_s): State<Arc<AppState>>, jar: CookieJar) -> impl IntoResponse {
    let session = auth::f88(&jar).or_else(|| {
        if f74_test_bypass() {
            Some(("waiver-test@example.com".into(), "waiver-test".into()))
        } else {
            None
        }
    });
    if session.is_none() {
        return (jar, Redirect::to("/auth/login?redirect=/waiver")).into_response();
    }
    let auth_link = head::f89(session.as_ref().map(|(e, n)| (e.as_str(), n.as_str())));
    let terms = waiver::terms_text();
    let v0 = f71(terms);
    let turnstile_script = std::env::var("TURNSTILE_SITE_KEY")
        .ok()
        .filter(|k| !k.is_empty())
        .map(|key| format!(
            r#"<script src="https://challenges.cloudflare.com/turnstile/v0/api.js" async defer></script><script>window.TURNSTILE_SITE_KEY="{}";</script>"#,
            html_escape_attr(&key)
        ))
        .unwrap_or_default();
    let extra_head = format!("{}{}", turnstile_script, head::f80());
    let turnstile_widget = std::env::var("TURNSTILE_SITE_KEY")
        .ok()
        .filter(|k| !k.is_empty())
        .map(|key| format!(
            r#"<div class="cf-turnstile" data-sitekey="{}" data-theme="dark" data-callback="window.turnstileCb" data-error-callback="window.turnstileErr"></div><p class="ds-turnstile-hint" id="turnstile-hint" style="display:none;font-size:0.9em;color:#888;margin-top:0.5em;">Verification didn't load. Try disabling ad blockers or <a href="/waiver">refresh</a>.</p>"#,
            html_escape_attr(&key)
        ))
        .unwrap_or_else(|| String::from(""));
    let content = format!(
        r#"<section class="ds-waiver"><div class="ds-steps"><span class="ds-step ds-active"><span class="ds-num">1</span> Review</span><span class="ds-step"><span class="ds-num">2</span> Sign</span><span class="ds-step"><span class="ds-num">3</span> Complete</span></div>
<div class="ds-doc"><h1 class="ds-title">Service Waiver &amp; Liability Release</h1><p class="ds-intro">Please review the agreement below. Your electronic signature creates a legally binding contract.</p>
<div class="ds-body" id="waiver-terms"><pre>{}</pre></div>
<p class="ds-hint">Scroll through the entire document before signing.</p>
<div class="ds-sign"><div class="ds-line"></div><p class="ds-sign-label">Sign here</p>
<form class="ds-form" method="post" action="/waiver" id="waiver-form"><label for="full_name">Full legal name</label><input type="text" id="full_name" name="full_name" required autocomplete="name" placeholder="Type your full legal name" maxlength="200"><label for="email">Email</label><input type="email" id="email" name="email" required autocomplete="email" placeholder="your@email.com" maxlength="254">
<div class="ds-consent"><label class="ds-check"><input type="checkbox" name="consent_electronic" value="1" required id="consent_electronic"> I agree to use electronic records and signatures.</label><label class="ds-check"><input type="checkbox" name="agree_terms" value="1" required id="agree_terms"> I have read and agree to the terms above.</label></div>
{}<button type="submit" class="ds-btn" id="submit-btn" disabled>Sign</button></form></div>
<p class="ds-note">By signing you acknowledge receipt. Records retained 7 years.</p></div></section>
<script>(function(){{var f=document.getElementById('waiver-form');var b=document.getElementById('submit-btn');var c1=document.getElementById('consent_electronic');var c2=document.getElementById('agree_terms');var n=document.getElementById('full_name');var e=document.getElementById('email');var hasTurnstile=!!document.querySelector('.cf-turnstile');function chk(){{var base=c1&&c2&&n&&e&&c1.checked&&c2.checked&&n.value.trim()&&e.value.trim();b.disabled=hasTurnstile?!(base&&window.turnstileDone):!base;}}function bind(){{if(!f)return;c1.onchange=c2.onchange=chk;n.oninput=n.onchange=e.oninput=e.onchange=chk;window.turnstileCb=function(){{window.turnstileDone=true;chk();}};window.turnstileErr=function(){{var h=document.getElementById('turnstile-hint');if(h)h.style.display='block';}};window.turnstileDone=!hasTurnstile;chk();if(hasTurnstile){{setTimeout(function(){{if(!window.turnstileDone){{var h=document.getElementById('turnstile-hint');if(h)h.style.display='block';}}}},15000);}}}}(document.readyState==='loading'?document.addEventListener('DOMContentLoaded',bind):bind());}})();</script>"#,
        v0, turnstile_widget
    );
    (jar, Html(format!("{}{}{}{}", f70("Waiver | OakilyDokily", "Sign the OakilyDokily service waiver and liability release before engaging our veterinary professional services.", "waiver", &extra_head), head::f90(&auth_link), content, FOOTER))).into_response()
}

/// f71 = html_escape
fn f71(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn html_escape_attr(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

/// t73 = WaiverForm. s73=full_name s74=email s75=consent_electronic s76=agree_terms s77=cf_turnstile_response
#[derive(serde::Deserialize)]
pub struct t73 {
    #[serde(rename = "full_name")]
    pub s73: String,
    #[serde(rename = "email")]
    pub s74: String,
    #[serde(rename = "consent_electronic")]
    pub s75: Option<String>,
    #[serde(rename = "agree_terms")]
    pub s76: Option<String>,
    #[serde(rename = "cf-turnstile-response")]
    pub s77: Option<String>,
}

/// f76 = verify_turnstile
async fn f76(token: &str, remoteip: &str, secret: &str) -> bool {
    let client = reqwest::Client::new();
    let params = [
        ("secret", secret),
        ("response", token),
        ("remoteip", remoteip),
    ];
    match client
        .post("https://challenges.cloudflare.com/turnstile/v0/siteverify")
        .form(&params)
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await
    {
        Ok(r) => match r.json::<serde_json::Value>().await {
            Ok(j) => j.get("success").and_then(|v| v.as_bool()).unwrap_or(false),
            Err(_) => false,
        },
        Err(_) => false,
    }
}

/// f75 = post_waiver. Why: Validates form, stores waiver, sends email, redirects to confirmed. Requires auth.
pub async fn f75(
    State(s): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    jar: CookieJar,
    Form(f): Form<t73>,
) -> impl IntoResponse {
    let authed = auth::f88(&jar).is_some() || f74_test_bypass();
    if !authed {
        return (jar, Redirect::to("/auth/login?redirect=/waiver")).into_response();
    }
    let full_name = f.s73.trim();
    let email = f.s74.trim();
    match waiver::f77(full_name, email) {
        Ok(()) => {}
        Err("name empty" | "email empty") => {
            return (StatusCode::BAD_REQUEST, "Missing required fields").into_response();
        }
        Err(_) => {
            return (StatusCode::BAD_REQUEST, "Invalid input length").into_response();
        }
    }
    if f.s75.as_deref() != Some("1") || f.s76.as_deref() != Some("1") {
        return (
            StatusCode::BAD_REQUEST,
            "You must consent to electronic signing and agree to the terms",
        )
            .into_response();
    }
    if let Ok(secret) = std::env::var("TURNSTILE_SECRET_KEY") {
        if !secret.is_empty() {
            let token = f.s77.as_deref().unwrap_or("");
            let remoteip = f72(addr, &headers);
            if token.is_empty() || !f76(token, &remoteip, &secret).await {
                return (
                    StatusCode::BAD_REQUEST,
                    "Security check failed. Please try again.",
                )
                    .into_response();
            }
        }
    }
    let v0 = f72(addr, &headers);
    let ua_str = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();
    match waiver::insert(
        &s.s0,
        full_name,
        email,
        Some(&v0),
        Some(&ua_str),
        full_name,
    )
    .await
    {
        Ok(ref_id) => {
            email::f78(email, full_name, &ref_id);
            let loc = format!("/waiver/confirmed?ref={}", urlencoding::encode(&ref_id));
            Redirect::to(loc.as_str()).into_response()
        }
        Err(e) => {
            tracing::error!("waiver insert failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Unable to save waiver. Please try again.",
            )
                .into_response()
        }
    }
}

/// f110 = waiver_confirmed. GET /waiver/confirmed.
pub async fn confirmed(
    Query(q): Query<std::collections::HashMap<String, String>>,
    jar: CookieJar,
) -> Html<String> {
    let auth_link = head::f89(
        auth::f88(&jar)
            .as_ref()
            .map(|(e, n)| (e.as_str(), n.as_str())),
    );
    let ref_line = q
        .get("ref")
        .filter(|s| !s.is_empty())
        .map(|r| {
            format!(
                r#"<p class="ds-detail">Reference ID: <code>{}</code></p>"#,
                html_escape_attr(r)
            )
        })
        .unwrap_or_default();
    let v0 = format!(
        r#"<section class="ds-waiver ds-done"><div class="ds-doc ds-complete"><div class="ds-check-icon">✓</div><h1>Waiver Signed</h1><p class="ds-success">Your signature has been recorded.</p>{}<p class="ds-detail">A copy is retained for 7 years. Contact us if you need a copy.</p><a href="/" class="ds-btn">Done</a></div></section>"#,
        ref_line
    );
    Html(format!(
        "{}{}{}{}",
        f70(
            "Waiver Confirmed | OakilyDokily",
            "Your OakilyDokily waiver has been signed and recorded.",
            "waiver-confirmed",
            &head::f80()
        ),
        head::f90(&auth_link),
        v0,
        FOOTER
    ))
}
