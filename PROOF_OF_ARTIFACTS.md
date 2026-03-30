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
| Lines of Rust | 4,146 across 14 modules |
| Largest module | cloudflare.rs (978 LOC) — full CF API integration |
| Routing modes | Host-based, path-based, suffix matching |
| API endpoints | 22 routes — register, apps, DNS, tunnel control, multi-tunnel, analytics, dashboard |
| Credential model | Centralized — apps never touch CF_TOKEN |
| Tunnel providers | Cloudflare, ngrok, Tailscale Funnel, Bore, localtunnel (multi-tunnel with competition metrics) |
| Analytics | Server-side from CF geo headers — zero JS, zero cookies, city-level geo |
| Binary optimization | opt-level=z, LTO, strip, panic=abort |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| Dynamic Registry | Apps self-register via HTTP — zero-downtime, file-persisted, thread-safe RwLock |
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

## How to Verify

```bash
cargo build --release -p approuter
cargo run -p approuter --release -- start-all   # Launches everything
curl localhost:8080/approuter/apps               # List registered apps
curl localhost:8080/approuter/openapi.json       # API spec
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
