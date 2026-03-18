<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# Next Steps for Deployment

## Done

- Monorepo built at `/Users/mcochran/cochranblock-stack` with full workspace
- Initial commit created
- Remote set to `git@github.com:cochranblock/cochranblock-stack.git`

## You Need To Do

### 1. Create GitHub repos

Create these on GitHub (github.com/new or org cochranblock):

- **cochranblock/cochranblock-stack** (public) — monorepo: approuter, cochranblock, oakilydokily, rogue-repo, kova
- **cochranblock/rogue-repo** (public, for standalone rogue-repo pushes)

### 2. Push the monorepo

```bash
cd /Users/mcochran/cochranblock-stack
git push -u origin main
```

### 3. Deploy

Deploy approuter and backends to gd (Debian), Docker, or your preferred host. See [approuter/docs/ROUTER.md](approuter/docs/ROUTER.md) and [approuter/docs/TUNNEL_SYSTEMD.md](approuter/docs/TUNNEL_SYSTEMD.md) for gd + Cloudflare tunnel setup.

### 4. Rebuild monorepo after workspace changes

```bash
cd /Users/mcochran
./cochranblock-stack/scripts/build-monorepo.sh /Users/mcochran
cd cochranblock-stack
git add -A && git commit -m "Sync from workspace" && git push
```

## Optional: Automated setup via API

No browser or `gh` CLI needed. Use token:

```bash
export GITHUB_TOKEN=ghp_xxx   # github.com → Settings → Developer settings → PAT (repo scope)
./scripts/setup-via-api.sh
```

This script will:
1. Create `cochranblock/cochranblock` and `cochranblock/rogue-repo` via GitHub API
2. Push the monorepo

`jq` optional (nicer output).

## Optional: GitHub CLI

If you install `gh` (`brew install gh`) and run `gh auth login`, you can create repos from the CLI:

```bash
gh repo create cochranblock/cochranblock-stack --public --source=/Users/mcochran/cochranblock-stack --push
gh repo create cochranblock/rogue-repo --public
```