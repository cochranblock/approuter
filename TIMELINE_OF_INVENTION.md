<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-04-02 — Live Status Endpoint

**What:** Added GET /approuter/status. Health-checks all registered products in parallel (3s timeout). Returns product name, backend URL, hostnames, healthy/unhealthy, HTTP status code, and latency for every routed service. The cross-reference — no lying about what's running.
**Commit:** `0e2138b`
**AI Role:** AI implemented f140 handler and StatusState type. Human specified the transparency requirement.

### 2026-04-02 — QA Hardening

**What:** Five-phase hardening pass. (1) Shared reqwest::Client across all Cloudflare API calls (was 10 separate instances). (2) Registry migration logging. (3) Hostname collision detection — register returns 409 Conflict if another app owns the hostname. (4) API key auth on mutating endpoints via ROUTER_API_KEY. (5) Split cloudflare.rs (978 LOC) into cloudflare/mod.rs + dns.rs + tunnel.rs.
**Commit:** `b207918`
**AI Role:** AI executed full QA audit and implemented all five phases. Human directed the audit scope.

### 2026-03-30 — Stale Tunnel Port Fix

**What:** Fixed bug where f96a (tunnel ingress sync) ran on startup even with --no-tunnel, pushing dev/test ephemeral ports (50433, 57701) to the Cloudflare dashboard. Production cloudflared then fetched the wrong port. Fix: gate f96a behind --no-tunnel flag, and sync explicitly before spawning cloudflared in start-all.
**Commit:** `37631f1`
**AI Role:** AI diagnosed root cause and fixed both code paths. Human reported the production failure.

### 2026-03-27 — Server-Side Analytics

**What:** Added server-side visitor analytics from Cloudflare geo headers. Zero JS, zero cookies, city-level geo for free. JSONL persistence, per-site stats, bot detection, analytics dashboard.
**Commit:** `708477c`
**AI Role:** AI implemented the analytics module and dashboard. Human designed the privacy model (hashed IPs, no cookies, CF headers only).

### 2026-03-27 — Purge Cache (God Mode)

**What:** Implemented purge-cache subcommand that purges all Cloudflare zones under the account in one shot.
**Commit:** `b895d9e`
**AI Role:** AI wrote the CF API integration. Human decided scope (all zones, not per-zone).

### 2026-03-22 — CODEOWNERS + Governance

**What:** Added CODEOWNERS and OWNERS.yaml for repository governance.
**Commit:** `ca99daf`
**AI Role:** AI generated governance files. Human decided ownership structure.

### 2026-03-20 — Tunnel Startup Sync

**What:** Sync tunnel ingress on startup to prevent stale port (55842) from previous runs.
**Why:** Production reliability — stale tunnel configs caused routing failures after restarts.
**Commit:** `6b07717`
**AI Role:** AI fixed the race condition. Human identified the production failure pattern.

### 2026-03-18 — Foundational Founders v0.2.0

**What:** Version bump, contributor attribution locked, Unlicense headers across all files.
**Commit:** `6366db2`
**AI Role:** AI applied headers systematically. Human decided licensing and attribution model.

### 2026-03-17 — GitHub Actions CI

**What:** CI workflow running exopack test binary (approuter-test) with xvfb + chromium.
**Commit:** `31f607e`
**AI Role:** AI wrote CI config. Human specified test requirements and environment.

### 2026-03-16 — Railway Removal + Cloudflare-Only

**What:** Removed all Railway deployment artifacts. approuter is Cloudflare tunnel + VPS only.
**Why:** Railway added unnecessary complexity. Single tunnel + single VPS is the architecture.
**Commit:** `3723a11`
**AI Role:** AI cleaned up artifacts. Human made the architectural decision to simplify.

### 2026-03-14 — Initial Architecture

**What:** Full reverse proxy with dynamic registration, Cloudflare tunnel management, DNS automation, OpenAPI spec, client library.
**Why:** Multi-product routing without operational overhead. One command, all services.
**AI Role:** AI generated initial implementation. Human designed routing strategy, credential model, and tunnel automation.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
