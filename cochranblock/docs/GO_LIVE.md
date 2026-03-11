<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Go Live — cochranblock.org

## 1. Prerequisites

- [x] Domain `cochranblock.org` in Cloudflare
- [x] Cloudflare API token (Zone.DNS Edit, Zone.Zone Read)
- [x] `PORTFOLIO_DOMAIN` and `CLOUDFLARE_API_TOKEN` in `.env`

## 2. CLI Options (No Scripts Required)

All runtime setup is via portfolio command-line options:

| Option | Description |
|--------|-------------|
| `--ensure-cloudflared` | Download cloudflared to `bin/` if missing. Exits after. |
| `--go-live` | Ensure cloudflared, set PORT=443 BIND=0.0.0.0, serve with tunnel. |
| `--no-tunnel` | Disable Cloudflare Tunnel (tunnel runs by default). |

**Go live (single command):**
```bash
# WSL (primary workflow)
./scripts/go-live-wsl.sh
```

Or manually: `cargo build --release -p portfolio && ./target/release/portfolio --go-live`

**Windows:** Cross-compile later via `cargo build --release --target x86_64-pc-windows-msvc -p portfolio`.

## 3. Quick Tunnel (No ACME, No Domain Setup)

Fastest way to get online — get a public URL in seconds:

```bash
# Terminal 1: Start portfolio (uses existing certs or falls back to HTTP)
./target/debug/portfolio

# Terminal 2: cloudflared tunnel
cloudflared tunnel --url https://localhost:443 --no-tls-verify
```

Cloudflared prints a URL like `https://random-words.trycloudflare.com` — that's your live site.

## 4. Obtain SSL Certs (ACME) — For cochranblock.org

```bash
# Staging first (no rate limits) — verify DNS-01 works
PORTFOLIO_ACME_STAGING=1 cargo run -p portfolio -- --acme-staging

# If staging succeeds, production
cargo run -p portfolio -- --acme
```

Certs save to `data/fullchain.pem` and `data/key.pem`.

**If DNS-01 fails:**
- Cloudflare may need 1–2 min to propagate the TXT record. Wait and retry.
- Verify token has **Zone.DNS Edit** and **Zone.Zone Read**.
- Verify `cochranblock.org` is in the same Cloudflare account as the token.
- Try apex only first: `PORTFOLIO_DOMAIN=cochranblock.org` (no www).

## 5. Start the Server

```bash
./target/debug/portfolio
```

Listens on https://0.0.0.0:443.

## 6. Expose via Cloudflare Tunnel (Named — cochranblock.org)

No port forward needed. Cloudflare proxies to your local server.

### Built-in tunnel (default)

The portfolio binary spawns cloudflared by default:

```bash
# From repo root (WSL or Windows)
cargo run -p portfolio
```

Or with the built binary:

```bash
./target/debug/portfolio
```

Use `--no-tunnel` to run without the tunnel.

**Requirements:**
- Run from repo root (so `data/` is found)
- `bin/cloudflared` — run `portfolio --ensure-cloudflared` to download, or use `cloudflared` in PATH
- `data/cloudflared.yml`

### Named tunnel (one-time setup)

```bash
cloudflared tunnel create portfolio
cloudflared tunnel route dns portfolio cochranblock.org
cloudflared tunnel route dns portfolio www.cochranblock.org
```

Config: `data/cloudflared.yml` (approuter-generated).

## 7. Run Both

```bash
cargo run -p portfolio
```

Tunnel runs by default. Use `--no-tunnel` to disable.

Visit https://cochranblock.org
