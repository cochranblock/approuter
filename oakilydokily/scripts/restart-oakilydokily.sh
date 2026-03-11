#!/bin/bash
# Restart oakilydokily only.
# Usage: ./scripts/restart-oakilydokily.sh [--bg]
# For waiver tests: OD_TEST_WAIVER_BYPASS=1 ./scripts/restart-oakilydokily.sh --bg
# Tries: oakilydokily dir, then cochranblock -p oakilydokily.
#
# Tokens: WORKSPACE_ROOT (parent of oakilydokily; auto-derived if unset)
#         COCHRANBLOCK_ROOT (overrides ${WORKSPACE_ROOT}/cochranblock)

set -e
OD_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORKSPACE_ROOT="${WORKSPACE_ROOT:-$(cd "$OD_ROOT/.." 2>/dev/null && pwd)}"
cd "$OD_ROOT"

[ -f "$OD_ROOT/.env" ] && set -a && source "$OD_ROOT/.env" && set +a

export PORT=3000
export BIND=0.0.0.0

pkill -f oakilydokily 2>/dev/null && echo "Stopped oakilydokily" || true
sleep 1

# Build: from oakilydokily if Cargo.toml exists, else from cochranblock
if [ -f "$OD_ROOT/Cargo.toml" ]; then
  cargo build --release -p oakilydokily --features approuter
  BIN="$OD_ROOT/target/release/oakilydokily"
else
  CB="${COCHRANBLOCK_ROOT:-${WORKSPACE_ROOT}/cochranblock}"
  if [ -f "$CB/Cargo.toml" ]; then
    (cd "$CB" && cargo build --release -p oakilydokily 2>/dev/null) || true
    BIN="$CB/target/release/oakilydokily"
  else
    echo "ERROR: oakilydokily not found. Need Cargo.toml in $OD_ROOT or cochranblock workspace."
    exit 1
  fi
fi

# Workspace may put target in parent dir
[ ! -f "$BIN" ] && [ -f "${WORKSPACE_ROOT}/target/release/oakilydokily" ] && BIN="${WORKSPACE_ROOT}/target/release/oakilydokily"
if [ ! -f "$BIN" ]; then
  echo "ERROR: oakilydokily binary not found at $BIN"
  exit 1
fi

if [ "$1" = "--bg" ]; then
  echo "Starting oakilydokily (3000) in background..."
  nohup "$BIN" </dev/null >/dev/null 2>&1 &
  echo "oakilydokily PID $!"
else
  echo "Starting oakilydokily (3000)..."
  exec "$BIN"
fi
