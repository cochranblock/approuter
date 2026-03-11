<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Getting Online — Caddy-like Auto HTTPS

The portfolio uses **lers** (Rust ACME/Let's Encrypt) with **Cloudflare DNS-01** for automatic HTTPS. No HTTP challenge needed — works behind NAT, firewalls, or before the server is reachable. **ACME is always enabled.** No escape hatch.

## Prerequisites

1. **Domain** — e.g. `cochranblock.org` with DNS managed by Cloudflare
2. **Cloudflare API token** — Zone.DNS Edit, Zone.Zone Read
3. **Build** — `cargo build -p portfolio`

### Build (WSL)

Primary workflow: build and run from WSL.

```bash
cargo build -p portfolio --release
```

**Windows cross-compile (later):** `cargo build --release --target x86_64-pc-windows-msvc -p portfolio`

## One-shot: Obtain certs then serve

```bash
# 1. Set env
export PORTFOLIO_DOMAIN=cochranblock.org,www.cochranblock.org
export CLOUDFLARE_API_TOKEN=your-token
export PORTFOLIO_DATA_DIR=data

# 2. Obtain certs (staging first to avoid rate limits)
PORTFOLIO_ACME_STAGING=1 cargo run -p portfolio -- --acme-staging

# 3. Production certs
cargo run -p portfolio -- --acme

# 4. Serve (certs in data/)
cargo run -p portfolio
```

## Caddy-like: Auto obtain on startup

When certs are missing, the server will attempt ACME before serving:

```bash
export PORTFOLIO_DOMAIN=cochranblock.org,www.cochranblock.org
export CLOUDFLARE_API_TOKEN=your-token
export PORTFOLIO_PORT=443
export PORTFOLIO_BIND=0.0.0.0

# First run: use staging to test
export PORTFOLIO_ACME_STAGING=1

cargo run -p portfolio
```

On success, certs are written to `PORTFOLIO_DATA_DIR` and the server serves HTTPS. On failure, it falls back to HTTP.

## Exposure options

| Method | Use case |
|--------|----------|
| **Port forward** | Router forwards 443 → your machine |
| **Cloudflare Tunnel** | `cloudflared tunnel` — no port forward, Cloudflare proxies to your origin |
| **VPS** | Run on a server with public IP |

With Cloudflare Tunnel, set origin to `https://localhost:443` or your bind address.

## Env reference

| Variable | Purpose |
|----------|---------|
| `PORTFOLIO_DOMAIN` | Comma-separated domains for ACME (e.g. `example.com,www.example.com`) |
| `CLOUDFLARE_API_TOKEN` | Cloudflare API token for DNS-01 |
| `PORTFOLIO_ACME_STAGING` | If set, use Let's Encrypt staging (no rate limits) |
| `PORTFOLIO_DATA_DIR` | Where certs are stored (`fullchain.pem`, `key.pem`) |
