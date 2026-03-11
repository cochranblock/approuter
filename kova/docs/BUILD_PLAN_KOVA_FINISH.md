# Build Plan: Finish Kova Project

## Current State

| Component | Location | Purpose |
|-----------|----------|---------|
| kova | workspace member | Main binary: gui, serve, c2, bootstrap, prompts, model, recent, autopilot, node |
| kova-test | kova/src/bin/kova-test.rs | Separate binary: clippy, TRIPLE SIMS (f61), release build, smoke, baked demo |
| exopack | workspace member (sibling) | Testing augmentation: triple_sims, baked_demo, interface, screenshot, video, mock, demo |
| kova-core | workspace member | Shared types (Intent, Backlog). Used by kova + kova-web |
| kova-web | workspace member | WASM thin client. egui + kova-core |

## Goals

1. **exopack as subcrate** — Move under kova (kova/exopack)
2. **kova binary deploys exopack** — Add `kova test` subcommand that runs full quality gate (clippy, TRIPLE SIMS, release build, smoke, baked demo)
3. **Singular kova binary** — Fold kova-test into main binary as `kova test`

## Implementation Steps

### Step 1: Move exopack under kova

- `mv exopack kova/exopack`
- Remove `exopack` from workspace members in root Cargo.toml
- Update kova Cargo.toml: `exopack = { path = "exopack", optional = true, ... }`

### Step 2: Add `kova test` subcommand

- Add `Test` to `Cmd` enum in main.rs
- Implement `run_test()` that runs: clippy → f61 (TRIPLE SIMS) → release build → bootstrap smoke → c2 nodes smoke → baked demo
- Gate on `#[cfg(feature = "tests")]`
- Remove or repurpose kova-test binary: either delete and use `kova test`, or make kova-test a thin wrapper that invokes `kova test`

### Step 3: exopack deploy functions in kova

Functions kova needs from exopack (already used by kova-test):

| Function | Module | Use |
|----------|--------|-----|
| f61 | triple_sims | Run cargo test N times (TRIPLE SIMS) |
| run_baked_demo | baked_demo | Full intended-usage demo, zero user input |
| http_client | interface | HTTP client for baked demo |

All present. Path change only (exopack as kova/exopack).

### Step 4: Workspace cleanup

- Root Cargo.toml: remove `exopack` from members
- No other workspace crates depend on exopack (verified)

### Step 5: kova-core / kova-web

- **Keep as-is.** kova-web needs kova-core for WASM build. Ingesting would complicate WASM target.
- kova-core is small (intent, backlog). No consolidation needed.

## Out of Scope (for now)

- Ingesting kova-web into kova (different target: wasm32)
- Ingesting kova-core (breaks kova-web; keep shared)
- exopack standalone binary (exopack live-demo) — can remain as `kova exopack live-demo` or stay in exopack bin for other projects

## Verification

After implementation:

```bash
cargo build -p kova --features tests
cargo run -p kova --bin kova --features tests -- test
# Or: cargo run -p kova --bin kova-test --features tests
```

Expected: clippy → TRIPLE SIMS → release build → smoke → baked demo. All pass.

## Completed (2026-03-10)

- exopack moved to kova/exopack
- Workspace members updated: cochranblock, oakilydokily, whyyoulying, wowasticker, approuter now use path = "../kova/exopack"
- Root exopack removed
- `kova test` subcommand added (gate on --features tests)
- kova-test binary now thin wrapper calling kova::run_test_suite()
- run_test_suite() in lib.rs for shared logic
