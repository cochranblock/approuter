<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized stack. Top = most important. Max 20 items.
Tags: `[build]` `[test]` `[docs]` `[feature]` `[fix]` `[research]`

---

1. ~~`[test]` Add integration test for API key auth~~ DONE (api_hardening.rs)
2. ~~`[test]` Add integration test for hostname collision~~ DONE (api_hardening.rs)
3. ~~`[test]` Add integration test for /approuter/status~~ DONE (api_hardening.rs)
4. `[feature]` Post-spawn health check loop in start-all — poll /health on each backend before declaring ready. Dep: cochranblock, oakilydokily, rogue-repo, ronin-sites must expose /health
5. ~~`[fix]` Replace reqwest::Client::new() in api.rs f110 with shared lazy client~~ DONE
6. ~~`[test]` Add proxy error tests — unreachable backend returns 502, analytics recorded~~ DONE
7. `[feature]` Startup env validation in start-all — warn on missing RONIN_ROOT, ROGUE_REPO_ROOT, DATABASE_URL before spawning
8. `[test]` Add cloudflare DNS mock tests — f95 (ensure_cname), f97 (update A/AAAA) against wiremock
9. `[build]` systemd unit files for gd deployment — approuter.service + per-product services. Dep: gd node (n1) accessible
10. `[feature]` Circuit breaker per backend — if /health fails N times, skip proxy and return 503 until recovery. Prevents cascading timeouts
11. `[test]` Tunnel provider spawn tests — verify ngrok/bore/localtunnel spawn and stop lifecycle with mock binaries
12. `[docs]` TROUBLESHOOTING.md — CF token errors (expired, wrong permissions, 403), tunnel 1033/502/520, backend unreachable
13. `[feature]` Analytics retention — prune events older than N days (configurable via ROUTER_ANALYTICS_RETENTION_DAYS, default 30)
14. `[research]` P23 triple-lens on multi-tunnel architecture — are ngrok/bore/localtunnel providers tested in production? Worth keeping or dead code?
15. `[feature]` Webhook on product health change — POST to configurable URL when a product goes healthy->unhealthy or vice versa. Dep: /approuter/status health check loop
16. `[docs]` MULTI_TUNNEL_GUIDE.md — provider comparison (pricing, latency, reliability), when to enable each, env vars
17. `[build]` Reduce binary size — measure current release size, try strip + LTO + panic=abort if not already set. Target: <5MB
18. `[feature]` /approuter/status HTML view — human-readable dashboard (like /approuter/analytics) showing live product health grid
19. `[test]` Analytics storage tests — verify JSONL persistence, stats_all_sites aggregation, bot detection accuracy
20. `[research]` Rate limiting at proxy layer — should approuter enforce per-IP rate limits, or leave it to Cloudflare? Dep: cochranblock traffic patterns

---

**Cross-project dependencies:**
- Items 4, 10, 15: require /health endpoints on cochranblock (:8081), oakilydokily (:3000), rogue-repo (:3001), ronin-sites (:8000)
- Item 9: requires SSH access to gd (n1, kova-tunnel-god)
- Item 14: requires IRONHIVE swarm (n0-n3) for triple-lens dispatch
- Item 20: requires Cloudflare dashboard analytics to compare proxy-layer vs edge-layer rate limiting
