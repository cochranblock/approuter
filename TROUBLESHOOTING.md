<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Troubleshooting

Quick fixes for common approuter issues. For tunnel-specific errors (1033, 502, 520-526), see [approuter/docs/TUNNEL_ERRORS.md](approuter/docs/TUNNEL_ERRORS.md).

## Cloudflare Token Errors

### "CF_TOKEN or CLOUDFLARE_API_TOKEN required"

Token not set. Add to `approuter/.env`:
```
CF_TOKEN=your_api_token_here
```

### 403 Forbidden from Cloudflare API

Token lacks permissions. The token needs:
- **Zone:DNS:Edit** (for CNAME creation)
- **Account:Cloudflare Tunnel:Edit** (for ingress sync)
- **Zone:Zone:Read** (for zone lookup)

Create a new token at [dash.cloudflare.com/profile/api-tokens](https://dash.cloudflare.com/profile/api-tokens) with these permissions.

Verify your token:
```bash
cargo run -p approuter -- cf-token-check
```

### 401 Unauthorized from Cloudflare API

Token expired or revoked. Generate a new one and update `approuter/.env`.

### Stale tunnel port (55842, random port in dashboard)

Approuter syncs the listen port to Cloudflare on startup. If you see a stale port:
1. Ensure `--no-tunnel` is NOT set when running in production
2. Run `cargo run -p approuter --release -- start-all` (syncs before spawning cloudflared)

Fixed in commit `37631f1`. If still occurring, manually sync:
```bash
CF_TOKEN=xxx CF_ACCOUNT_ID=xxx cargo run -p approuter -- --update-tunnel
```

## Backend Unreachable (502 from proxy)

The proxy returns 502 when a backend doesn't respond within 30s or refuses connection.

### Check what's running

```bash
curl http://127.0.0.1:8080/approuter/status
```

This health-checks all products and shows which are up/down.

### Individual backend checks

| Product | Check | Expected |
|---------|-------|----------|
| cochranblock | `curl http://127.0.0.1:8081/health` | 200 |
| oakilydokily | `curl http://127.0.0.1:3000/health` | 200 |
| rogue-repo | `curl http://127.0.0.1:3001/health` | 200 |
| ronin-sites | `curl http://127.0.0.1:8000/health` | 200 |
| approuter | `curl http://127.0.0.1:8080/health` | 200 |

### Restart a single product

```bash
cargo run -p approuter -- restart-cochranblock
cargo run -p approuter -- restart-oakilydokily
cargo run -p approuter -- restart-roguerepo
cargo run -p approuter -- restart-ronin
cargo run -p approuter -- restart              # approuter itself
```

### Restart everything

```bash
cargo run -p approuter --release -- start-all
```

## API Key Auth (401 on register/unregister)

If `ROUTER_API_KEY` is set, mutating endpoints require `Authorization: Bearer <key>`:

```bash
curl -X POST http://127.0.0.1:8080/approuter/register \
  -H "Authorization: Bearer your-key-here" \
  -H "Content-Type: application/json" \
  -d '{"app_id":"test","hostnames":["test.com"],"backend_url":"http://127.0.0.1:9999"}'
```

If you get 401 and don't expect it, check if `ROUTER_API_KEY` is set in your environment or `.env`.

Read-only endpoints (`/approuter/apps`, `/approuter/status`, `/health`) never require auth.

## Hostname Collision (409 on register)

Two different apps cannot claim the same hostname. If you get 409 Conflict:
- Check which app owns the hostname: `curl http://127.0.0.1:8080/approuter/apps`
- Unregister the old app first, or update it with the same `app_id`

Self-update (same `app_id`, different hostnames) is always allowed.

## start-all Warnings

The `start-all` command prints warnings if env vars are missing:
```
=== start-all env warnings ===
  ! CF_TOKEN not set — tunnel will not start
  ! RONIN_ROOT not set and ronin-sites not found — will be skipped
==============================
```

These are non-fatal — approuter still starts, but the affected products are skipped.

## Debug Endpoints

| Endpoint | What it shows |
|----------|--------------|
| GET /approuter/status | Live health of all products |
| GET /approuter/apps | Registered apps and hostnames |
| GET /approuter/tunnel | Tunnel PID and running state |
| GET /approuter/tunnels | All tunnel providers status |
| GET /approuter/analytics/recent | Last 50 requests with status codes |
| GET /approuter/openapi.json | Full API spec |

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
