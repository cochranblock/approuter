# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Build rogue-runner for iOS Simulator (macOS only)
# Requires: rustup target add x86_64-apple-ios
set -e
cd "$(dirname "$0")/.."
rustup target add x86_64-apple-ios 2>/dev/null || true
cargo build -p rogue-runner --target x86_64-apple-ios --release
mkdir -p RogueRunner.app
cp target/x86_64-apple-ios/release/rogue-runner RogueRunner.app/
cat > RogueRunner.app/Info.plist << 'EOF'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
<key>CFBundleExecutable</key><string>rogue-runner</string>
<key>CFBundleIdentifier</key><string>io.roguerepo.rogue-runner</string>
<key>CFBundleName</key><string>Rogue Runner</string>
<key>CFBundleVersion</key><string>1</string>
<key>CFBundleShortVersionString</key><string>0.1.0</string>
</dict>
</plist>
EOF
echo "Built RogueRunner.app. Install: xcrun simctl install booted RogueRunner.app/"
echo "Launch: xcrun simctl launch booted io.roguerepo.rogue-runner"
