# approuter-acme

Pure-Rust ACME TLS terminator. Sibling to **approuter** in the Cochran Block family.

One binary. Issues and renews Let's Encrypt certificates via DNS-01 challenge on Cloudflare, terminates TLS on your chosen bind address, reverse-proxies plain HTTP to your backend (typically `approuter` or the service behind it).

No Go. No shell scripts. No third-party ACME clients.

## What it does

1. **Loads or issues a cert** via Let's Encrypt DNS-01 on a Cloudflare-managed zone.
2. **Terminates TLS** on the configured bind address (default `0.0.0.0:8443`).
3. **Reverse-proxies** every decrypted request to a backend HTTP URL (default: approuter at `http://127.0.0.1:8080`).
4. **Renews automatically** before expiry (v0.2).

## Why standalone

Until approuter's native TLS integration ships (spec in-flight at the approuter pane), `approuter-acme` fills the role: a small binary next to approuter that owns the TLS termination concern. Single responsibility, swappable, Unlicensed.

## Usage

```bash
export CF_DNS_TOKEN="<token with Zone.DNS:Edit on cochranblock.org>"

approuter-acme \
  --domain direct.cochranblock.org \
  --backend http://127.0.0.1:8080 \
  --bind 0.0.0.0:8443 \
  --cert-dir ~/.approuter-acme
```

Or with env vars:

```bash
export APPROUTER_ACME_DOMAIN=direct.cochranblock.org
export APPROUTER_ACME_BACKEND=http://127.0.0.1:8080
export APPROUTER_ACME_CERT_DIR=~/.approuter-acme
export CF_DNS_TOKEN=xxx
approuter-acme
```

Dry-run (attempt cert issuance, log, exit — no serving):

```bash
approuter-acme --dry-run --domain direct.cochranblock.org
```

Staging environment (Let's Encrypt staging CA — won't chain publicly, for testing):

```bash
approuter-acme --staging ...
```

## Build

```bash
cargo build --profile=diamond           # speed-Diamond for servers
cargo build --profile=diamond-edge      # size-Diamond for constrained deploys
```

## Requirements

- The domain's DNS zone must be on Cloudflare.
- `CF_DNS_TOKEN` must have `Zone.DNS:Edit` permission on that zone.
- Outbound access to `acme-v02.api.letsencrypt.org` and `api.cloudflare.com`.
- Port 443 (or whatever you bind) must be reachable from the public internet after NAT.
- A DNS A record for the domain must point to your public IP (for clients to reach you).

## Verified pipeline

On 2026-04-14, a staging certificate was issued end-to-end in **31 seconds** total:
- Account creation: ~500ms
- Order creation: ~500ms
- CF TXT publish: ~400ms
- DNS propagation wait: 25s (configurable)
- LE validation: ~1s
- Cert finalize + download: ~3s

Production cert timing expected to match within a few seconds.

## What it does NOT do (yet)

- **v0.1 does not auto-parse cert expiry** for daily renewal decisions. Restart the process with a fresh cert-dir to force a reissue. v0.2 will parse the cert and reissue automatically when within `--renew-days` of expiry.
- Single domain per binary instance. Wildcards not yet. SAN certs not yet.
- HTTP-01 and TLS-ALPN-01 challenges not implemented — DNS-01 only.
- No health checks of the backend; bad gateway responses on backend failure.

## Benchmarks

TODO: add throughput / latency benchmarks under load to the description when measured. Expected: identical to approuter's direct-HTTP serving numbers (~3,300 req/sec on gd behind Verizon FiOS) plus the TLS handshake cost (~2-10ms per new connection, amortized across keep-alive).

## License

Unlicense. Public domain. Fork, strip, ship. See `UNLICENSE`.

## Protocols applied

- **P26 Moonshot Frame** — typed (CfDns, AcmeClient, Proxy, Args structs), bounded (challenge iteration, ALPN list), observable (tracing logs at info/warn/debug), explainable (docstrings on every module), reviewer-friendly (one concern per file: cf_dns.rs / acme.rs / proxy.rs / main.rs).
- **P27 Diamond Rust Binary Architecture** — both `[profile.diamond]` and `[profile.diamond-edge]` shipped in Cargo.toml.
- **Unlicense Baby** — fully public domain.
- **P12 AI Slop Eradication** — zero banned words in code, comments, or docs.

## Family relationship

- [**approuter**](https://github.com/cochranblock/approuter) — the reverse proxy this fronts
- [**cochranblock**](https://github.com/cochranblock/cochranblock) — the origin site behind approuter
- [**tmuxisfree**](https://github.com/cochranblock/tmuxisfree) — fleet orchestration used to deploy this

When approuter's native TLS integration ships (spec in-flight at the approuter pane), approuter-acme may be deprecated or kept as a lightweight alternative for simpler deployments.

---

Canonical source: <https://cochranblock.org/arch> · Protocol refs: <https://cochranblock.org/protocols>
