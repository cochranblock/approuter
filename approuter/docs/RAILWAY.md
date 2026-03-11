# Railway Deployment

Deploy the cochranblock monorepo (approuter, cochranblock, oakilydokily, rogue-repo) to Railway.

## Prerequisites

- Railway account (connect via GitHub OAuth — no token needed)
- GitHub repo cochranblock/cochranblock-stack with the full workspace. The Dockerfiles require build context = workspace root (parent of approuter, cochranblock, oakilydokily, rogue-repo, kova, vendor, etc.)

## Setup

### 1. Create Project

1. New Project → Empty project
2. Add 4 services: approuter, cochranblock, oakilydokily, rogue-repo
3. Add Postgres plugin (for rogue-repo); link to rogue-repo service

### 2. Configure Each Service

| Service      | Root Directory | Dockerfile      | Port |
|-------------|----------------|-----------------|------|
| approuter   | approuter      | approuter/Dockerfile | 8080 |
| cochranblock| cochranblock   | cochranblock/Dockerfile | 8081 |
| oakilydokily| oakilydokily   | oakilydokily/Dockerfile | 3000 |
| rogue-repo  | rogue-repo     | rogue-repo/Dockerfile | 3001 |

Set `RAILWAY_DOCKERFILE_PATH` if Railway does not auto-detect (e.g. `approuter/Dockerfile` for approuter when build context is repo root).

### 3. Environment Variables

**approuter:** ROUTER_CONFIG_DIR=/app (baked in), ROUTER_BIND=0.0.0.0

**cochranblock:** APPROUTER_URL=http://approuter.railway.internal:8080, CB_BACKEND_URL=http://cochranblock.railway.internal:8081

**oakilydokily:** APPROUTER_URL=http://approuter.railway.internal:8080

**rogue-repo:** DATABASE_URL (from Postgres plugin), APPROUTER_URL=http://approuter.railway.internal:8080, REPO_BACKEND_URL=http://rogue-repo.railway.internal:3001

### 4. Domains

Attach custom domains to the **approuter** service only (it proxies to backends via private networking):

- cochranblock.org, www.cochranblock.org
- roguerepo.io, www.roguerepo.io
- kaylie.cochranblock.org, oakilydokily.com, www.oakilydokily.com

In Cloudflare DNS: CNAME each to the Railway-provided host (e.g. `xxx.railway.app`).

### 5. Build Context

For monorepo, ensure build context is the **repository root** so all workspace members are available. Use Root Directory for service source, but Docker build context should remain repo root (Railway default when using RAILWAY_DOCKERFILE_PATH).

## No Cloudflare Tunnel

Railway exposes HTTPS directly. No CF_TOKEN or cloudflared needed. The registry (`config/registry.railway.json`) pre-populates backend URLs for Railway private networking.
