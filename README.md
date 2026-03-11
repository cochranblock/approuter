# approuter

Index and router for cochranblock products. **This repo contains approuter only.** Product source lives in separate repos.

## Products (separate repos)

| Product | Repo | Description |
|---------|------|-------------|
| **cochranblock** | [cochranblock/cochranblock](https://github.com/cochranblock/cochranblock) | Portfolio site (cochranblock.org) |
| **oakilydokily** | [cochranblock/oakilydokily](https://github.com/cochranblock/oakilydokily) | Hero site with mural |
| **rogue-repo** | [cochranblock/rogue-repo](https://github.com/cochranblock/rogue-repo) | Software repo + ISO 8583 |
| **kova** | [cochranblock/kova](https://github.com/cochranblock/kova) | Augment engine |
| **whyyoulying** | [cochranblock/whyyoulying](https://github.com/cochranblock/whyyoulying) | Labor fraud detection |
| **wowasticker** | [cochranblock/wowasticker](https://github.com/cochranblock/wowasticker) | Student goals app |

## This repo

- **approuter** — Reverse proxy + app registration for Cloudflare tunnel. Routes traffic to the products above.

## Build

```bash
cargo build -p approuter
```

## Railway

Deploy each product from its own repo. Approuter runs here; backends (cochranblock, oakilydokily, rogue-repo) connect via Railway private networking. See [approuter/docs/RAILWAY.md](approuter/docs/RAILWAY.md).

## Local development

Clone the product repos alongside this one. Run approuter; it will route to backends by URL (e.g. `ROUTER_PORTFOLIO_URL`, `ROUTER_KAYLIE_HOST`).
