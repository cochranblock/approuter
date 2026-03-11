#!/bin/bash
# Build the cochranblock-stack monorepo from the workspace.
# Run from workspace root. Output: cochranblock-stack/ with full workspace.
set -e
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
STACK_ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
WS_ROOT="${1:-$(cd "$STACK_ROOT/../.." && pwd)}"

# If run from workspace root, WS_ROOT is parent of cochranblock-stack
if [ ! -d "$WS_ROOT/approuter" ]; then
  WS_ROOT="$(cd "$STACK_ROOT/../.." && pwd)"
fi
if [ ! -d "$WS_ROOT/approuter" ]; then
  echo "ERROR: Cannot find workspace root (need approuter, cochranblock, etc.)"
  echo "Usage: $0 /path/to/workspace/root"
  exit 1
fi

echo "Workspace: $WS_ROOT"
echo "Stack: $STACK_ROOT"

# Sync workspace into stack root (so stack root = workspace root for Railway)
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

echo "Monorepo built. Root = $STACK_ROOT (workspace root for Railway)"
echo "Next: cd $STACK_ROOT && git add -A && git status"
