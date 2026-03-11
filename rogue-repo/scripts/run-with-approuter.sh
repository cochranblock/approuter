# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Start approuter, then rogue-repo. Rogue-repo registers with approuter on startup.
# Requires: approuter at ../../approuter, DATABASE_URL optional for auth.
set -e
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

APPROUTER_ROOT="${APPROUTER_ROOT:-$(cd "$REPO_ROOT/../approuter" 2>/dev/null && pwd)}"
if [ -z "$APPROUTER_ROOT" ] || [ ! -f "$APPROUTER_ROOT/Cargo.toml" ]; then
  echo "APPROUTER_ROOT not set or approuter not found. Set APPROUTER_ROOT or ensure ../approuter exists."
  exit 1
fi

cleanup() {
  if [ -n "$APPR_PID" ] && kill -0 "$APPR_PID" 2>/dev/null; then
    kill "$APPR_PID" 2>/dev/null || true
  fi
}
trap cleanup EXIT

echo "Starting approuter on :8080 (with tunnel for internet)..."
cd "$APPROUTER_ROOT"
cargo run -p approuter -- &
APPR_PID=$!
cd "$REPO_ROOT"

echo "Waiting for approuter..."
for i in $(seq 1 30); do
  if curl -s -o /dev/null -w "%{http_code}" http://127.0.0.1:8080/approuter/openapi.json 2>/dev/null | grep -q 200; then
    echo "Approuter ready."
    break
  fi
  sleep 1
  if [ $i -eq 30 ]; then
    echo "Approuter did not become ready."
    exit 1
  fi
done

echo "Starting rogue-repo on :3001..."
export APPROUTER_URL="${APPROUTER_URL:-http://127.0.0.1:8080}"
export REPO_HOSTNAMES="${REPO_HOSTNAMES:-roguerepo.io,www.roguerepo.io}"
export REPO_BACKEND_URL="${REPO_BACKEND_URL:-http://127.0.0.1:3001}"
# rogue-repo loads .env via dotenvy when it runs
cargo run -p rogue-repo
