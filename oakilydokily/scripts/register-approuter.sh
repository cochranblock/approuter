# Copyright (c) 2026 The Cochran Block. All rights reserved.
#!/bin/bash
# Register oakilydokily with approuter for Cloudflare tunnel routing.
# Uses approuter's openapi.json to discover base URL. Run when approuter is up.
# Requires: approuter running, CF_TOKEN in approuter env.

set -e
ROUTER="${ROUTER:-http://127.0.0.1:8080}"
MAX_ATTEMPTS="${REGISTER_RETRIES:-10}"

# Resolve base URL from openapi.json (source of truth for connection)
get_base_url() {
  local spec
  spec=$(curl -s --connect-timeout 2 "$ROUTER/approuter/openapi.json" 2>/dev/null || true)
  if [ -n "$spec" ]; then
    echo "$spec" | python3 -c "
import json, sys
try:
    d = json.load(sys.stdin)
    url = d.get('servers', [{}])[0].get('url', '').rstrip('/')
    if url: print(url)
except: pass
" 2>/dev/null || true
  fi
}

BASE=""
for i in $(seq 1 "$MAX_ATTEMPTS"); do
  BASE=$(get_base_url)
  [ -n "$BASE" ] && break
  [ $i -lt "$MAX_ATTEMPTS" ] && sleep 2
done
BASE="${BASE:-$ROUTER}"

curl -s -X POST "$BASE/approuter/register" \
  -H "Content-Type: application/json" \
  -d '{
    "app_id": "oakilydokily",
    "hostnames": ["oakilydokily.com", "www.oakilydokily.com", "kaylie.cochranblock.org"],
    "backend_url": "http://127.0.0.1:3000"
  }'

echo ""
echo "Registered via $BASE. Verify: curl -s $BASE/approuter/apps"
echo "To provision approuter systemd: scripts/provision-approuter-systemd.sh"
