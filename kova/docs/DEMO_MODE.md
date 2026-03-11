# Demo — Baked-In Full Usage for Zero-Input Iteration

**Purpose:** A **baked-in demo** built into the code replicates every intended use of Kova. No user input. Runs autonomously so you can iterate through dev cycles with zero human interaction.

**Date:** 2026-03-10

---

## Overview

- **Baked demo** — `exopack::baked_demo::run_baked_demo()` runs every CLI subcommand + every HTTP endpoint. Code, not recording.
- **kova-test** — Runs the baked demo. Full intended usage, no input from you.
- **Recording mode** (optional) — `?demo=1` / `--demo` for action recording; secondary to the baked demo.

---

## Web (serve)

**Enable:** `?demo=1` in URL, or `kova serve --open --demo`

**Recorded:**
- Clicks (selector)
- Input changes (prompt textarea)
- API calls (method, path, body summary)

**Save:** Click "Save demo" → POST to `/api/demo/record` → writes `~/.kova/demos/{name}.json`

---

## Egui (native GUI)

**Enable:** `kova gui --demo`

**Recorded:**
- `egui_send` — chat message sent
- `egui_confirm` — intent confirmed (y/n)
- `api_call` — run_intent (full-pipeline, etc.)

**Save:** On exit (window close) → writes `~/.kova/demos/egui-{timestamp}.json`

---

## exopack

**Features:**
- `video` — xcap screen capture. `capture_screenshot(out_dir, name)` saves primary monitor PNG.
- `demo` — `DemoRecord`, `DemoAction` types; `demo_dir()` for `~/.kova/demos` or `KOVA_DEMO_DIR`.

**Usage:**
```rust
#[cfg(feature = "video")]
exopack::video::capture_screenshot(Path::new("/tmp"), "before")?;
```

---

## kova-test: Baked Demo (Zero Input)

- **Full intended usage** — Runs: bootstrap, prompts, model list, recent, c2 nodes, then serve + every HTTP endpoint.
- **No user input** — The demo is code. Replicates you using Kova in every intended way.
- **Iterate autonomously** — Run `cargo run -p kova --bin kova-test --features tests`; no interaction needed.
- **Artifact** — POST /api/demo/record writes `baked-demo.json`; verified on success.

---

## Record Format

```json
{
  "name": "web-1234567890",
  "source": "web",
  "actions": [
    {"kind": "web_click", "selector": "#send", "ts_ms": 123},
    {"kind": "api_call", "method": "POST", "path": "/api/intent", "body_summary": "..."}
  ],
  "started_at": "1234567890"
}
```

---

## Environment

| Var | Purpose |
|-----|---------|
| `KOVA_DEMO_DIR` | Override demo output dir (default: `~/.kova/demos`) |
| `TEST_DEMO` | Set by exopack live-demo when running -test binaries. Enables demo for self-evaluation. |

---

## Project-Wide Convention

- **Test binaries** (`*-test`) should use demo mode when exercising UI or serve.
- **exopack live-demo** sets `TEST_DEMO=1` when running any -test binary.
- **Self-evaluation** — Demo artifacts from test runs support iterative dev cycle evaluation.
