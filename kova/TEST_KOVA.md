# Kova Test — Exopack + UI Simulations

Run the combined kova validation suite:

```bash
cargo run -p kova --bin kova-test --features tests
```

## What it does (f90)

1. **Clippy** — `cargo clippy -D warnings`
2. **TRIPLE SIMS** — `cargo test` 3× via exopack::triple_sims::f61
3. **Release build** — `cargo build --release --features serve` (kova + kova-c2)
4. **Bootstrap smoke** — `kova bootstrap` in temp HOME
5. **kova-c2 smoke** — `kova-c2 nodes` (tokenized orchestration binary)
6. **Serve smoke** — spawn `kova serve`, GET / via exopack::interface::http_client

## Prerequisites

- exopack with `triple_sims` and `interface` features (kova tests feature)
- capnp compiler (for kova-node if building full workspace)
