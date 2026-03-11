// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! Native Rust PWA: HTML generated in Rust, assets embedded, no client JS for app logic.
//! Service worker (JS by spec) embedded as string, served by Rust.

use axum::http::{header, StatusCode};
use axum::{
    extract::Path,
    response::{IntoResponse, Response},
};
use rust_embed::RustEmbed;

#[derive(RustEmbed)]
#[folder = "assets/"]
pub struct Assets;

/// f90 = pwa_html — app store PWA. Zero client JS for app logic.
pub fn f90() -> String {
    let css = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/pwa.css"));
    format!(
        r##"<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="UTF-8">
  <meta name="viewport" content="width=device-width,initial-scale=1">
  <meta name="theme-color" content="#050508">
  <meta name="apple-mobile-web-app-capable" content="yes">
  <meta name="apple-mobile-web-app-status-bar-style" content="black-translucent">
  <title>Rogue Repo — Rust App Store</title>
  <link rel="manifest" href="/manifest.json">
  <link rel="icon" type="image/svg+xml" href="/assets/icon-192.svg">
  <style>{}</style>
</head>
<body>
  <a href="#main" class="skip-link">Skip to main content</a>
  <header class="header">
    <nav class="nav">
      <a href="/" class="nav-brand">Rogue Repo</a>
      <a href="/#featured">Featured</a>
      <a href="/#security">Security</a>
      <a href="/#economy">Rogue Bucks</a>
      <a href="/login">Login</a>
      <a href="/logout">Logout</a>
      <a href="/health">Health</a>
    </nav>
  </header>
  <main id="main" class="content">
    <section class="hero">
      <div class="hero-bg" aria-hidden="true"></div>
      <div class="hero-content">
        <h1 class="logo">Rogue Repo</h1>
        <p class="tagline">Rust-only app store. Sovereign. Offline-first.</p>
        <p class="hero-sub">Bank-grade payments · Rogue Bucks · No JavaScript tax</p>
        <div class="trust-callout">
          <p>Your ATM speaks <abbr title="The international standard for financial transaction messages — same format banks and card networks use">ISO 8583</abbr>. So do we. Real payment rails, not a proprietary API.</p>
        </div>
        <a href="/#economy" class="btn btn-primary">Get Started</a>
      </div>
    </section>
    <section id="featured" class="featured" aria-label="Featured apps">
      <h2 class="section-title">Featured</h2>
      <div class="app-grid">
        <a href="/apps/rogue-runner" class="app-card" style="animation-delay: 0.05s">
          <div class="app-icon"><img src="/assets/apps/rogue-runner.png" alt="Rogue Runner" width="96" height="96"></div>
          <h3>Rogue Runner</h3>
          <p class="app-meta">Endless Runner · 1000 Levels</p>
        </a>
        <article class="app-card" style="animation-delay: 0.1s">
          <div class="app-icon"><img src="/assets/apps/vault-raid.png" alt="Vault Raid" width="96" height="96"></div>
          <h3>Vault Raid</h3>
          <p class="app-meta">Heist Puzzle · Coming Soon</p>
        </article>
        <article class="app-card" style="animation-delay: 0.15s">
          <div class="app-icon"><img src="/assets/apps/ledger-quest.png" alt="Ledger Quest" width="96" height="96"></div>
          <h3>Ledger Quest</h3>
          <p class="app-meta">RPG Economy · Coming Soon</p>
        </article>
        <article class="app-card" style="animation-delay: 0.2s">
          <div class="app-icon"><img src="/assets/apps/switch-storm.png" alt="Switch Storm" width="96" height="96"></div>
          <h3>Switch Storm</h3>
          <p class="app-meta">Arcade Reflex · Coming Soon</p>
        </article>
        <a href="/apps/null-terminal?demo=1" class="app-card" style="animation-delay: 0.25s">
          <div class="app-icon"><img src="/assets/apps/null-terminal.png" alt="Null Terminal" width="96" height="96"></div>
          <h3>Null Terminal</h3>
          <p class="app-meta">Hacker Sim · Play</p>
        </a>
        <article class="app-card" style="animation-delay: 0.3s">
          <div class="app-icon"><img src="/assets/apps/sovereign-strike.png" alt="Sovereign Strike" width="96" height="96"></div>
          <h3>Sovereign Strike</h3>
          <p class="app-meta">Tower Defense · Coming Soon</p>
        </article>
        <article class="app-card" style="animation-delay: 0.35s">
          <div class="app-icon"><img src="/assets/apps/crypto-kart.png" alt="Crypto Kart" width="96" height="96"></div>
          <h3>Crypto Kart</h3>
          <p class="app-meta">Arcade Racing · Coming Soon</p>
        </article>
        <article class="app-card" style="animation-delay: 0.4s">
          <div class="app-icon"><img src="/assets/apps/offline-odyssey.png" alt="Offline Odyssey" width="96" height="96"></div>
          <h3>Offline Odyssey</h3>
          <p class="app-meta">Exploration · Coming Soon</p>
        </article>
        <article class="app-card" style="animation-delay: 0.45s">
          <div class="app-icon"><img src="/assets/apps/rust-rumble.png" alt="Rust Rumble" width="96" height="96"></div>
          <h3>Rust Rumble</h3>
          <p class="app-meta">Arcade Brawler · Coming Soon</p>
        </article>
        <article class="app-card" style="animation-delay: 0.5s">
          <div class="app-icon"><img src="/assets/apps/buck-blitz.png" alt="Buck Blitz" width="96" height="96"></div>
          <h3>Buck Blitz</h3>
          <p class="app-meta">Fast Arcade · Coming Soon</p>
        </article>
      </div>
    </section>
    <section id="security" class="security-section" aria-label="Payment security">
      <h2 class="section-title">Fort Knox Protection</h2>
      <div class="security-grid">
        <div class="security-card">
          <span class="security-icon" aria-hidden="true">🛡</span>
          <h3>Encrypted Offsite</h3>
          <p>Payment data never touches our servers. Encrypted at rest in trusted, well-known secure clouds.</p>
        </div>
        <div class="security-card">
          <span class="security-icon" aria-hidden="true">🔍</span>
          <h3>APT Hunter</h3>
          <p>Proprietary threat-hunting software monitors for advanced persistent threats. AI-level detection, 24/7.</p>
        </div>
        <div class="security-card">
          <span class="security-icon" aria-hidden="true">🔐</span>
          <h3>Bank-Grade Rails</h3>
          <p>ISO 8583. Same standard banks use. Sovereign payment infrastructure, not a black box.</p>
        </div>
      </div>
    </section>
    <section id="economy" class="economy-section" aria-label="Rogue Bucks economy">
      <h2 class="section-title">Rogue Bucks</h2>
      <div class="economy-card economy-coming-soon">
        <table class="economy">
          <tr><th>100 Rogue Bucks</th><td>= $1.00 USD</td></tr>
          <tr><th>Entry buy-in</th><td>$4.20 (420 bucks)</td></tr>
          <tr><th>Game download</th><td>42 bucks</td></tr>
          <tr><th>Add device</th><td>420 bucks</td></tr>
        </table>
        <p class="economy-placeholder">Coming soon</p>
      </div>
      <p class="badge">Mission-Critical FinTech</p>
    </section>
    <section id="downloads" class="economy-section" aria-label="Downloads">
      <h2 class="section-title">Downloads</h2>
      <p class="economy-placeholder">Login required. Rogue Runner — 1000-level endless runner.</p>
      <div class="download-grid" style="display:flex;gap:1rem;flex-wrap:wrap;margin-top:1rem;">
        <a href="/downloads/rogue-runner?platform=windows-msi" class="btn btn-primary">Windows (MSI)</a>
        <a href="/downloads/rogue-runner?platform=windows" class="btn btn-primary">Windows (EXE)</a>
        <a href="/downloads/rogue-runner?platform=android" class="btn btn-primary">Android (APK)</a>
      </div>
    </section>
  </main>
  <footer class="footer">
    <p>&copy; 2026 Rogue Repo</p>
  </footer>
  <script>
    if ('serviceWorker' in navigator) {{
      navigator.serviceWorker.register('/sw.js').catch(function() {{}});
    }}
  </script>
</body>
</html>"##,
        css
    )
}

