# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Build rogue-runner for Windows (cross-compile from Linux)
# rustup target add x86_64-pc-windows-gnu
# May need: sudo apt install mingw-w64
set -e
cd "$(dirname "$0")/.."
rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
cargo build -p rogue-runner --target x86_64-pc-windows-gnu --release
echo "Built: target/x86_64-pc-windows-gnu/release/rogue-runner.exe"
