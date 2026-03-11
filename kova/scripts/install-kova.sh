#!/bin/bash
# Copyright (c) 2026 The Cochran Block. All rights reserved.
# Install Kova AI to ~/.local/bin. Run from intent-engine root.

set -e
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

echo "Building Kova..."
cd "$ROOT"
cargo build --release --bin kova -F serve

BIN="$ROOT/target/release/kova"
DEST="$HOME/.local/bin"
mkdir -p "$DEST"
cp "$BIN" "$DEST/kova"

if [[ ":$PATH:" != *":$DEST:"* ]]; then
    echo "Add to PATH: export PATH=\"\$HOME/.local/bin:\$PATH\""
    echo "Appending to ~/.bashrc..."
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
fi

echo "OK: $DEST/kova"
echo "Run: kova serve   # API at http://127.0.0.1:3002"
