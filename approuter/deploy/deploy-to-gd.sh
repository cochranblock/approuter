#!/bin/bash
# Deploy approuter stack to gd (kova-tunnel-god). Run from workspace root.
set -e

HOST=gd
REMOTE_ROOT=/home/mcochran/cochranblock
WS_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"

echo "=== Deploying to $HOST ($REMOTE_ROOT) ==="

# 0. Ensure DNS on gd (required for Rust install)
if ! ssh "$HOST" "getent hosts static.rust-lang.org" &>/dev/null; then
  echo "Fixing DNS on $HOST (adding 8.8.8.8)..."
  ssh "$HOST" "echo 'nameserver 8.8.8.8' | su -c 'tee -a /etc/resolv.conf'" 2>/dev/null || true
  sleep 1
  if ! ssh "$HOST" "getent hosts static.rust-lang.org" &>/dev/null; then
    echo "ERROR: Still no DNS. Manually run on $HOST as root:"
    echo "  echo 'nameserver 8.8.8.8' >> /etc/resolv.conf"
    exit 1
  fi
fi

# 1. Create remote dir
ssh "$HOST" "mkdir -p $REMOTE_ROOT"

# 2. Sync workspace via tar (rsync may not be on minimal Debian)
echo "Syncing workspace..."
SYNC_DIRS="kova-core kova-web cochranblock approuter oakilydokily kova rogue-repo vendor Cargo.toml Cargo.lock"
for d in whyyoulying wowasticker; do [ -d "$WS_ROOT/$d" ] && SYNC_DIRS="$SYNC_DIRS $d"; done
(cd "$WS_ROOT" && tar cf - --exclude 'target' --exclude '.git' --exclude 'node_modules' $SYNC_DIRS) | ssh "$HOST" "cd $REMOTE_ROOT && tar xf -"

# 3. Copy .env
if [ -f "$WS_ROOT/approuter/.env" ]; then
  echo "Syncing approuter/.env..."
  scp "$WS_ROOT/approuter/.env" "$HOST:$REMOTE_ROOT/approuter/.env"
else
  echo "WARNING: approuter/.env not found. Copy manually or create on server."
fi

# 4. Install build deps (cc, etc) + Rust + cloudflared + jq
echo "Ensuring build deps on $HOST..."
ssh "$HOST" "command -v cc" &>/dev/null || \
  ssh "$HOST" "su -c 'apt-get update -qq && apt-get install -y build-essential pkg-config libssl-dev'"
echo "Ensuring Rust and cloudflared on $HOST..."
if ! ssh "$HOST" "command -v cargo" &>/dev/null; then
  echo "Installing Rust (copying installer from local, gd may have DNS issues)..."
  curl -sSf https://sh.rustup.rs -o /tmp/rustup-init.sh
  scp /tmp/rustup-init.sh "$HOST:/tmp/rustup-init.sh"
  ssh "$HOST" "sh /tmp/rustup-init.sh -y && rm /tmp/rustup-init.sh"
fi
ssh "$HOST" bash -s << 'REMOTE'
  export PATH="$HOME/.cargo/bin:$HOME/bin:$PATH"
  mkdir -p "$HOME/bin"
  if ! command -v cloudflared &>/dev/null || [ ! -s "$HOME/bin/cloudflared" ]; then
    wget -q "https://github.com/cloudflare/cloudflared/releases/download/2026.3.0/cloudflared-linux-amd64" -O "$HOME/bin/cloudflared"
    chmod +x "$HOME/bin/cloudflared"
    [ -s "$HOME/bin/cloudflared" ] || { echo "cloudflared download failed"; exit 1; }
  fi
  if ! command -v jq &>/dev/null; then
    wget -q "https://github.com/jqlang/jq/releases/download/jq-1.7.1/jq-linux-amd64" -O "$HOME/bin/jq" 2>/dev/null || true
    [ -f "$HOME/bin/jq" ] && chmod +x "$HOME/bin/jq"
  fi
REMOTE

# 5. Build on server
echo "Building on $HOST..."
ssh "$HOST" "cd $REMOTE_ROOT && export PATH=\"\$HOME/.cargo/bin:\$PATH\" && cargo build --release -p approuter -p cochranblock --features approuter -p oakilydokily --features approuter"
ssh "$HOST" "cd $REMOTE_ROOT/rogue-repo && export PATH=\"\$HOME/.cargo/bin:\$PATH\" && cargo build --release -p rogue-repo"

# 5b. Ensure registry config for approuter (base/config/registry.json)
ssh "$HOST" "mkdir -p $REMOTE_ROOT/config && cp -n $REMOTE_ROOT/approuter/config/registry.json $REMOTE_ROOT/config/registry.json 2>/dev/null || true"

# 6. Install user systemd units (no sudo required)
echo "Installing systemd units..."
ssh "$HOST" "mkdir -p ~/.config/systemd/user"
for svc in approuter cochranblock oakilydokily rogue-repo cloudflared-cochranblock; do
  scp "$WS_ROOT/approuter/deploy/systemd/${svc}.service" "$HOST:~/.config/systemd/user/"
done

# 7. Make cloudflared-start.sh executable
ssh "$HOST" "chmod +x $REMOTE_ROOT/approuter/deploy/cloudflared-start.sh"

# 8. Stop local Mac processes (so tunnel switches to gd)
echo "Stopping local stack (tunnel will move to gd)..."
pkill -f "cloudflared.*tunnel" 2>/dev/null || true
pkill -f "target/release/approuter" 2>/dev/null || true
pkill -f "target/release/cochranblock" 2>/dev/null || true
pkill -f "target/release/oakilydokily" 2>/dev/null || true
pkill -f "target/release/rogue-repo" 2>/dev/null || true
sleep 2

# 9. Enable and start on gd (user systemd)
echo "Starting services on $HOST..."
ssh "$HOST" "export PATH=\$HOME/bin:\$PATH; systemctl --user daemon-reload && systemctl --user enable approuter cochranblock oakilydokily rogue-repo cloudflared-cochranblock && systemctl --user restart approuter cochranblock oakilydokily rogue-repo cloudflared-cochranblock"

echo ""
echo "=== Deploy complete ==="
echo "Check status: ssh $HOST 'systemctl --user status approuter cochranblock oakilydokily rogue-repo cloudflared-cochranblock'"
echo "Logs: ssh $HOST 'journalctl --user -u cloudflared-cochranblock -f'"
