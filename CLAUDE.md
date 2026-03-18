<!-- Unlicense — cochranblock.org -->
<!-- Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3 -->

# approuter

- Reverse proxy + app registration for Cloudflare tunnel. Routes to cochranblock products.
- Build: cargo build -p approuter
- Products live in separate repos; approuter points to them via env vars (ROUTER_COCHRANBLOCK_URL, etc.)