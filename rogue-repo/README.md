<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Rogue Repo

Sovereign, high-security software repository and ISO 8583 payment engine. 100% Rust.

## Workspace Crates

- **repo-vault**: AES-256-GCM encryption, PAN vaulting (Radioactive Data policy)
- **repo-switch**: ISO 8583 MTI 0200 engine, bitmask packing, bank TCP
- **repo-ledger**: PostgreSQL source of truth — users, devices, entitlements
- **repo-api**: Axum API — `/buy-bucks`, `/provision-app`, `/add-device`

## Rogue Bucks Economy

| Item | Amount |
|------|--------|
| 100 Rogue Bucks | $1.00 USD |
| Entry buy-in | $4.20 (420 bucks) |
| Game download | 42 bucks |
| Add device fee | 420 bucks |

## Build

```bash
cargo build
cargo run -p repo-api
```

## Test

```bash
cargo run -p repo-api -- --test
```

Runs f49 (unit), f50 (integration), f51 (HTTP). Exit 0 = pass, 1 = fail.

## Database

PostgreSQL. Run migrations:

```bash
sqlx migrate run
```

Set `DATABASE_URL` in `.env`.

## Tokenization

See `rogue-repo/compression_map.md` for identifier mapping.
