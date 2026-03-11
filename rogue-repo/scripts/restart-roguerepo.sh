#!/bin/bash
# Restart roguerepo (repo-api) only.
# Usage: ./scripts/restart-roguerepo.sh [--bg]

set -e
cd "$(dirname "$0")/.."
REPO_ROOT="$(pwd)"

[ -f .env ] && set -a && source .env && set +a

pkill -f rogue-repo 2>/dev/null && echo "Stopped rogue-repo" || true
sleep 1

cargo build --release -p rogue-repo

BIN="$REPO_ROOT/target/release/rogue-repo"
if [ ! -f "$BIN" ]; then
  echo "ERROR: rogue-repo not found. Build with: cargo build -p rogue-repo"
  exit 1
fi

if [ "$1" = "--bg" ]; then
  echo "Starting roguerepo (3001) in background..."
  nohup "$BIN" </dev/null >/dev/null 2>&1 &
  echo "rogue-repo PID $!"
else
  echo "Starting roguerepo (3001)..."
  exec "$BIN"
fi
