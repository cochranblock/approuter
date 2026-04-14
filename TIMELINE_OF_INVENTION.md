<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Human Revelations — Invented Techniques

*Novel ideas that came from human insight, not AI suggestion. These are original contributions to the field.*

### Multi-Tunnel Abstraction (March 2026)

**Invention:** A single API that abstracts Cloudflare Tunnels, ngrok, Tailscale, and Bore behind one interface — apps register once and the tunnel provider is swappable without changing any application code.

**The Problem:** Every tunnel provider has its own CLI, configuration, and API. Switching from ngrok to Cloudflare Tunnels means rewriting deployment scripts. Running multiple tunnels for different services means managing multiple configs. The application shouldn't know or care how it reaches the internet.

**The Insight:** A reverse proxy already sits between the internet and the application. If the proxy manages the tunnel, the application only needs to know its local port. The tunnel becomes an implementation detail of the proxy, not the application. One binary, one config, any tunnel provider.

**The Technique:**
1. `tunnel_provider.rs`: trait-based abstraction over tunnel providers
2. Cloudflare, ngrok, Tailscale, Bore all implement the same interface
3. Apps register with approuter via REST API — approuter handles DNS, tunnel ingress, health checks
4. Switching providers = changing one env var, zero app changes
5. Tunnel ingress sync on startup prevents stale port bugs

**Result:** 5 products behind one reverse proxy, one Cloudflare tunnel, one domain. Adding a new product = one API call. Switching tunnel providers = one config change. No per-app tunnel management.

**Named:** Multi-Tunnel Abstraction
**Commit:** See initial architecture commit
**Origin:** Military network operations — the transport layer is abstracted from the application layer. A radio operator doesn't configure TCP/IP; the radio handles it. Same principle applied to web tunnels.

### Automatic App Registration with Health Routing (March 2026)

**Invention:** Applications self-register with the reverse proxy on startup, providing their port and hostnames. The proxy health-checks all registered apps and routes traffic only to healthy backends, with hostname collision detection (409 Conflict if two apps claim the same hostname).

**The Problem:** Traditional reverse proxies (nginx, HAProxy) require manual config files. Adding a new backend means editing a config and reloading. If a backend dies, traffic still routes to it until someone notices. Two developers accidentally claiming the same hostname produces silent routing conflicts.

**The Insight:** The app knows its own port and hostnames. The proxy should learn from the app, not from a config file. Registration should be a POST request, not a file edit. Health checks should be automatic, not optional. And hostname collisions should be detected at registration time, not after production traffic breaks.

**The Technique:**
1. `registry.rs`: POST /register with app name, port, hostnames — proxy adds to routing table
2. Health checks: parallel GET /health to all backends every 30s, unhealthy backends removed from rotation
3. Hostname collision: 409 Conflict if another app already owns the hostname
4. GET /approuter/status: real-time health of all registered products with latency
5. API key auth on mutating endpoints

**Result:** Zero-config reverse proxy. Apps register themselves. Dead apps stop receiving traffic. Hostname conflicts are caught immediately. Full observability via /approuter/status.

**Named:** Self-Registering Reverse Proxy
**Commit:** See initial architecture commit and `0e2138b` (status endpoint)
**Origin:** Service mesh patterns (Consul, Envoy) reduced to their simplest form — no sidecar proxies, no service mesh control plane, just a single binary that apps talk to on startup.

### 2026-04-08 — Human Revelations Documentation Pass

**What:** Documented novel human-invented techniques across the full CochranBlock portfolio. Added Human Revelations section with Multi-Tunnel Abstraction and Automatic App Registration.
**Commit:** See git log
**AI Role:** AI formatted and wrote the sections. Human identified which techniques were genuinely novel, provided the origin stories, and directed the documentation pass.

---

## Entries

### 2026-04-09 — Tor Exit Node Blocking + Security Hardening Sprint

**What:** Two-phase security hardening. Phase 1 (Apr 7): Block Tor exit nodes at the proxy layer using Cloudflare's `CF-IPCountry: T1` header — returns 403 before traffic reaches any backend. Zero dependencies, zero lists to maintain — Cloudflare classifies the traffic, approuter enforces. Phase 2 (Apr 3-7): Auth-gated all tunnel write endpoints (`start`/`stop`/`ensure`) that were previously unauthenticated, completing Backlog #1 from the P23 triple-lens audit. Added 7 tunnel auth tests covering reject-without-key, accept-correct-key, and open-when-no-key scenarios.
**Why:** P23 paranoia lens flagged tunnel start/stop/ensure as live unauthenticated write endpoints. Tor blocking closes the anonymous abuse vector. Together: no anonymous actor can start/stop tunnels or reach backends through Tor.
**Commit:** `e4e0f7f` (Tor blocking), `ce783ea` (tunnel auth gate)
**AI Role:** AI implemented the CF-IPCountry check and auth-gate plumbing. Human directed the security model — block at proxy, not at app; use CF headers, not IP lists.

### 2026-04-03 — Backlog Blitz (12 Items, 33 Tests, Status Dashboard)

**What:** Single-day sprint closed 12 of 20 backlog items from the P23 audit. Shipped: (1) 12 new integration tests across 4 test files — API key auth, hostname collision, status endpoint, route matching, proxy forwarding, proxy errors, Cloudflare DNS mocks. (2) Startup env validation for `start-all`. (3) Shared lazy reqwest client for Google APIs handler. (4) Analytics retention — auto-prune old JSONL on startup. (5) TROUBLESHOOTING.md. (6) Post-spawn health polling loop with `[ready]`/`[timeout]` per backend. (7) HTML status dashboard at `/approuter/status/` with dark theme, auto-refresh, card grid. (8) Binary size verified at 4.8MB (target was <5MB). Total test count reached 33 across 8 test files (1,414 LOC of tests).
**Why:** P23 triple-lens audit generated a prioritized backlog. This sprint executed it.
**Commit:** `8d2de8a`, `dc3beb2`, `e4cb83c`, `387b9be`, `ee81881`, `f82e636`, `8357da8`, `e86e3e4`, `54f7f4d`
**Method:** P23 triple-lens (optimist/pessimist/paranoia) generated the backlog; serial execution closed items by priority.
**AI Role:** AI implemented all features and tests. Human directed priority order and scope per item.

### 2026-04-02 — Live Status Endpoint

**What:** Added GET /approuter/status. Health-checks all registered products in parallel (3s timeout). Returns product name, backend URL, hostnames, healthy/unhealthy, HTTP status code, and latency for every routed service. The cross-reference — no lying about what's running.
**Commit:** `0e2138b`
**AI Role:** AI implemented f140 handler and StatusState type. Human specified the transparency requirement.

### 2026-04-02 — QA Hardening (P23 Triple Lens)

**What:** Five-phase hardening pass driven by P23 triple-lens audit (guest analysis = outsider perspective). (1) Shared reqwest::Client across all Cloudflare API calls (was 10 separate instances). (2) Registry migration logging. (3) Hostname collision detection — register returns 409 Conflict if another app owns the hostname. (4) API key auth on mutating endpoints via ROUTER_API_KEY. (5) Split cloudflare.rs (978 LOC) into cloudflare/mod.rs + dns.rs + tunnel.rs.
**Commit:** `b207918`
**Method:** P23 — outsider code review (pessimist lens), routing table cross-reference (paranoia lens), then synthesis into prioritized action plan.
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
