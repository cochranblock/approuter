<!-- Unlicense — cochranblock.org -->
<!-- Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->
# Cloudflare Tunnel via systemd (hardened)

Approuter and cloudflared as systemd user services. Startup order: approuter first, then cloudflared.

## Provision (one script)

```bash
/home/mcochran/scripts/provision-approuter-systemd.sh
```

Installs approuter.service and cloudflared.service. Cloudflared has `After=approuter.service` so it starts only after approuter is up.

## One-time setup (before provision)

1. **Ensure cloudflared binary exists**:
   ```bash
   cd /home/mcochran && cargo run -p approuter -- fix-tunnel
   ```

2. **Configure tunnel ingress** (when adding hostnames):
   ```bash
   cd /home/mcochran && set -a && . ./cochranblock/.env && set +a
   ROUTER_CONFIG_DIR=/home/mcochran/cochranblock cargo run -p approuter -- --update-tunnel
   ```

3. **Enable lingering** (services run without login session):
   ```bash
   loginctl enable-linger $USER
   ```

## Verify

```bash
systemctl --user status approuter
systemctl --user status cloudflared
curl -sI https://cochranblock.org | head -1
```

## Architecture

See [docs/STACK_ARCHITECTURE.md](../../docs/STACK_ARCHITECTURE.md).
