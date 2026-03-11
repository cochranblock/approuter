// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Shared head helpers: GA4, etc.

/// f80 = ga4_script. Returns gtag script when GA4_MEASUREMENT_ID set.
pub fn f80() -> String {
    std::env::var("GA4_MEASUREMENT_ID")
        .ok()
        .filter(|id| !id.is_empty())
        .map(|id| {
            format!(
                r#"<script async src="https://www.googletagmanager.com/gtag/js?id={}"></script><script>window.dataLayer=window.dataLayer||[];function gtag(){{dataLayer.push(arguments);}}gtag('js',new Date());gtag('config','{}');</script>"#,
                f81(&id),
                f81(&id)
            )
        })
        .unwrap_or_default()
}

/// f89 = nav_auth_link. Returns HTML for Sign in / Logout.
/// Facebook link shown when FB_APP_ID set. Apple deprecated.
pub fn f89(session: Option<(&str, &str)>) -> String {
    match session {
        Some((_email, name)) => {
            let n = f81(name);
            format!(
                r#"<a href="/auth/logout">Logout</a><span class="nav-user">{}</span>"#,
                n
            )
        }
        None => {
            let signin = r#"<a href="/auth/login" title="Sign in or create account">Sign in</a>"#;
            let facebook = std::env::var("FB_APP_ID")
                .ok()
                .filter(|k| !k.trim().is_empty())
                .map(|_| r#"<a href="/auth/facebook" title="Sign in with Facebook">Facebook</a>"#)
                .unwrap_or_default();
            format!("{}{}", signin, facebook)
        }
    }
}

/// f90 = nav. Full nav HTML with auth link.
pub fn f90(auth_link: &str) -> String {
    format!(
        r##"<a href="#main" class="skip-link">Skip to main content</a><nav class="nav"><a href="/" class="nav-brand">OakilyDokily</a><button class="nav-toggle" aria-label="Toggle menu" aria-expanded="false" aria-controls="nav-links"><span class="nav-toggle-bar"></span><span class="nav-toggle-bar"></span><span class="nav-toggle-bar"></span></button><div id="nav-links" class="nav-links" role="navigation"><a href="/">Home</a><a href="/about">About</a><a href="/contact">Contact</a><a href="/waiver">Waiver</a>{}</div></nav><main id="main" class="content">"##,
        auth_link
    )
}

/// f96 = book_call_link. OD_BOOK_CALL_URL (Calendly etc) or mailto fallback.
pub fn f96() -> String {
    std::env::var("OD_BOOK_CALL_URL")
        .ok()
        .filter(|u| !u.trim().is_empty())
        .map(|u| format!(r#"<a href="{}" class="btn btn-secondary">Book a Call</a>"#, f81(u.trim())))
        .unwrap_or_else(|| r#"<a href="mailto:byrdkaylie34@gmail.com?subject=Discovery%20Call%20Request" class="btn btn-secondary">Book a Call</a>"#.into())
}

/// f81 = html_escape_attr (head/attr context)
fn f81(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}
