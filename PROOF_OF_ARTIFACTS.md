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
    AR --> OpenAPI[/approuter/openapi.json]
```

## Build Output

| Metric | Value |
|--------|-------|
| Lines of Rust | 2,811 across 10 modules |
| Largest module | cloudflare.rs (978 LOC) — full CF API integration |
| Routing modes | Host-based, path-based, suffix matching |
| API endpoints | Register, unregister, list apps, DNS update, tunnel control |
| Credential model | Centralized — apps never touch CF_TOKEN |
| Tunnel management | Spawns/manages cloudflared as child process with SHA256 verification |
| Binary optimization | opt-level=z, LTO, strip, panic=abort |

## Key Artifacts

| Artifact | Description |
|----------|-------------|
| Dynamic Registry | Apps self-register via HTTP — zero-downtime, file-persisted, thread-safe RwLock |
| Cloudflare Integration | Zone management, CNAME creation, tunnel sync, ingress rules, rate limits, cache rules |
| Tunnel Automation | Generates cloudflared.yml from registry, auto-syncs on startup |
| OpenAPI Spec | Embedded, self-documenting API at /approuter/openapi.json |
| Client Library | approuter-client crate with retry logic (10 attempts) for startup race conditions |
| start-all | Single command orchestrates all backend services + tunnel |

## How to Verify

```bash
cargo build --release -p approuter
cargo run -p approuter --release -- start-all   # Launches everything
curl localhost:8080/approuter/apps               # List registered apps
curl localhost:8080/approuter/openapi.json       # API spec
```

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
