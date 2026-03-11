# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Build rogue-runner APK for Android (requires Docker + cargo-quad-apk)
# docker pull notfl3/cargo-apk
# cargo install cargo-quad-apk
set -e
cd "$(dirname "$0")/.."
docker run --rm -v "$(pwd):/root/src" -w /root/src notfl3/cargo-apk cargo quad-apk build --release
echo "APK: target/android-artifacts/release/apk/"
