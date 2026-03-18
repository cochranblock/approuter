#!/bin/bash
# Unlicense — cochranblock.org
# Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

# Run cloudflared with token from Cloudflare API. Requires CF_TOKEN, CF_ACCOUNT_ID, CF_TUNNEL_ID in env.
set -e
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT"
[ -f approuter/.env ] && set -a && source approuter/.env && set +a

CF_TUNNEL_ID="${CF_TUNNEL_ID:-b12525df-6971-4c47-9a0d-61ee57a5cbd5}"

if [ -n "$TUNNEL_TOKEN" ]; then
  exec cloudflared tunnel run --token "$TUNNEL_TOKEN"
fi

if [ -z "$CF_TOKEN" ] || [ -z "$CF_ACCOUNT_ID" ]; then
  echo "Set CF_TOKEN+CF_ACCOUNT_ID or TUNNEL_TOKEN in approuter/.env" >&2
  exit 1
fi

RESP=$(
  if command -v wget &>/dev/null; then
    wget -qO- --header="Authorization: Bearer $CF_TOKEN" \
      "https://api.cloudflare.com/client/v4/accounts/$CF_ACCOUNT_ID/cfd_tunnel/$CF_TUNNEL_ID/token"
  else
    curl -s -H "Authorization: Bearer $CF_TOKEN" \
      "https://api.cloudflare.com/client/v4/accounts/$CF_ACCOUNT_ID/cfd_tunnel/$CF_TUNNEL_ID/token"
  fi
)
TOKEN=$(echo "$RESP" | grep -o '"result":"[^"]*"' | cut -d'"' -f4)

if [ -z "$TOKEN" ]; then
  echo "Failed to fetch tunnel token from Cloudflare API" >&2
  exit 1
fi

exec cloudflared tunnel run --token "$TOKEN"