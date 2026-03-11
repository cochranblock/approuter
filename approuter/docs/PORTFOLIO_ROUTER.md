<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Router

Reverse proxy that routes traffic to the correct backend based on URL (Host header or path).

## Architecture

```
Cloudflare Tunnel → router:8080 → cochranblock:443 (cochranblock.org)
                                    └──→ oakilydokily:3000 (kaylie.cochranblock.org or /kaylie)
```

## Routing Modes

### Host-based (recommended)

Set `ROUTER_KAYLIE_HOST=kaylie.cochranblock.org`. Requests with that Host header go to Kaylie; all others go to cochranblock.

### Path-based

Set `ROUTER_KAYLIE_PATH=/kaylie`. Requests to `/kaylie` or `/kaylie/*` go to Kaylie; the path prefix is stripped when forwarding.

## Env Vars

| Var | Default | Description |
|-----|---------|-------------|
| ROUTER_PORT | 8080 | Port the router listens on |
| ROUTER_BIND | 127.0.0.1 | Bind address |
| ROUTER_PORTFOLIO_URL | https://127.0.0.1:443 | cochranblock backend |
| ROUTER_KAYLIE_URL | http://127.0.0.1:3000 | Kaylie backend |
| ROUTER_KAYLIE_HOST | — | Hostname for Kaylie (Host-based routing) |
| ROUTER_KAYLIE_PATH | — | Path prefix for Kaylie (path-based routing) |

## Cloudflare Tunnel Config

To route all traffic through the router, update the tunnel config via API:

```bash
export CF_ACCOUNT_ID=your_account_id
export CF_TUNNEL_ID=b12525df-6971-4c47-9a0d-61ee57a5cbd5  # or from config
export CF_TOKEN=your_api_token  # or CLOUDFLARE_API_TOKEN

cargo run -p approuter -- --update-tunnel
```

This sets ingress so cochranblock.org, www.cochranblock.org, and kaylie.cochranblock.org all route to `http://127.0.0.1:8080` (the router).

**DNS:** Add a CNAME for kaylie.cochranblock.org pointing to the tunnel (e.g. `b12525df-6971-4c47-9a0d-61ee57a5cbd5.cfargotunnel.com` or your tunnel's DNS target).

## Run Order

1. **cochranblock** on 443 (or ROUTER_PORTFOLIO_URL)
2. **oakilydokily** on 3000 (or ROUTER_KAYLIE_URL)
3. **router** on 8080
4. **cloudflared** tunnel pointing to http://localhost:8080 (after `--update-tunnel`)

## Example

```bash
# Terminal 1: cochranblock
PORT=443 BIND=0.0.0.0 cargo run -p portfolio -- --go-live

# Terminal 2: oakilydokily
BIND=127.0.0.1 PORT=3000 cargo run -p oakilydokily

# Terminal 3: router (Host-based)
ROUTER_KAYLIE_HOST=kaylie.cochranblock.org cargo run -p approuter

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
