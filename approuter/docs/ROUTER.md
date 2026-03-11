<!-- Unlicense — cochranblock.org -->
# App Router

Reverse proxy that routes traffic to the correct backend based on URL (Host header or path).

## Architecture

```
Cloudflare Tunnel → router:8080 → cochranblock:443 (cochranblock.org)
                                    └──→ oakilydokily:3000 (oakilydokily.com or /oakilydokily)
```

## Routing Modes

### Host-based (recommended)

Set `ROUTER_OAKILYDOKILY_HOST=oakilydokily.com`. Requests with that Host header go to oakilydokily; all others go to cochranblock.

### Path-based

Set `ROUTER_OAKILYDOKILY_PATH=/oakilydokily`. Requests to `/oakilydokily` or `/oakilydokily/*` go to oakilydokily; the path prefix is stripped when forwarding.

## Env Vars

| Var | Default | Description |
|-----|---------|-------------|
| ROUTER_PORT | 8080 | Port the router listens on |
| ROUTER_BIND | 127.0.0.1 | Bind address |
| ROUTER_COCHRANBLOCK_URL | https://127.0.0.1:443 | cochranblock backend |
| ROUTER_OAKILYDOKILY_URL | http://127.0.0.1:3000 | oakilydokily backend |
| ROUTER_OAKILYDOKILY_HOST | — | Hostname for oakilydokily (Host-based routing) |
| ROUTER_OAKILYDOKILY_PATH | — | Path prefix for oakilydokily (path-based routing) |
| ROUTER_RONIN_URL | http://127.0.0.1:8000 | Ronin Sites backend |
| ROUTER_RONIN_HOST | — | Comma-separated hostnames for Ronin (exact match) |
| ROUTER_RONIN_SUFFIX | — | Suffix match (e.g. .ronin-sites.pro) — any host ending with this routes to Ronin |

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

## App Registration (centralized CF_TOKEN)

Apps can register themselves with the router. The router holds **CF_TOKEN** (and CF_ACCOUNT_ID) centrally; apps no longer need Cloudflare API tokens.

### Register an app

```bash
curl -X POST http://127.0.0.1:8080/approuter/register \
  -H "Content-Type: application/json" \
  -d '{"app_id":"oakilydokily","hostnames":["oakilydokily.com","www.oakilydokily.com"],"backend_url":"http://127.0.0.1:3000"}'
```

- **app_id**: Unique identifier (used for unregister).
- **hostnames**: Host headers that route to this app.
- **backend_url**: Upstream URL (e.g. `http://127.0.0.1:3000`).

On success, the router updates the Cloudflare tunnel ingress so these hostnames route to the router. Registry is persisted to `data/registry.json`.

### List / unregister

```bash
curl http://127.0.0.1:8080/approuter/apps
curl -X DELETE http://127.0.0.1:8080/approuter/apps/oakilydokily
```

### DNS API (for apps needing dynamic IP)

Apps that update A/AAAA records (e.g. dynamic IP) can call the router instead of using CF_TOKEN:

```bash
curl -X POST http://127.0.0.1:8080/approuter/dns/update-a \
  -H "Content-Type: application/json" \
  -d '{"zone_id":"xxx","record_id":"yyy","content":"1.2.3.4"}'
```

### Centralized tokens

Put Cloudflare credentials in the **router** `.env` only:

```
CF_TOKEN=your_api_token
CF_ACCOUNT_ID=your_account_id
CF_TUNNEL_ID=b12525df-6971-4c47-9a0d-61ee57a5cbd5
```

Apps (oakilydokily, roguerepo, cochranblock) no longer need CF_TOKEN. Use `--setup-oakilydokily` or `--setup-roguerepo` from the router for one-time DNS setup.
