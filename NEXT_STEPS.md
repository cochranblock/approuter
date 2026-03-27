<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Next Steps

## Deployment

Deploy approuter and backends to gd (Debian VPS). See [approuter/docs/TUNNEL_SYSTEMD.md](approuter/docs/TUNNEL_SYSTEMD.md) for systemd setup.

### Quick deploy

```bash
cargo run -p approuter --release -- start-all
```

This spawns approuter (--no-tunnel), cochranblock, oakilydokily, rogue-repo, ronin-sites, then gets a tunnel token and spawns cloudflared.

### Env required

Put Cloudflare credentials in `approuter/.env`:

```
CF_TOKEN=your_api_token
CF_ACCOUNT_ID=your_account_id
CF_TUNNEL_ID=your_tunnel_id
```

### Per-service restart

```bash
cargo run -p approuter -- restart
cargo run -p approuter -- restart-cochranblock
cargo run -p approuter -- restart-oakilydokily
cargo run -p approuter -- restart-ronin
cargo run -p approuter -- restart-roguerepo
```

## Verify

```bash
curl http://127.0.0.1:8080/approuter/apps       # List registered apps
curl http://127.0.0.1:8080/approuter/tunnel      # Tunnel status
curl http://127.0.0.1:8080/approuter/tunnels     # Multi-tunnel status
curl http://127.0.0.1:8080/approuter/analytics/data  # Analytics
```
