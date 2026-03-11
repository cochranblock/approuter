# approuter

- Reverse proxy + app registration for Cloudflare tunnel. Routes to cochranblock products.
- Build: cargo build -p approuter
- Products live in separate repos; approuter points to them via env vars (ROUTER_COCHRANBLOCK_URL, etc.)
