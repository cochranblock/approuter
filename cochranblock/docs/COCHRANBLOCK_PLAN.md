# CochranBlock — Implementation Plan

**Date:** 2026-03-01  
**Scope:** Gaps identified in codebase review. Ordered by priority.

---

## Phase 1: Production Hardening

### 1.1 Graceful shutdown (§17) ✓
- **What:** Handle `ctrl_c` so process exits cleanly.
- **Where:** `src/bin/cochranblock.rs`
- **How:** `tokio::select!` on `axum::serve` vs `tokio::signal::ctrl_c()`. On signal, drop listener, await in-flight requests.
- **Effort:** S

### 1.2 Security headers ✓
- **What:** Add `X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`.
- **Where:** Router layer or middleware.
- **How:** `tower_http::set_header` or custom layer.
- **Effort:** S

### 1.3 Custom 404 ✓
- **What:** Site-styled 404 page instead of axum default.
- **Where:** Router fallback handler.
- **How:** `.fallback(f71)` returning Html with nav/footer.
- **Effort:** S

---

## Phase 2: SEO & Discoverability

### 2.1 robots.txt ✓
- **What:** `GET /robots.txt` → allow all, optional Sitemap URL.
- **Where:** Router, new handler in `pages.rs` or `assets.rs`.
- **How:** Static response or embedded string.
- **Effort:** S

### 2.2 sitemap.xml ✓
- **What:** `GET /sitemap.xml` with main pages (/, /services, /products, /about, /contact, /book, /federal-partners).
- **Where:** New handler.
- **How:** Generate XML string; base URL from env or constant.
- **Effort:** S

### 2.3 JSON-LD Organization ✓
- **What:** Structured data in `<head>` for rich results.
- **Where:** `f62` (html_head) or per-page.
- **How:** `<script type="application/ld+json">` with Organization schema.
- **Effort:** S

---

## Phase 3: Config & Env ✓

### 3.1 .env load path ✓
- **What:** Load `.env` from deterministic path (e.g. `COCHRANBLOCK_ROOT` or binary dir).
- **Done:** `dotenv()` then `from_path_override(COCHRANBLOCK_ROOT/.env)` and `from_path_override("cochranblock/.env")`.

### 3.2 .env.example cleanup ✓
- **What:** Remove or document unused `PORTFOLIO_*` vars for static-only mode.
- **Done:** Split into "Required for static", "Required for admin/dns", "Optional for admin/dns".

---

## Phase 4: Dead Code Decision ✓

### 4.1 Wire or remove admin/auth/dns ✓
- **What:** db, auth, sessions, crypto, dns exist but t0 is empty; nothing uses them.
- **Done:** **C. Feature-flag.** `admin` feature gates config, crypto, auth, dns, db. Default = static only. Tests use `admin`.

### 4.2 Developer routes (f57, f58, f59, f65, f66) ✓
- **What:** serve_source, executive_summary, rules, ai_orchestration, prompts exist in pages.rs but not routed.
- **Done:** **C. Feature-flag.** `dev` feature exposes `/dev/source`, `/dev/exec-summary`, `/dev/rules`, `/dev/ai-orchestration`, `/dev/prompts`. Test binary enables `dev`; `dev_routes_200` test added.

---

## Phase 5: Optional Enhancements

### 5.1 RSS/Atom feed
- **What:** `GET /feed.xml` for blog or updates (if content exists).
- **Effort:** M (only if there’s feed-worthy content)

### 5.2 Rate limiting
- **What:** Per-IP rate limit on public routes.
- **Where:** Router layer.
- **Effort:** M

---

## Summary

| Phase | Items | Total Effort |
|-------|-------|--------------|
| 1 | 3 | S×3 |
| 2 | 3 | S×3 |
| 3 | 2 | S + XS |
| 4 | 2 (choose A/B/C each) | L or M |
| 5 | 2 | Optional |

**Recommended order:** 1.1 → 1.2 → 1.3 → 2.1 → 2.2 → 3.1 → 4.1 (choose) → 4.2 (choose).
