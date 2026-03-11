<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Binary Structure — Two Binaries Per Project

**Rule:** Every project has exactly two binaries. AI iterates on the test binary; the release binary is a stripped product-only build.

## Structure

| Binary | Purpose | Contents |
|--------|---------|----------|
| `<project>` | Release | Product only. No tests, no self-eval (screenshot, etc.). |
| `<project>-test` | AI iteration | Full. Tests + self-eval. AI uses this to iterate, then strips for release. |

## Build

- **Release:** `cargo build --release -p <project>` → product-only binary
- **Test:** `cargo build --release -p <project> --bin <project>-test --features tests` → full binary

## Feature Flag

Projects use a `tests` feature to gate test/self-eval code:

```toml
[features]
default = []
tests = ["colored", ...]  # deps only used by tests/self-eval

[[bin]]
name = "<project>"
path = "src/main.rs"

[[bin]]
name = "<project>-test"
path = "src/bin/<project>-test.rs"
required-features = ["tests"]
```

## Projects

| Project | Release | Test |
|---------|---------|------|
| cochranblock | cochranblock | cochranblock-test |
| oakilydokily | oakilydokily | oakilydokily-test |
| whyyoulying | whyyoulying | whyyoulying-test |
| rogue-repo | (per crate) | (per crate) |
| ronin-sites | ronin-sites | ronin-sites-test |
| approuter | approuter | approuter-test |
