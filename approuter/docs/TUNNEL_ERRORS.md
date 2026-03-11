<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Cloudflare Tunnel Errors — Fix for Everything

Common Cloudflare tunnel/origin errors and how to fix them for this stack (approuter + cloudflared → localhost:8080).

## Quick Fix (Most Errors)

```bash
cd /home/mcochran
cargo run -p approuter -- fix-tunnel
```

Or restart each project individually: `@restart_cb` `@restart_od` `@restart_rtr` `@restart_ronin` (blocking)

---

## Error 1033: Argo Tunnel Error

**Meaning:** Cloudflare cannot find a healthy `cloudflared` instance. Tunnel is down, inactive, or disconnected.

**Fix:** Call the fix API (ensures cloudflared + restarts tunnel):
```bash
curl -X POST http://127.0.0.1:8080/approuter/tunnel/fix
```
Or if approuter is down: `approuter fix-tunnel` or `approuter restart` (blocking)

---

## Error 502: Bad Gateway

**Meaning:** Tunnel works but cannot reach the local app (approuter or backend down).

**Fix:**
- Approuter must be listening on 8080: `curl -s http://127.0.0.1:8080/` (with `Host: cochranblock.org`)
- Backends must be running: cochranblock (443), oakilydokily (3000), ronin-sites (8000)
- Run backends: `@restart_cb` `@restart_od` `@restart_rtr` `@restart_ronin` (one per terminal, blocking)

---

## Error 520: Web Server Returns Unknown Error

**Meaning:** Origin returned an empty, unknown, or unexpected response.

**Fix:** Usually backend crash or invalid response. Check backend logs. Restart the affected backend: `approuter restart-cochranblock` `approuter restart-oakilydokily` `approuter restart` or `approuter restart-ronin`

---

## Error 521: Web Server Is Down

**Meaning:** Cloudflare connected but origin refused the connection.

**Fix:** Origin (approuter:8080) not listening. Start approuter: `approuter restart` (blocking)

---

## Error 522: Connection Timed Out

**Meaning:** Cloudflare could not complete a TCP connection to the origin.

**Fix:** With tunnel, this usually means cloudflared lost connection. Restart: `cargo run -p approuter -- fix-tunnel`

---

## Error 523: Origin Is Unreachable

**Meaning:** Cloudflare could not reach the origin.

**Fix:** Same as 522. Tunnel down or cloudflared disconnected. Run `cargo run -p approuter -- fix-tunnel`

---

## Error 524: A Timeout Occurred

**Meaning:** Connection established but origin took too long to respond.

**Fix:** Backend is slow or hung. Check cochranblock/oakilydokily/ronin-sites. Restart the slow service.

---

## Error 525: SSL Handshake Failed

**Meaning:** TLS handshake between Cloudflare and origin failed.

**Fix:** With tunnel, traffic to localhost:8080 is HTTP (no TLS). If you see 525, you may be using proxy (orange cloud) to a direct origin. For tunnel-only setup, ensure DNS uses CNAME → tunnel, not A record to your IP.

---

## Error 526: Invalid SSL Certificate

**Meaning:** Origin's SSL certificate is invalid (expired, self-signed, wrong domain).

**Fix:** With tunnel, no origin SSL needed (tunnel uses HTTP to localhost). If using proxy to direct origin (e.g. ronin-sites with Caddy), ensure Cloudflare Origin cert is valid. For tunnel setup: run `cargo run -p approuter -- --setup-ronin` and `--update-tunnel` so traffic goes through tunnel, not direct origin.

---

## Checklist (All Errors)

| Check | Command |
|-------|---------|
| cloudflared installed | Approuter ensures on start. Or `curl -X POST http://127.0.0.1:8080/approuter/tunnel/ensure` |
| Credentials exist | `data/cloudflared.yml` → `credentials-file` path (e.g. `/mnt/c/Users/<you>/.cloudflared/<tunnel-id>.json`). Override with `CLOUDFLARED_CREDENTIALS` env. |
| Config exists | `data/cloudflared.yml` (approuter-generated) with correct `tunnel:` and `ingress:` |
| Approuter running | `curl -s http://127.0.0.1:8080/approuter/apps` |
| Tunnel running | `curl -s http://127.0.0.1:8080/approuter/tunnel` → `running: true` |
| Backends running | cochranblock, oakilydokily, ronin-sites |

---

## Stop Tunnel (Without Killing Other Projects)

Use the API — never `pkill cloudflared`:

```bash
curl -X POST http://127.0.0.1:8080/approuter/tunnel/stop
```

---

## See Also

- [TUNNEL_1033.md](TUNNEL_1033.md) — Focused 1033 fix
- [ROUTER.md](ROUTER.md) — Approuter and tunnel setup
- [Cloudflare Error 1033](https://developers.cloudflare.com/support/troubleshooting/http-status-codes/cloudflare-1xxx-errors/error-1033/)
