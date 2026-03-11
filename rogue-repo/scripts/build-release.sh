# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Build rogue-runner for Windows EXE + Android APK. Copies to rogue-repo/assets/downloads/.
# Run from WSL. Requires: Docker, rustup, mingw-w64 (sudo apt install mingw-w64)
set -e
REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$REPO_ROOT"

echo "=== Building Windows EXE ==="
rustup target add x86_64-pc-windows-gnu 2>/dev/null || true
cargo build -p rogue-runner --release --target x86_64-pc-windows-gnu

mkdir -p rogue-repo/assets/downloads
cp target/x86_64-pc-windows-gnu/release/rogue-runner.exe rogue-repo/assets/downloads/rogue-runner-windows-x64.exe
echo "Windows: rogue-repo/assets/downloads/rogue-runner-windows-x64.exe"

echo "=== Building Android APK ==="
# notfl3/cargo-apk uses older Cargo; Cargo.lock v4 may fail. If so, run manually from rogue-runner/.
PARENT="$(cd "$REPO_ROOT/../.." && pwd)"
docker run --rm -v "$PARENT:$PARENT" -w "$REPO_ROOT" notfl3/cargo-apk cargo quad-apk build -p rogue-runner --release || {
  echo "APK build failed. If Cargo.lock v4 error: run 'cd rogue-runner && docker run --rm -v \$(pwd):/root/src -w /root/src notfl3/cargo-apk cargo quad-apk build --release' (may need standalone rogue-runner)"
  exit 1
}

APK_DIR="$REPO_ROOT/target/android-artifacts/release/apk"
if [ -d "$APK_DIR" ]; then
  APK=$(find "$APK_DIR" -name "*.apk" -type f | head -1)
  if [ -n "$APK" ]; then
    cp "$APK" rogue-repo/assets/downloads/rogue-runner.apk
    echo "Android: rogue-repo/assets/downloads/rogue-runner.apk"
  else
    echo "No APK found in $APK_DIR"
    exit 1
  fi
else
  echo "APK dir not found: $APK_DIR"
  exit 1
fi

echo "=== Done. Binaries in rogue-repo/assets/downloads/ ==="
echo ""
echo "For MSI installer (Windows only): copy repo to Windows, run rogue-runner/scripts/build-msi.ps1"