/// f92 = serve_manifest, GET /manifest.json
pub async fn f92() -> Response {
    let body = Assets::get("manifest.json")
        .map(|f| f.data.into_owned())
        .unwrap_or_default();
    (
        [
            (header::CONTENT_TYPE, "application/manifest+json"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        body,
    )
        .into_response()
}

/// f93 = serve_sw, GET /sw.js — service worker (JS by spec, embedded in Rust)
pub async fn f93() -> Response {
    let body = Assets::get("sw.js")
        .map(|f| f.data.into_owned())
        .unwrap_or_default();
    (
        [
            (header::CONTENT_TYPE, "application/javascript"),
            (header::CACHE_CONTROL, "no-cache"),
        ],
        body,
    )
        .into_response()
}

/// f95 = serve_rogue_runner_wasm, GET /apps/rogue-runner-wasm (Rust/WASM build)
pub async fn f95() -> Response {
    let body = Assets::get("apps/rogue-runner-wasm/index.html")
        .map(|f| f.data.into_owned())
        .unwrap_or_default();
    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        body,
    )
        .into_response()
}

/// f117 = serve_null_terminal, GET /apps/null-terminal
pub async fn f117() -> Response {
    let body = Assets::get("apps/null-terminal.html")
        .map(|f| f.data.into_owned())
        .unwrap_or_default();
    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        body,
    )
        .into_response()
}

/// f94 = serve_rogue_runner, GET /apps/rogue-runner
pub async fn f94() -> Response {
    let body = Assets::get("apps/rogue-runner.html")
        .map(|f| f.data.into_owned())
        .unwrap_or_default();
    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=3600"),
        ],
        body,
    )
        .into_response()
}

/// f102 = serve_login, GET /login
pub async fn f102() -> Response {
    let body = Assets::get("login.html")
        .map(|f| f.data.into_owned())
        .unwrap_or_default();
    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        body,
    )
        .into_response()
}

/// f103 = serve_register, GET /register
pub async fn f103() -> Response {
    let body = Assets::get("register.html")
        .map(|f| f.data.into_owned())
        .unwrap_or_default();
    (
        [
            (header::CONTENT_TYPE, "text/html; charset=utf-8"),
            (header::CACHE_CONTROL, "public, max-age=300"),
        ],
        body,
    )
        .into_response()
}

/// f91 = serve_asset, GET /assets/*
pub async fn f91(Path(path): Path<String>) -> Response {
    let path = path.trim_start_matches('/').replace("..", "");
    if path.is_empty() {
        return StatusCode::NOT_FOUND.into_response();
    }
    if let Some(file) = Assets::get(&path) {
        let mime = mime_guess::from_path(&path).first_or_octet_stream();
        (
            [
                (header::CONTENT_TYPE, mime.as_ref()),
                (header::CACHE_CONTROL, "public, max-age=86400"),
            ],
            file.data.into_owned(),
        )
            .into_response()
    } else {
        StatusCode::NOT_FOUND.into_response()
    }
}
