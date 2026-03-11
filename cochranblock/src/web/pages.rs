// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use axum::extract::State;
use axum::response::Html;
use chrono::{Datelike, Duration, NaiveDate, Utc, Weekday};
use chrono_tz::America::New_York;
use serde::Serialize;
use std::sync::Arc;

use crate::t0;

#[derive(Serialize)]
struct t19 {
    label: String,
    mailto: String,
}

#[derive(Serialize)]
struct t20 {
    date: String,
    day_name: String,
    date_label: String,
    times: Vec<t19>,
}

const BASE_URL: &str = "https://cochranblock.org";

/// f69 = robots_txt. Why: Allow crawlers; point to sitemap.
pub async fn f69(State(_p0): State<Arc<t0>>) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "text/plain; charset=utf-8")],
        "User-agent: *\nAllow: /\nSitemap: https://cochranblock.org/sitemap.xml\n",
    )
}

/// f70 = sitemap_xml. Why: Main pages for search engines.
pub async fn f70(State(_p0): State<Arc<t0>>) -> impl axum::response::IntoResponse {
    let urls = [
        ("/", "1.0", "weekly"),
        ("/services", "0.9", "weekly"),
        ("/products", "0.9", "weekly"),
        ("/about", "0.8", "monthly"),
        ("/contact", "0.8", "monthly"),
        ("/book", "0.8", "weekly"),
        ("/federal-partners", "0.7", "monthly"),
    ];
    let mut xml = String::from(r#"<?xml version="1.0" encoding="UTF-8"?><urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#);
    for (path, priority, changefreq) in urls {
        xml.push_str(&format!(
            r#"<url><loc>{}{}</loc><changefreq>{}</changefreq><priority>{}</priority></url>"#,
            BASE_URL, path, changefreq, priority
        ));
    }
    xml.push_str("</urlset>");
    (
        axum::http::StatusCode::OK,
        [(axum::http::header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    )
}

const JSON_LD_ORG: &str = r#"<script type="application/ld+json">{"@context":"https://schema.org","@type":"Organization","name":"CochranBlock","url":"https://cochranblock.org","description":"Rust-only SaaS challenging enterprise greed. AI-powered software reset: offline-first, creative mode, superior pricing."}</script>"#;

/// f62 = html_head. Why: Consistent head + body open; data-page for CSS/JS targeting; JSON-LD Organization.
fn f62(p0: &str, p1: &str) -> String {
    let v_path = if p0 == "home" {
        "/".to_string()
    } else {
        format!("/{}", p0)
    };
    format!(
        r#"<!DOCTYPE html><html lang="en"><head><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1"><meta name="description" content="CochranBlock — Rust-only SaaS challenging enterprise greed. AI-powered software reset: offline-first, creative mode, superior pricing."><title>{}</title><meta property="og:title" content="{}"><meta property="og:description" content="Rust-only SaaS challenging enterprise greed. AI-powered software reset: offline-first, creative mode, superior pricing."><meta property="og:type" content="website"><meta property="og:url" content="{}{}"><link rel="icon" type="image/svg+xml" href="/assets/favicon.svg?v=4" sizes="32x32"><link rel="stylesheet" href="/assets/css/main.css">{}</head><body data-page="{}">"#,
        p1,
        p1,
        BASE_URL,
        v_path,
        JSON_LD_ORG,
        p0
    )
}
pub const C7: &str = r##"<a href="#main" class="skip-link">Skip to main content</a><nav class="nav"><a href="/" class="nav-brand"><img src="/assets/favicon.svg?v=4" alt="" class="nav-favicon" width="32" height="32">CochranBlock</a><button class="nav-toggle" aria-label="Toggle menu" aria-expanded="false" aria-controls="nav-links"><span class="nav-toggle-bar"></span><span class="nav-toggle-bar"></span><span class="nav-toggle-bar"></span></button><div id="nav-links" class="nav-links" role="navigation"><a href="/">Home</a><a href="/services">Services</a><a href="/products">Products</a><a href="/federal-partners">FBI & DOD IG</a><a href="/about">About</a><a href="/contact">Contact</a><a href="/book">Book</a></div></nav><main id="main" class="content">"##;
pub const C8: &str = r#"</main><footer class="footer"><nav class="footer-nav"><a href="/">Home</a><a href="/services">Services</a><a href="/products">Products</a><a href="/federal-partners">FBI & DOD IG</a><a href="/about">About</a><a href="/contact">Contact</a><a href="/book">Book</a></nav><p class="footer-brand"><a href="https://cochranblock.org"><img src="/assets/cochranblock-logo.svg?v=4" alt="CochranBlock" class="footer-logo" width="180" height="32"></a></p><p>&copy; 2026 CochranBlock</p><p class="footer-cta"><a href="mailto:mclarkfyrue@gmail.com?subject=CochranBlock%20Inquiry" class="btn btn-secondary">Get in Touch</a></p><p class="footer-links"><a href="https://www.linkedin.com/in/michael-c-ab55451b3" target="_blank" rel="noopener noreferrer">LinkedIn</a></p></footer><script>(function(){var t=document.querySelector('.nav-toggle');var n=document.getElementById('nav-links');if(t&&n){t.onclick=function(){var o=n.classList.toggle('nav-open');t.setAttribute('aria-expanded',o);}}}());</script></body></html>"#;

/// f2 = serve_index. Why: Hero page; first impression for cochranblock.org.
pub async fn f2(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="hero"><p class="hero-status">Product in development · Consulting: limited capacity</p><div class="hero-logo"><a href="/products"><img src="/assets/cochranblock-hero-logo.svg?v=4" alt="" class="hero-logo-img" width="128" height="128"></a></div><h1>CochranBlock</h1><p class="tagline">Rust-only SaaS that goes after the big guys — and their greed.</p><p class="hero-stats">Offline-first · Creative mode · Superior pricing · AI-powered software reset</p><p class="hero-note">Leveraging AI to reset the software market globally. Old-school offline options. No lock-in. No bloat.</p><p class="hero-skills">Rust · Zero bloat · Offline-first · Creative mode · AI orchestration · Enterprise-grade security</p><p class="hero-cta"><a href="/services" class="btn">What We Build</a><a href="/book" class="btn btn-secondary">Book a Call</a><a href="/contact" class="btn btn-secondary">Get in Touch</a><a href="/about" class="btn btn-secondary">About CochranBlock</a></p></section>"#;
    Html(format!("{}{}{}{}", f62("home", "CochranBlock | Rust SaaS Challenging Enterprise Greed"), C7, v0, C8))
}

/// f11 = serve_services. Why: What We Build + consulting capacity.
pub async fn f11(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="services"><h1>What We Do</h1><p class="services-intro">CochranBlock is building Rust-only SaaS that challenges enterprise greed. We use AI to reset the software market globally — with offline options, creative mode, and pricing that doesn't punish you for succeeding.</p><h2 class="services-section-head">Product</h2><p class="services-intro">Our flagship product is <a href="/products">Rogue Repo</a> — Rust-only app store. We also build Ronin Sites and Kova. See <a href="/products">Products</a> for details.</p><div class="service-cards"><details class="service-card" open><summary>Rust-Only App Store (Rogue Repo)</summary><p>We're building software that goes after the big guys. No JavaScript tax. No cloud lock-in. Offline-first by default. Creative mode for power users who want to own their workflow.<span class="service-outcome">Outcome: Software that respects you, your data, and your wallet.</span></p></details><details class="service-card"><summary>Offline-First & Creative Mode</summary><p>Work without the internet. Tinker, extend, and customize. No vendor deciding what you can and can't do. Your data stays yours.<span class="service-outcome">Outcome: Freedom from SaaS fatigue and subscription creep.</span></p></details><details class="service-card"><summary>Superior Pricing</summary><p>Enterprise software shouldn't cost enterprise prices for everyone. We're resetting expectations — fair pricing, no surprise audits, no per-seat gouging.<span class="service-outcome">Outcome: Tools you can actually afford to scale with.</span></p></details></div><h2 class="services-section-head">Consulting</h2><p class="services-intro">Consulting open now — limited capacity. Systems engineering, security, and integration work for teams that want to ship faster.</p><div class="service-cards"><details class="service-card" open><summary>Systems Engineering</summary><p>Design, build, and harden mission-critical infrastructure. Rust, Python, C/C++. Enterprise and high-compliance sectors.<span class="service-outcome">Outcome: Ship hardened systems faster, with less risk.</span></p></details><details class="service-card"><summary>Vulnerability Research & Product Security</summary><p>Product security, scoping, red teaming. 11+ years in the field.<span class="service-outcome">Outcome: Security-validated teams and capabilities.</span></p></details><details class="service-card"><summary>API & Integration Development</summary><p>Secure integrations, data pipelines, interoperability. No vendor lock-in.<span class="service-outcome">Outcome: Data flow across secure boundaries.</span></p></details></div><p class="services-cta"><a href="/contact" class="btn">Get in Touch</a><a href="/book" class="btn btn-secondary">Book a Call</a></p></section>"#;
    Html(format!("{}{}{}{}", f62("services", "Services | CochranBlock"), C7, v0, C8))
}

/// f12 = serve_about. Why: Tabbed Mission + Credentials (resume).
pub async fn f12(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = include_str!("../../content/resume.html");

    let v5 = format!(
        r#"<div class="legacy-tab-content"><div class="print-resume-bar"><button type="button" class="btn" onclick="window.print()">Print Resume</button></div><section class="resume-section">{}</section></div>"#,
        v0
    );

    let v6 = format!(
        r#"<section class="about"><h1>About CochranBlock</h1>
<div class="tabs" role="tablist">
  <button class="tab-btn active" data-tab="profile" role="tab" aria-selected="true">Mission</button>
  <button class="tab-btn" data-tab="legacy" role="tab" aria-selected="false">Credentials</button>
</div>
<div id="profile" class="tab-pane active" role="tabpanel" aria-hidden="false">
  <h3 class="profile-subhead">The Goal</h3>
  <p>CochranBlock exists to use AI to reset the software market globally. The big players have gotten greedy — lock-in, bloat, per-seat gouging, and cloud dependency. We're building the alternative: Rust-only SaaS with offline options, creative mode, and pricing that doesn't punish success.</p>
  <h3 class="profile-subhead">What We Build</h3>
  <p>Rust-only software. Offline-first. Creative mode for power users. Superior pricing. No JavaScript tax. No vendor deciding what you can do with your own data. We go after the big guys because someone has to.</p>
  <h3 class="profile-subhead">Consulting</h3>
  <p>Consulting open now — limited capacity. We take on select engagements: systems engineering, vulnerability research, product security, and integration work. 11+ years enterprise experience. Truthfinder. No bullshit.</p>
  <h3 class="profile-subhead">Founded By</h3>
  <p>Michael Cochran — Senior Systems Engineer & Product Security. Enterprise-vetted.</p>
</div>
<div id="legacy" class="tab-pane legacy-pane" role="tabpanel" aria-hidden="true">
  {}
</div>
</section>
<script>
document.addEventListener('DOMContentLoaded',function(){{var btns=document.querySelectorAll('.tab-btn');var panes=document.querySelectorAll('.tab-pane');btns.forEach(function(btn){{btn.addEventListener('click',function(){{var tab=btn.getAttribute('data-tab');btns.forEach(function(b){{b.classList.remove('active');b.setAttribute('aria-selected','false')}});panes.forEach(function(p){{p.classList.remove('active');p.setAttribute('aria-hidden',p.id===tab?'false':'true');if(p.id===tab)p.classList.add('active')}});btn.classList.add('active');btn.setAttribute('aria-selected','true')}})}})}});
</script>"#,
        v5
    );
    Html(format!("{}{}{}{}", f62("about", "About | CochranBlock"), C7, v6, C8))
}

/// f13 = serve_contact. Why: Email CTA; no form friction.
pub async fn f13(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="contact"><h1>Contact</h1><p class="trust-badge">Product in development · Consulting: limited capacity</p><blockquote class="testimonial">"Delivered ahead of schedule. Deep technical depth and clear communication."<cite>— Former client, enterprise security</cite></blockquote><p>Interested in our product roadmap, consulting, or a discovery call? Reach out by email.</p><p class="contact-micro">Product interest? Email with subject "Product Launch" to get notified when we ship.</p><p class="contact-micro">No form, no friction — just email.</p><p class="contact-cta"><a href="mailto:mclarkfyrue@gmail.com?subject=CochranBlock%20Inquiry" class="btn">Email Us</a><a href="/book" class="btn btn-secondary">Book a Call</a><a href="/assets/resume.pdf" class="btn btn-secondary" download>Michael's Resume</a></p><p class="contact-secondary">Or connect on <a href="https://www.linkedin.com/in/michael-c-ab55451b3" target="_blank" rel="noopener noreferrer">LinkedIn</a></p><p class="contact-note">We typically respond within 24–48 hours.</p></section>"#;
    Html(format!("{}{}{}{}", f62("contact", "Contact | CochranBlock"), C7, v0, C8))
}

/// f64 = get_date_slots — weekdays, 8am–5pm EST
fn f64() -> Vec<t20> {
    let today = Utc::now().with_timezone(&New_York).date_naive();
    let times = [
        "8:00am", "8:30am", "9:00am", "9:30am", "10:00am", "10:30am", "11:00am", "11:30am",
        "12:00pm", "12:30pm", "1:00pm", "1:30pm", "2:00pm", "2:30pm", "3:00pm", "3:30pm",
        "4:00pm", "4:30pm", "5:00pm",
    ];
    let mut date_slots = Vec::new();
    for day_offset in 0..90i64 {
        let date = today + Duration::days(day_offset);
        let wd = date.weekday();
        if wd != Weekday::Sat && wd != Weekday::Sun {
            let day_name = match wd {
                Weekday::Mon => "Mon",
                Weekday::Tue => "Tue",
                Weekday::Wed => "Wed",
                Weekday::Thu => "Thu",
                Weekday::Fri => "Fri",
                _ => continue,
            };
            let date_str = date.format("%b %d").to_string();
            let date_iso = date.format("%Y-%m-%d").to_string();
            let time_slots: Vec<t19> = times
                .iter()
                .map(|time| {
                    let subject = format!(
                        "Discovery%20Call%20Request%20-%20{}%20{}%20EST",
                        date_str.replace(' ', "%20"),
                        time.replace(':', "%3A")
                    );
                    t19 {
                        label: format!("{} EST", time),
                        mailto: format!("mailto:mclarkfyrue@gmail.com?subject={}", subject),
                    }
                })
                .collect();
            date_slots.push(t20 {
                date: date_iso,
                day_name: day_name.to_string(),
                date_label: date_str,
                times: time_slots,
            });
        }
        if date_slots.len() >= 60 {
            break;
        }
    }
    date_slots
}

/// f63 = serve_book. Why: Calendar + time slots for discovery calls; Braintrust flow.
pub async fn f63(State(_p0): State<Arc<t0>>) -> Html<String> {
    let date_slots = f64();
    let slots_json = serde_json::to_string(&date_slots).unwrap_or_default();
    let slots_json_escaped = slots_json.replace('<', "\\u003c").replace('>', "\\u003e");
    let v0 = format!(
        r##"<section class="booking-page"><h1>Schedule a Discovery Call</h1><p class="booking-intro">30 minutes · Discuss your goals and how I can help · All times Eastern (EST)</p><p class="booking-context">Shared via Braintrust — pick a date, then a time. Your email will open with a pre-filled request.</p><p class="booking-legend">Available weekdays · 8am–5pm EST</p><p class="booking-hint" id="booking-hint">Select a date, then a time.</p><a href="#booking-calendar" class="skip-link skip-link-calendar">Skip to calendar</a><div class="booking-calendar-wrapper" id="booking-calendar" role="region" aria-label="Calendar - pick a date then a time" aria-describedby="booking-hint"><div class="booking-calendar-header"><div class="booking-month-row"><button type="button" class="booking-nav" id="booking-prev" aria-label="Previous month">&larr;</button><h3 class="booking-month" id="booking-month"></h3><button type="button" class="booking-nav" id="booking-next" aria-label="Next month">&rarr;</button><span class="booking-available-badge" id="booking-available-badge"></span></div></div><div class="booking-calendar-grid" id="booking-grid" role="grid" aria-label="Month view"></div></div><div class="booking-time-panel" id="booking-time-panel" aria-live="polite" hidden><h3 class="booking-time-heading" id="booking-time-heading"></h3><div class="booking-time-slots" id="booking-time-slots" role="group"></div></div><p class="booking-note">I'll confirm within 24 hours.</p><p class="booking-fallback">None of these work? <a href="mailto:mclarkfyrue@gmail.com?subject=Discovery%20Call%20Request">Email me</a> to propose a time.</p><script type="application/json" id="booking-slots-data">{}</script><script src="/assets/js/booking.js"></script></section>"##,
        slots_json_escaped
    );
    Html(format!(
        "{}{}{}{}",
        f62("book", "Schedule a Call | CochranBlock"),
        C7,
        v0,
        C8
    ))
}

/// f67 = serve_products. Why: Rogue Repo, Ronin Sites, Kova product cards.
pub async fn f67(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="products"><h1>Products</h1><p class="products-intro">CochranBlock builds these products. Each is separate from cochranblock.org — the company site. All in active development — coming soon.</p><div class="product-cards"><article class="product-card"><span class="product-badge">Coming Soon</span><a href="https://roguerepo.io" rel="noopener noreferrer"><img src="/assets/img/rogue-repo.png" alt="Rogue Repo" class="product-img" width="400" height="300"></a><h2><a href="https://roguerepo.io" rel="noopener noreferrer">Rogue Repo</a></h2><p>Rust-only app store challenging enterprise greed. Offline-first, creative mode, superior pricing. No JavaScript tax. No cloud lock-in.</p></article><article class="product-card"><span class="product-badge">Coming Soon</span><a href="https://ronin-sites.pro" rel="noopener noreferrer"><img src="/assets/img/ronin-sites.png" alt="Ronin Sites" class="product-img" width="400" height="300"></a><h2><a href="https://ronin-sites.pro" rel="noopener noreferrer">Ronin Sites</a></h2><p>Shop and artist platform with subdomain routing, MinIO blob storage, and mobile-first site tuner. For creators and small businesses.</p></article><article class="product-card"><span class="product-badge">Coming Soon</span><img src="/assets/img/kova.png" alt="Kova" class="product-img" width="400" height="300"><h2>Kova</h2><p>AI Bending with Iron Man Method — human directs, AI executes. Sustained identity, no endless clarification loops. Build fast, ship fast.</p></article></div><p class="products-cta"><a href="/contact" class="btn">Get Notified</a><a href="/services" class="btn btn-secondary">What We Do</a></p></section>"#;
    Html(format!("{}{}{}{}", f62("products", "Products | CochranBlock"), C7, v0, C8))
}

/// f68 = serve_federal_partners. Why: FBI & DOD IG landing; COTS positioning.
pub async fn f68(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = r#"<section class="federal-partners"><h1>FBI & DOD Inspector General Partners</h1><p class="federal-badge">Free · Commercial Off The Shelf · Coming Soon</p><p class="federal-intro">The most capable free COTS product ever built — <strong>specially designed for FBI and Department of Defense Inspector General partners</strong>. Nothing else comes close.</p><div class="federal-claims"><h2>Built for the Bureau. Built for the Pentagon.</h2><p>This isn't generic enterprise software. Every line of code was crafted with <strong>FBI investigators</strong> and <strong>DOD IG auditors</strong> in mind. The perfect tool for the most demanding missions. Free. No strings. No lock-in.</p><ul class="federal-list"><li><strong>FBI-grade</strong> — Designed from the ground up for federal law enforcement workflows</li><li><strong>DOD IG–approved architecture</strong> — Meets the strictest defense audit requirements</li><li><strong>100% free COTS</strong> — The best commercial off-the-shelf product you'll ever deploy. At zero cost.</li><li><strong>Surprise launch</strong> — Stay tuned. This will change how federal partners work.</li></ul></div><p class="federal-cta">Interested? <a href="/contact">Get in Touch</a> — we'll notify FBI and DOD IG partners first.</p><p class="services-cta"><a href="/contact" class="btn">Notify Me</a><a href="/products" class="btn btn-secondary">All Products</a></p></section>"#;
    Html(format!("{}{}{}{}", f62("federal-partners", "FBI & DOD IG Partners | CochranBlock"), C7, v0, C8))
}

/// f10 = health. Why: Load balancer / approuter liveness probe.
pub async fn f10(State(_p0): State<Arc<t0>>) -> &'static str {
    "OK"
}

/// f71 = handler_404. Why: Site-styled 404 instead of axum default.
pub async fn f71(State(_p0): State<Arc<t0>>) -> (axum::http::StatusCode, Html<String>) {
    let body = r#"<section class="contact"><h1>Page Not Found</h1><p>The page you're looking for doesn't exist or has moved.</p><p class="contact-cta"><a href="/" class="btn">Back to Home</a><a href="/contact" class="btn btn-secondary">Get in Touch</a></p></section>"#;
    let html = format!("{}{}{}{}", f62("404", "Page Not Found | CochranBlock"), C7, body, C8);
    (axum::http::StatusCode::NOT_FOUND, Html(html))
}

/// f60 = html_escape. Why: XSS prevention for user content.
fn f60(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

/// f61 = strip_frontmatter. Why: Markdown-style frontmatter removal for embedded content.
fn f61(s: &str) -> &str {
    if let Some(v0) = s.strip_prefix("---\n") {
        if let Some(v1) = v0.find("\n---\n") {
            return s.get(v1 + 5..).unwrap_or(s).trim_start();
        }
    }
    s
}

/// f57 = serve_source — developer source view
pub async fn f57(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v_test = include_str!("../bin/cochranblock-test.rs");
    let v_tunnel = include_str!("../tunnel.rs");
    let v_router = include_str!("router.rs");
    let v0 = format!(
        r#"<section class="developer-page"><h1>Source Code</h1><p class="page-intro">Key files. Tokenized identifiers (f0, t0, etc.) per compression map.</p>
<details class="source-file" open><summary><span>cochranblock-test.rs</span><button type="button" class="copy-btn" data-copy="test" aria-label="Copy">Copy</button></summary><pre class="source-code" id="test"><code>{}</code></pre></details>
<details class="source-file"><summary><span>tunnel.rs</span><button type="button" class="copy-btn" data-copy="tunnel" aria-label="Copy">Copy</button></summary><pre class="source-code" id="tunnel"><code>{}</code></pre></details>
<details class="source-file"><summary><span>router.rs</span><button type="button" class="copy-btn" data-copy="router" aria-label="Copy">Copy</button></summary><pre class="source-code" id="router"><code>{}</code></pre></details>
</section><script>document.querySelectorAll('.copy-btn').forEach(function(b){{b.onclick=function(e){{e.stopPropagation();var p=document.getElementById(b.getAttribute('data-copy'));if(p&&navigator.clipboard){{navigator.clipboard.writeText(p.textContent);b.textContent='Copied!';setTimeout(function(){{b.textContent='Copy';}},1500);}}}}}});</script>"#,
        f60(v_test),
        f60(v_tunnel),
        f60(v_router)
    );
    Html(format!(
        "{}{}{}{}",
        f62("source", "Source Code | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f58 = serve_executive_summary
pub async fn f58(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = include_str!("../../content/executive_summary.html");
    Html(format!(
        "{}{}{}{}",
        f62("executive-summary", "Executive Summary | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f65 = serve_ai_orchestration
pub async fn f65(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = include_str!("../../content/ai_orchestration.html");
    Html(format!(
        "{}{}{}{}",
        f62("ai-orchestration", "AI Orchestration | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f66 = serve_prompts
pub async fn f66(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v0 = include_str!("../../content/prompt_reference.html");
    Html(format!(
        "{}{}{}{}",
        f62("prompts", "Prompt Reference | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}

/// f59 = serve_rules — cursor rules view
pub async fn f59(State(_p0): State<Arc<t0>>) -> Html<String> {
    let v_rebuild = include_str!("../../.cursor/rules/rebuild-redeploy.mdc");
    let v_token = include_str!("../../.cursor/rules/tokenization.mdc");
    let v_anti = include_str!("../../.cursor/rules/anti-patterns.mdc");
    let v_ci = include_str!("../../.cursor/rules/ci-pipeline.mdc");
    let v_test = include_str!("../../.cursor/rules/test-binary.mdc");
    let v_prod = include_str!("../../.cursor/rules/production-binary.mdc");
    let v0 = format!(
        r#"<section class="developer-page"><h1>Cursor Rules</h1><p class="page-intro">Project rules that govern AI assistance on this codebase.</p>
<details class="rule-file" open><summary>rebuild-redeploy.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>tokenization.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>anti-patterns.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>ci-pipeline.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>test-binary.mdc</summary><pre class="rule-content">{}</pre></details>
<details class="rule-file"><summary>production-binary.mdc</summary><pre class="rule-content">{}</pre></details>
</section>"#,
        f60(f61(v_rebuild)),
        f60(f61(v_token)),
        f60(f61(v_anti)),
        f60(f61(v_ci)),
        f60(f61(v_test)),
        f60(f61(v_prod))
    );
    Html(format!(
        "{}{}{}{}",
        f62("cursor-rules", "Cursor Rules | Michael Cochran"),
        C7,
        v0,
        C8
    ))
}
