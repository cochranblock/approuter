# Next Steps for Railway Deployment

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

### 3. Set up Railway

1. Go to [railway.com](https://railway.com) and sign in with GitHub
2. New Project → Deploy from GitHub → select **cochranblock/cochranblock-stack**
3. Add 4 services (or use "Add Service" for each):
   - **approuter** — Root Directory: `approuter`
   - **cochranblock** — Root Directory: `cochranblock`
   - **oakilydokily** — Root Directory: `oakilydokily`
   - **rogue-repo** — Root Directory: `rogue-repo`
4. Add Postgres plugin, link to rogue-repo service
5. Set env vars per [approuter/docs/RAILWAY.md](approuter/docs/RAILWAY.md)
6. Attach custom domains to approuter: cochranblock.org, roguerepo.io, kaylie.cochranblock.org, oakilydokily.com

### 4. Rebuild monorepo after workspace changes

```bash
cd /Users/mcochran
./cochranblock-stack/scripts/build-monorepo.sh /Users/mcochran
cd cochranblock-stack
git add -A && git commit -m "Sync from workspace" && git push
```

## Optional: Automated setup via API

No browser or `gh` CLI needed. Use tokens:

```bash
export GITHUB_TOKEN=ghp_xxx   # github.com → Settings → Developer settings → PAT (repo scope)
export RAILWAY_TOKEN=xxx      # railway.app → Account → Tokens
./scripts/setup-via-api.sh
```

This script will:
1. Create `cochranblock/cochranblock` and `cochranblock/rogue-repo` via GitHub API
2. Push the monorepo
3. Create a Railway project linked to the repo via Railway GraphQL API

`jq` optional (nicer output).

## Optional: GitHub CLI

If you install `gh` (`brew install gh`) and run `gh auth login`, you can create repos from the CLI:

```bash
gh repo create cochranblock/cochranblock-stack --public --source=/Users/mcochran/cochranblock-stack --push
gh repo create cochranblock/rogue-repo --public
```
