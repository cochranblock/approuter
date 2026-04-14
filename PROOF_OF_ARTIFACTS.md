<!-- Unlicense — cochranblock.org -->

# Proof of Artifacts

*Concrete evidence that this project works, ships, and is real.*

> This is the routing hub that makes cochranblock.org possible. One binary, all products, one tunnel.

## Architecture

```mermaid
flowchart LR
    Internet[Internet] --> CF[Cloudflare Tunnel]
    CF --> AR[approuter :8080]
    AR -->|Host: cochranblock.org| CB[cochranblock :8081]
    AR -->|Host: oakilydokily.com| OD[oakilydokily :3000]
    AR -->|Host: roguerepo.io| RR[rogue-repo :3001]
    AR -->|Host: *.ronin-sites.pro| RS[ronin-sites :8000]
    AR --> API[/approuter/register]
    AR --> DNS[/approuter/dns/update-a]
    AR --> TUN[/approuter/tunnels/*]
    AR --> ANA[/approuter/analytics]
    AR --> OpenAPI[/approuter/openapi.json]
```

## Build Output

| Metric | Value |
|--------|-------|
| Lines of Rust | 4,472 across 17 modules + 1,414 LOC in 7 test files |
| Functions | 156 |
| Tests | 33 across 8 test files |
| Largest module | run.rs (484 LOC) — server orchestration, post-spawn health polling |
| Largest subsystem | cloudflare/ (1,005 LOC across 3 files) — DNS, tunnel sync, token auth |
| Routing modes | Host-based, path-based, suffix matching, wildcard (*.domain) |
| API endpoints | 23+ routes — register, apps, DNS, tunnel control, multi-tunnel, analytics, live status |
| Credential model | Centralized — apps never touch CF_TOKEN. Mutating endpoints gated by ROUTER_API_KEY |
| Tunnel providers | Cloudflare, ngrok, Tailscale Funnel, Bore, localtunnel (multi-tunnel with competition metrics) |
| Analytics | Server-side from CF geo headers — zero JS, zero cookies, city-level geo |
| Binary size | 4.8MB (release) — opt-level=z, LTO, strip, panic=abort |
| Security | Tor exit blocking (CF-IPCountry T1), API key auth on all write endpoints |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| Dynamic Registry | Apps self-register via HTTP — zero-downtime, file-persisted, thread-safe RwLock, hostname collision detection (409) |
| Live Status | GET /approuter/status — parallel health check of all products with latency, hostnames, backend URLs |
| API Key Auth | ROUTER_API_KEY env var gates mutating endpoints (register, unregister, DNS, tunnel). Unset = open |
| Cloudflare Integration | Zone management, CNAME creation, tunnel sync, ingress rules, rate limits, cache rules |
| Multi-Tunnel Provider | Abstraction over 5 tunnel providers with spawn/stop/health check per provider |
| Tunnel Metrics | Per-provider latency tracking (p50/p95/p99), uptime percentage, streak counting |
| Tunnel Competition Dashboard | HTML dashboard at /approuter/tunnels/compete comparing providers head-to-head |
| Server-Side Analytics | Cloudflare geo header extraction, JSONL persistence, per-site stats, bot detection |
| Analytics Dashboard | HTML dashboard at /approuter/analytics with live request feed |
| Tunnel Automation | Generates cloudflared.yml from registry, auto-syncs on startup |
| OpenAPI Spec | Embedded, self-documenting API at /approuter/openapi.json |
| Client Library | approuter-client crate with retry logic (10 attempts) for startup race conditions |
| start-all | Single command orchestrates all backend services + tunnel |
| Restart Commands | Per-service restart: approuter, cochranblock, oakilydokily, ronin-sites, rogue-repo |
| Purge Cache | God mode — purges all Cloudflare zones under the account |
| Tor Exit Blocking | CF-IPCountry T1 header check in proxy — 403 before traffic reaches backends. Zero deps, zero lists |
| Status Dashboard | HTML dashboard at /approuter/status/ — dark theme, auto-refresh 15s, per-product health cards with latency |
| Post-Spawn Health Polling | 30s polling loop after start-all, 500ms interval, prints [ready]/[timeout] per backend |
| Analytics Retention | Auto-prune old JSONL analytics files on startup |
| Startup Env Validation | Validates required env vars before launching backends in start-all |
| Integration Test Suite | 33 tests across 8 files — API auth, hostname collision, route matching, proxy forwarding, tunnel auth, CF DNS mocks |

## Named Techniques

| Technique | What | Origin |
|-----------|------|--------|
| Multi-Tunnel Abstraction | Trait-based abstraction over 5 tunnel providers — apps register once, provider is swappable | Military network ops — transport layer abstracted from application layer |
| Self-Registering Reverse Proxy | Apps POST to /register on startup, proxy learns routing from apps, not config files | Service mesh patterns (Consul, Envoy) reduced to one binary |
| P23 Triple Lens | Optimist/Pessimist/Paranoia audit generates prioritized backlog — applied to QA hardening | Human-invented audit methodology |
| CF-IPCountry Gate | Block traffic classes at the proxy using Cloudflare's geo classification headers — zero lists, zero deps | Proxy-layer enforcement — let CF classify, let approuter enforce |

## How to Verify

```bash
cargo build --release -p approuter
cargo run -p approuter --release -- start-all   # Launches everything
curl localhost:8080/approuter/status             # Live health of all products
curl localhost:8080/approuter/apps               # List registered apps
curl localhost:8080/approuter/openapi.json       # API spec
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*

**Live products:** [cochranblock.org](https://cochranblock.org) | [oakilydokily.com](https://oakilydokily.com) | [roguerepo.io](https://roguerepo.io) | [ronin-sites.pro](https://ronin-sites.pro)
