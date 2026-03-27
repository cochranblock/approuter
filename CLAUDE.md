<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# approuter

- Reverse proxy + app registration + multi-tunnel + server-side analytics for cochranblock products.
- Build: cargo build -p approuter
- Run all: cargo run -p approuter --release -- start-all
- Products live in separate repos; approuter points to them via env vars (ROUTER_COCHRANBLOCK_URL, etc.)
- 14 modules, 4,116 LOC. Key: main.rs (CLI + server), cloudflare.rs (CF API), tunnel_provider.rs (multi-tunnel), analytics.rs (geo analytics), proxy.rs (reverse proxy), registry.rs (app registry).