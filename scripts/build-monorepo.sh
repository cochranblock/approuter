#!/bin/bash
# cochranblock-stack is thin (approuter only). Products live in separate repos.
# This script is for local dev when you have a full workspace elsewhere.
# It syncs products into cochranblock-stack for Railway-style deployment.
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
STACK_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WS_ROOT="${1:-$(cd "$STACK_ROOT/../.." && pwd)}"

if [ ! -d "$WS_ROOT/approuter" ]; then
  WS_ROOT="$(cd "$STACK_ROOT/../.." && pwd)"
fi
if [ ! -d "$WS_ROOT/approuter" ]; then
  echo "ERROR: Cannot find workspace root (need approuter, cochranblock, etc.)"
  echo "Usage: $0 /path/to/workspace/root"
  echo ""
  echo "Note: cochranblock-stack repo contains approuter only. Products are in separate repos."
  exit 1
fi

echo "Workspace: $WS_ROOT"
echo "Stack: $STACK_ROOT"
echo "Syncing products (for local Railway-style deploy; not committed to cochranblock-stack repo)..."

SYNC_DIRS="approuter cochranblock oakilydokily rogue-repo kova kova-core kova-web vendor whyyoulying wowasticker Cargo.toml Cargo.lock"
for d in $SYNC_DIRS; do
  [ -e "$WS_ROOT/$d" ] || continue
  if [ -d "$WS_ROOT/$d" ]; then
    rsync -a --delete \
      --exclude='.git' --exclude='target' --exclude='node_modules' \
      "$WS_ROOT/$d" "$STACK_ROOT/"
  else
    cp "$WS_ROOT/$d" "$STACK_ROOT/"
  fi
done

echo "Done. Products synced to $STACK_ROOT (gitignored; not in cochranblock-stack repo)"
