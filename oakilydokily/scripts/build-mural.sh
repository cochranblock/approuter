#!/bin/bash
# Build mural-wasm and copy to assets for embedding.
set -e
OD_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
WORKSPACE_ROOT="${WORKSPACE_ROOT:-$(cd "$OD_ROOT/.." 2>/dev/null && pwd)}"
cd "$OD_ROOT"
cargo build --target wasm32-unknown-unknown -p mural-wasm --release
# Target may be in workspace root or project target/
WASM="$OD_ROOT/target/wasm32-unknown-unknown/release/mural-wasm.wasm"
[ -f "$WASM" ] || WASM="$WORKSPACE_ROOT/target/wasm32-unknown-unknown/release/mural-wasm.wasm"
cp "$WASM" "$OD_ROOT/assets/"
