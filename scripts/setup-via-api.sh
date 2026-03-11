#!/bin/bash
# Automated setup via GitHub API + Railway GraphQL API.
# No browser or gh CLI required — just tokens.
#
# Prerequisites:
#   export GITHUB_TOKEN=ghp_xxx   # GitHub Personal Access Token (repo scope)
#   export RAILWAY_TOKEN=xxx      # Railway token (railway.app → Account → Tokens)
#
# Run from monorepo root (cochranblock-stack folder):
#   ./scripts/setup-via-api.sh

set -e
cd "$(cd "$(dirname "$0")/.." && pwd)"

GITHUB_API="https://api.github.com"
RAILWAY_GRAPHQL="https://backboard.railway.app/graphql/v2"
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

# --- Railway ---
if [[ -z "$RAILWAY_TOKEN" ]]; then
  echo ""
  echo "RAILWAY_TOKEN not set. Skip Railway project creation."
  echo "  Create token: railway.app → Account → Tokens"
else
  echo ""
  echo "Creating Railway project..."

  RESP=$(curl -s -X POST "$RAILWAY_GRAPHQL" \
    -H "Authorization: Bearer $RAILWAY_TOKEN" \
    -H "Content-Type: application/json" \
    -d '{
      "query": "mutation projectCreate($input: ProjectCreateInput!) { projectCreate(input: $input) { id name } }",
      "variables": {
        "input": {
          "name": "cochranblock-stack",
          "isPublic": false,
          "repo": {
            "fullRepoName": "'"$ORG/$REPO_STACK"'",
            "branch": "main"
          }
        }
      }
    }')

  if command -v jq &>/dev/null && echo "$RESP" | jq -e '.data.projectCreate' >/dev/null 2>&1; then
    PROJ_ID=$(echo "$RESP" | jq -r '.data.projectCreate.id')
    echo "  Project created: $PROJ_ID"
    echo "  Open: https://railway.app/project/$PROJ_ID"
  elif echo "$RESP" | grep -q '"projectCreate"'; then
    echo "  Project created. Open https://railway.app to view."
  else
    echo "  Railway response: $(echo "$RESP" | head -c 400)"
    echo ""
    echo "  If repo not found, push the monorepo first and retry."
  fi
fi

echo ""
echo "Done. Next: add services (approuter, cochranblock, oakilydokily, rogue-repo) and Postgres in Railway UI."
