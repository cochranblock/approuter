#!/bin/bash
# Restart cochranblock only. Does not touch oakilydokily, router.
# Usage: ./scripts/restart-cochranblock.sh [--bg]
# --bg: run in background and return. Otherwise foreground (exec).

set -e
cd "$(dirname "$0")/.."
REPO_ROOT="$(pwd)"

[ -f .env ] && set -a && source .env && set +a

export PORTFOLIO_PORT=443
export PORTFOLIO_BIND=0.0.0.0

pkill -f cochranblock 2>/dev/null && echo "Stopped cochranblock" || true
sleep 1

cargo build --release -p cochranblock

BIN="$REPO_ROOT/target/release/cochranblock"
if ! getcap "$BIN" 2>/dev/null | grep -q cap_net_bind_service; then
  echo "One-time: allow port 443 (sudo)"
  sudo setcap 'cap_net_bind_service=+ep' "$BIN"
fi

if [ "$1" = "--bg" ]; then
  echo "Starting cochranblock (443) in background..."
  nohup "$BIN" --go-live </dev/null >/dev/null 2>&1 &
  echo "cochranblock PID $!"
else
  echo "Starting cochranblock (443)..."
  exec "$BIN" --go-live
fi
