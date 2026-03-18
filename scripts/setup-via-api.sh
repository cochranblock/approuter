#!/bin/bash
# Unlicense — cochranblock.org
# Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

# Automated setup via GitHub API.
# No browser or gh CLI required — just token.
#
# Prerequisites:
#   export GITHUB_TOKEN=ghp_xxx   # GitHub Personal Access Token (repo scope)
#
# Run from monorepo root (cochranblock-stack folder):
#   ./scripts/setup-via-api.sh

set -e
cd "$(cd "$(dirname "$0")/.." && pwd)"

GITHUB_API="https://api.github.com"
ORG="cochranblock"
REPO_STACK="cochranblock-stack"
REPO_ROGUE="rogue-repo"

echo "=== cochranblock-stack setup via API ==="

# --- GitHub ---
if [[ -z "$GITHUB_TOKEN" ]]; then
  echo "GITHUB_TOKEN not set. Skip GitHub repo creation."
  echo "  Create token: github.com → Settings → Developer settings → Personal access tokens"
  echo "  Required scope: repo"
else
  echo "Creating GitHub repos..."

  # cochranblock/cochranblock-stack (monorepo)
  echo "  Create $ORG/$REPO_STACK..."
  R=$(curl -s -w "\n%{http_code}" -X POST "$GITHUB_API/orgs/$ORG/repos" \
    -H "Authorization: Bearer $GITHUB_TOKEN" \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    -d '{"name":"'"$REPO_STACK"'","private":false}')
  CODE=$(echo "$R" | tail -n1)
  [[ "$CODE" == "201" ]] && echo "    Created." || echo "    (exists or error: $CODE)"

  # cochranblock/rogue-repo
  echo "  Create $ORG/$REPO_ROGUE..."
  R=$(curl -s -w "\n%{http_code}" -X POST "$GITHUB_API/orgs/$ORG/repos" \
    -H "Authorization: Bearer $GITHUB_TOKEN" \
    -H "Accept: application/vnd.github+json" \
    -H "X-GitHub-Api-Version: 2022-11-28" \
    -d '{"name":"'"$REPO_ROGUE"'","private":false}')
  CODE=$(echo "$R" | tail -n1)
  [[ "$CODE" == "201" ]] && echo "    Created." || echo "    (exists or error: $CODE)"

  echo "GitHub repos ready."
fi

# --- Push monorepo ---
echo ""
echo "Pushing monorepo..."
git remote set-url origin "git@github.com:$ORG/$REPO_STACK.git" 2>/dev/null || git remote add origin "git@github.com:$ORG/$REPO_STACK.git" 2>/dev/null || true
git push -u origin main 2>/dev/null || git push -u origin master 2>/dev/null || echo "Push failed — ensure repo exists and you have push access."

echo ""
echo "Done. Next: deploy approuter and backends (gd, Docker, or your preferred host)."