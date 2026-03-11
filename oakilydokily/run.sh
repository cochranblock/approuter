# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Run oakilydokily. Keeps running until Ctrl+C.
cd "$(dirname "$0")"
pkill -f "oakilydokily" 2>/dev/null && sleep 1
exec ./target/release/oakilydokily
