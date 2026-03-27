<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# App Router (legacy: PORTFOLIO_ROUTER)

See [ROUTER.md](ROUTER.md) for current docs. This file kept for reference.

Reverse proxy that routes traffic to the correct backend based on URL (Host header or path).

## Architecture

```
Cloudflare Tunnel → approuter:8080 → cochranblock:8081 (cochranblock.org)
                                       └──→ oakilydokily:3000 (oakilydokily.com or /oakilydokily)
```

## Env Vars

| Var | Default | Description |
|-----|---------|-------------|
| ROUTER_COCHRANBLOCK_URL | http://127.0.0.1:8081 | cochranblock backend |
| ROUTER_OAKILYDOKILY_URL | http://127.0.0.1:3000 | oakilydokily backend |
| ROUTER_OAKILYDOKILY_HOST | — | Hostname for oakilydokily (Host-based routing) |
| ROUTER_OAKILYDOKILY_PATH | — | Path prefix for oakilydokily (path-based routing) |

## Example

```bash
# Terminal 1: cochranblock
PORT=443 BIND=0.0.0.0 cargo run -p cochranblock -- --go-live

# Terminal 2: oakilydokily
BIND=127.0.0.1 PORT=3000 cargo run -p oakilydokily

# Terminal 3: router (Host-based)
ROUTER_OAKILYDOKILY_HOST=oakilydokily.com cargo run -p approuter

# Update tunnel (one-time)
CF_ACCOUNT_ID=xxx CF_TOKEN=xxx cargo run -p approuter -- --update-tunnel

### roguerepo.io (Rogue Repo product domain)

1. **Add DNS** (one-time, requires roguerepo.io zone in Cloudflare):
   ```bash
   CF_TOKEN=xxx cargo run -p approuter -- --setup-roguerepo
   ```
   Creates CNAME for roguerepo.io and www.roguerepo.io → tunnel.

2. **Update tunnel** (adds roguerepo.io to ingress):
   ```bash
   CF_ACCOUNT_ID=xxx CF_TOKEN=xxx cargo run -p approuter -- --update-tunnel
   ```

3. **Local config** — `data/cloudflared.yml` (approuter-generated) includes roguerepo.io. Router routes it to cochranblock (Rogue Repo product).
```