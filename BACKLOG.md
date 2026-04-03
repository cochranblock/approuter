<!-- Unlicense — cochranblock.org -->

# Backlog

Prioritized stack. Top = most important. Max 20 items.
Tags: `[build]` `[test]` `[docs]` `[feature]` `[fix]` `[research]`

---

1. ~~`[test]` Add integration test for API key auth~~ DONE (api_hardening.rs)
2. ~~`[test]` Add integration test for hostname collision~~ DONE (api_hardening.rs)
3. ~~`[test]` Add integration test for /approuter/status~~ DONE (api_hardening.rs)
4. ~~`[feature]` Post-spawn health check loop in start-all — poll /health on each backend before declaring ready~~ DONE (run.rs: 30s polling loop, prints [ready]/[timeout] per backend)
5. ~~`[fix]` Replace reqwest::Client::new() in api.rs f110 with shared lazy client~~ DONE
6. ~~`[test]` Add proxy error tests — unreachable backend returns 502, analytics recorded~~ DONE
7. ~~`[feature]` Startup env validation in start-all — warn on missing env vars before spawning~~ DONE
8. ~~`[test]` Add cloudflare DNS mock tests — f95 (CNAME create), f97 (A/AAAA update) via wiremock~~ DONE
9. `[build]` systemd unit files for gd deployment — approuter.service + per-product services. Dep: gd node (n1) accessible
10. `[feature]` Circuit breaker per backend — if /health fails N times, skip proxy and return 503 until recovery. Prevents cascading timeouts
11. `[test]` Tunnel provider spawn tests — verify ngrok/bore/localtunnel spawn and stop lifecycle with mock binaries
12. ~~`[docs]` TROUBLESHOOTING.md — CF token errors, backend unreachable, API auth, hostname collision~~ DONE
13. ~~`[feature]` Analytics retention — prune old JSONL files on startup (ROUTER_ANALYTICS_RETENTION_DAYS, default 30)~~ DONE
14. `[research]` P23 triple-lens on multi-tunnel architecture — are ngrok/bore/localtunnel providers tested in production? Worth keeping or dead code?
15. `[feature]` Webhook on product health change — POST to configurable URL when a product goes healthy->unhealthy or vice versa. Dep: /approuter/status health check loop
16. `[docs]` MULTI_TUNNEL_GUIDE.md — provider comparison (pricing, latency, reliability), when to enable each, env vars
17. ~~`[build]` Binary size — already 4.8MB with LTO+strip+panic=abort+codegen-units=1. Target <5MB met~~ DONE
18. ~~`[feature]` /approuter/status/ HTML dashboard — product health grid, auto-refresh 15s~~ DONE
19. `[test]` Analytics storage tests — verify JSONL persistence, stats_all_sites aggregation, bot detection accuracy
20. `[research]` Rate limiting at proxy layer — should approuter enforce per-IP rate limits, or leave it to Cloudflare? Dep: cochranblock traffic patterns

---

**Cross-project dependencies:**
- Items 10, 15: require /health endpoints on cochranblock (:8081), oakilydokily (:3000), rogue-repo (:3001), ronin-sites (:8000)
- Item 9: requires SSH access to gd (n1, kova-tunnel-god)
- Item 14: requires IRONHIVE swarm (n0-n3) for triple-lens dispatch
- Item 20: requires Cloudflare dashboard analytics to compare proxy-layer vs edge-layer rate limiting

---

## UI/UX Analysis — Admin API (2026-04-03)

All management endpoints traced:

| Endpoint | Method | Auth | Gap |
|---|---|---|---|
| `/approuter/register` | POST | Bearer | No GET to inspect a single app by ID |
| `/approuter/apps` | GET | none | No filter/search; returns all apps flat |
| `/approuter/apps/:app_id` | DELETE | none | No auth on delete — anyone can unregister |
| `/approuter/dns/update-a` | POST | Bearer | No GET to read current A record |
| `/approuter/tunnel` | GET | none | Returns legacy CF child only; multi-tunnel state separate |
| `/approuter/tunnel/stop\|ensure\|restart\|fix` | POST | Bearer on stop/restart/fix; NOT on ensure | Inconsistency: `/tunnel/ensure` skips auth |
| `/approuter/tunnels` | GET | none | Good — multi-tunnel status |
| `/approuter/tunnels/:provider/start\|stop` | POST | none | No auth on tunnel start/stop — any client can toggle |
| `/approuter/analytics/data\|recent` | GET | none | No auth; analytics readable without key |
| `/approuter/status` | GET | none | No auth; fine for monitoring |
| `/approuter/openapi.json` | GET | none | Spec may be stale vs actual routes |
| `/approuter/google/apis` | GET | none | Leaks service enumeration to unauthenticated clients |

**Top gaps identified:**
1. `DELETE /approuter/apps/:app_id` — no API key check (any client can unregister apps)
2. `POST /approuter/tunnels/:provider/start|stop` — no auth (any client can kill tunnels)
3. `POST /approuter/tunnel/ensure` — inconsistent: all other write tunnel ops require auth
4. `GET /approuter/apps/:app_id` — missing; only bulk list exists
5. `PATCH /approuter/apps/:app_id` — no partial update; full re-register required
6. `/approuter/openapi.json` — spec likely diverged from actual routes (tunnels/*, analytics/* not reflected)

## Feature Gap Analysis — BACKLOG vs Implemented

| Backlog Item | Status | Notes |
|---|---|---|
| Circuit breaker (item 10) | Not started | No per-backend failure counting in proxy.rs |
| Webhook on health change (item 15) | Not started | /approuter/status runs ad-hoc; no persistent state or notification |
| Tunnel provider spawn tests (item 11) | Not started | No mock binary infra yet |
| Analytics storage tests (item 19) | Not started | JSONL write/read/prune paths untested |
| systemd unit files (item 9) | Not started | Needs gd access |
| MULTI_TUNNEL_GUIDE.md (item 16) | Not started | Docs only |
| Rate limiting research (item 20) | Not started | Needs traffic data |
| GET /apps/:app_id | Not implemented | Unregistering requires knowing app_id; no lookup |
| Auth on DELETE /apps and tunnel start/stop | Not implemented | Security gap (see UI/UX above) |
