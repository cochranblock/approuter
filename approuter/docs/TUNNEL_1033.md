<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->
# Fixing Cloudflare Tunnel Error 1033

**Error 1033** = Cloudflare cannot find a healthy `cloudflared` instance. The tunnel is down, inactive, or degraded.

## Quick Fix

```bash
cd /home/mcochran
cargo run -p approuter -- fix-tunnel
```

With systemd cloudflared (recommended): `fix-tunnel` restarts the systemd service. See [TUNNEL_SYSTEMD.md](TUNNEL_SYSTEMD.md).

Or via API (approuter must be running):

```bash
curl -X POST http://127.0.0.1:8080/approuter/tunnel/fix
```

Or restart approuter (ensures cloudflared + spawns on start, blocking):

```bash
approuter restart
```

## Checklist

| Check | Command / Action |
|-------|------------------|
| cloudflared installed | Approuter ensures on start via API. Or `curl -X POST http://127.0.0.1:8080/approuter/tunnel/ensure` |
| Credentials exist | `data/cloudflared.yml` references `credentials-file`. Verify that path exists (e.g. `/mnt/c/Users/<you>/.cloudflared/<tunnel-id>.json`). Override with `CLOUDFLARED_CREDENTIALS`. |
| ROUTER_CONFIG_DIR | Approuter needs `data/` and `bin/`. `restart-router.sh` sets `ROUTER_CONFIG_DIR` to cochranblock |
| Tunnel config | `data/cloudflared.yml` (approuter-generated) must exist with correct `tunnel:` ID and `ingress:` hostnames |
| Approuter running | `curl -s http://127.0.0.1:8080/approuter/apps` |
| oakilydokily running | `curl -s http://127.0.0.1:3000/health` or similar |

## Credentials Path

The config uses:

```yaml
credentials-file: /mnt/c/Users/mclar/.cloudflared/b12525df-6971-4c47-9a0d-61ee57a5cbd5.json
```

If your Windows username differs, set `CLOUDFLARED_CREDENTIALS` env to point to your tunnel credentials JSON.

## Restart (one project per terminal, blocking)

```bash
cd /home/mcochran
approuter restart
approuter restart-oakilydokily
approuter restart-ronin
approuter restart-cochranblock
```

## Verify Tunnel Status

```bash
cloudflared tunnel list
# Or in Cloudflare dashboard: Networks > Connectors > Cloudflare Tunnels
```

Status should be **Healthy**. If **Down** or **Inactive**, cloudflared is not connected.

## See Also

- [TUNNEL_ERRORS.md](TUNNEL_ERRORS.md) — All Cloudflare tunnel errors (1033, 502, 520–526)
