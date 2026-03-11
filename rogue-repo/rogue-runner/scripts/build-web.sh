# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Build rogue-runner for web (wasm32). Output: rogue-runner/web/
set -e
REPO_ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$REPO_ROOT"
cargo build -p rogue-runner --target wasm32-unknown-unknown --release
mkdir -p rogue-runner/web rogue-repo/rogue-repo/assets/apps/rogue-runner-wasm
cp target/wasm32-unknown-unknown/release/rogue-runner.wasm rogue-runner/web/
cp rogue-runner/web/rogue-runner.wasm rogue-repo/rogue-repo/assets/apps/rogue-runner-wasm/
cp rogue-runner/web/index.html rogue-repo/rogue-repo/assets/apps/rogue-runner-wasm/
echo "Built rogue-runner/web/. Wasm copied to rogue-repo assets. Route: /apps/rogue-runner-wasm"
