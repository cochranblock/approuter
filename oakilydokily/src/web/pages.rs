// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use axum::extract::State;
use axum::response::Html;
use axum_extra::extract::CookieJar;
use std::sync::Arc;

use super::{auth, head};
use crate::AppState;

/// f70 = head. p3 = optional extra head (e.g. GA4)
fn f70(p0: &str, p1: &str, p2: &str, p3: &str) -> String {
    format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="{}"><title>{}</title><link rel="icon" type="image/svg+xml" href="/assets/favicon.svg" sizes="32x32"><link rel="stylesheet" href="/assets/css/main.css">{}</head><body data-page="{}">"#,
        p1, p0, p3, p2
    )
}

const FOOTER: &str = r##"</main><footer class="footer"><nav class="footer-nav"><a href="/">Home</a><a href="/about">About</a><a href="/contact">Contact</a><a href="/waiver">Waiver</a></nav><p>&copy; 2026 OakilyDokily</p><p class="footer-cta"><a href="mailto:byrdkaylie34@gmail.com?subject=OakilyDokily%20Inquiry" class="btn btn-primary">Get in Touch</a></p></footer><script>(function(){var t=document.querySelector('.nav-toggle');var n=document.getElementById('nav-links');if(t&&n){t.onclick=function(){var o=n.classList.toggle('nav-open');t.setAttribute('aria-expanded',o);}}}());</script></body></html>"##;

/// f73 = hero cover — 8-bit island mural as cover image (LinkedIn/Facebook style)
fn f73() -> String {
    r#"<div class="hero-cover" aria-hidden="true">
          <img src="/assets/mural.png" alt="" class="hero-cover-img" width="1200" height="400">
        </div>"#.to_string()
}

/// f104 = home. GET /. Hero with 8-bit island cover, auth link.
pub async fn home(State(_s): State<Arc<AppState>>, jar: CookieJar) -> Html<String> {
    let auth_link = head::f89(
        auth::f88(&jar)
            .as_ref()
            .map(|(e, n)| (e.as_str(), n.as_str())),
    );
    let v0 = f73();
    let book_call = head::f96();
    let content = format!(
        r#"<section class="hero">{}<div class="hero-content"><p class="hero-status">Serving Maryland · Flexible: contract, temp, part-time</p><h1>OakilyDokily</h1><p class="tagline">Veterinary professional services — kennel operations, overnight care, surgical support, and technician coverage for clinics and boarding facilities.</p><p class="hero-stats">Kennel operations · Overnight care · Surgical support · Client communication</p><p class="hero-note">Supporting clinics and boarding facilities with experienced, compassionate care</p><p class="hero-cta"><a href="mailto:byrdkaylie34@gmail.com?subject=OakilyDokily%20Inquiry" class="btn btn-primary">Get in Touch</a>{}<a href="/about" class="btn btn-secondary">Services & Experience</a></p></div></section>"#,
        v0, book_call
    );
    Html(format!(
        "{}{}{}{}",
        f70("OakilyDokily | Veterinary Professional Services", "OakilyDokily — Veterinary professional services for clinics and boarding facilities. Kennel operations, overnight care, surgical support, technician coverage.", "home", &format!(r#"<link rel="preload" href="/assets/mural.png" as="image" />{}"#, head::f80())),
        head::f90(&auth_link),
        content,
        FOOTER
    ))
}

/// f105 = about. GET /about. Services, resume, auth link.
pub async fn about(State(_s): State<Arc<AppState>>, jar: CookieJar) -> Html<String> {
    let auth_link = head::f89(
        auth::f88(&jar)
            .as_ref()
            .map(|(e, n)| (e.as_str(), n.as_str())),
    );
    let resume = include_str!("../../content/resume.html");
    let content = format!(
        r#"<section class="about"><h1>About OakilyDokily</h1><p class="services-intro">Veterinary professional services for clinics, hospitals, and boarding facilities. Overnight patient care, surgical support, kennel operations, and client communication.</p><h2 class="profile-subhead">Our Team</h2><p>Kaylie Cochran — 7+ years in animal care, kennel operations, overnight monitoring, and technician roles.</p><div class="print-resume-bar"><button type="button" class="btn btn-secondary" onclick="window.print()">Print Resume</button></div><div class="about-content">{}</div></section>"#,
        resume
    );
    Html(format!(
        "{}{}{}{}",
        f70("About | OakilyDokily", "OakilyDokily — Veterinary professional services for clinics and boarding facilities. Kennel operations, overnight care, surgical support.", "about", &head::f80()),
        head::f90(&auth_link),
        content,
        FOOTER
    ))
}

/// f106 = contact. GET /contact. Email, book-call CTA.
pub async fn contact(State(_s): State<Arc<AppState>>, jar: CookieJar) -> Html<String> {
    let auth_link = head::f89(
        auth::f88(&jar)
            .as_ref()
            .map(|(e, n)| (e.as_str(), n.as_str())),
    );
    let book_call = head::f96();
    let content = format!(
        r#"<section class="contact"><h1>Contact</h1><p class="trust-badge">Veterinary professional services for clinics and boarding facilities</p><p>Interested in overnight care, kennel support, or technician coverage? Reach out by email or request a discovery call.</p><p class="contact-micro">No form, no friction — just email.</p><p class="contact-cta"><a href="mailto:byrdkaylie34@gmail.com?subject=OakilyDokily%20Inquiry" class="btn btn-primary">Email Us</a>{}</p><p class="contact-note">We typically respond within 24–48 hours.</p></section>"#,
        book_call
    );
    Html(format!(
        "{}{}{}{}",
        f70("Contact | OakilyDokily", "Contact OakilyDokily — Veterinary professional services for clinics and boarding facilities.", "contact", &head::f80()),
        head::f90(&auth_link),
        content,
        FOOTER
    ))
}

/// f107 = health. GET /health. Returns "OK".
pub async fn health() -> &'static str {
    "OK"
}

/// f95 = sitemap. GET /sitemap.xml. For Search Console.
pub async fn sitemap() -> impl axum::response::IntoResponse {
    let base = std::env::var("OD_BASE_URL")
        .unwrap_or_else(|_| "https://oakilydokily.com".into())
        .trim_end_matches('/')
        .to_string();
    let urls = ["", "/about", "/contact", "/waiver"];
    let entries: String = urls
        .iter()
        .map(|p| {
            format!(
                "  <url><loc>{}{}</loc><changefreq>weekly</changefreq></url>",
                base, p
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let xml = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
{}
</urlset>"#,
        entries
    );
    ([(axum::http::header::CONTENT_TYPE, "application/xml")], xml)
}
