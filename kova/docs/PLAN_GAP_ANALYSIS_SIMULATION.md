# Plan: Gap Analysis Simulation — Kova Subcommands (Sequential)

**Purpose:** Iterate through each kova subcommand in order, run a gap analysis simulation, and document Expected vs Observed vs Gaps. No parallel tasking — strictly sequential.

**Reference:** USER_STORY_PERFECT_RUST.md, TRIPLE_SIMS_KOVA.md, PLAN_BUILD_KOVA.md, compression_map.md

**Date:** 2026-03-09

---

## Subcommand Order (Sequential)

| # | Subcommand | Invocation | Primary intent |
|---|------------|------------|----------------|
| 1 | (default) | `kova` | Opens GUI when gui feature enabled |
| 2 | gui | `kova gui` | Native egui. Code gen, backlog, prompts. |
| 3 | serve | `kova serve` | HTTP API + web client at / |
| 4 | node | `kova node` | Worker daemon (Phase 1 stub) |
| 5 | c2 | `kova c2 run f20`, `kova c2 nodes` | Tokenized orchestration |
| 6 | bootstrap | `kova bootstrap` | Create ~/.kova, prompts, config |
| 7 | prompts | `kova prompts` | Print Cursor prompts (baked + external) |
| 8 | model | `kova model install`, `kova model list` | Model management |
| 9 | recent | `kova recent [--project DIR] [--minutes N]` | Recent changes (f86/f87) |
| 10 | autopilot | `kova autopilot "prompt"` | Type into Cursor composer |

---

## Simulation Protocol (Per Subcommand)

For each subcommand, in order:

1. **Precondition** — Document required state (e.g. bootstrap done, models installed).
2. **Execute** — Run the command. Capture stdout, stderr, exit code.
3. **Expected** — From USER_STORY, TRIPLE_SIMS, or PLAN_BUILD_KOVA.
4. **Observed** — What actually happened.
5. **Gap** — Expected vs Observed. Severity: High / Medium / Low / None.
6. **Recommendation** — Fix or defer.

---

## Sim 1: (default) — `kova`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | Run `kova` (no args) | Opens GUI if gui feature | ✓ Opens egui window | None |
| 2 | Without gui feature | Prints usage, suggests gui/serve | ✓ Prints usage | None |
| 3 | Bootstrap | Auto-runs if ~/.kova missing | ✓ run_gui calls bootstrap() | None |

**Precondition:** None.  
**Execute:** `kova`  
**Recommendation:** None.

---

## Sim 2: gui — `kova gui`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | Window opens | egui window, chat input | ✓ | None |
| 2 | Prompts panel | system.md, persona.md visible | ✓ | None |
| 3 | Backlog panel | Load from ~/.kova/backlog.json | ✓ | None |
| 4 | Project selector | discover_projects, dropdown | ✓ | None |
| 5 | Code gen (inference) | cursor_prompts in Coder context | ✓ build_system_prompt_impl injects | None |
| 6 | Copy/Apply/diff | After rust block in output | ✓ | None |

**Precondition:** Bootstrap done. Models for inference.  
**Execute:** `kova gui`  
**Recommendation:** None. GUI already injects cursor_prompts via build_system_prompt_impl.

---

## Sim 3: serve — `kova serve`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | Server starts | Binds to KOVA_BIND or default | ✓ | None |
| 2 | GET / | Returns app.html (web client) | ✓ | None |
| 3 | GET /api/status | `{"status":"ok"}` | ✓ | None |
| 4 | GET /api/projects | Discovered projects list | ✓ | None |
| 5 | GET /api/prompts | system, persona | ✓ | None |
| 6 | POST /api/intent | FullPipeline → pipeline, WebSocket stream | ✓ | None |
| 7 | Copy/Apply/diff/backlog | Web client has parity with GUI | ✓ | None |
| 8 | serve --open | Opens browser after bind | ✓ | None |

**Precondition:** Bootstrap done.  
**Execute:** `kova serve`, `kova serve --open`  
**Recommendation:** None.

---

## Sim 4: node — `kova node`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | With daemon feature | Prints "schema loaded, daemon stub" | ✓ (requires capnp) | None |
| 2 | Without daemon feature | Bails: "Build with --features daemon" | ✓ | None |
| 3 | Cap'n Proto | Schema compiles; no network listener yet | Phase 1 | None |

**Precondition:** `--features daemon` + capnp installed.  
**Execute:** `kova node`  
**Recommendation:** None.

---

## Sim 5: c2 — `kova c2 run`, `kova c2 nodes`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | `kova c2 nodes` | Prints lf, gd, bt, st | ✓ | None |
| 2 | `kova c2 run f20` | Full pipeline locally | ✓ | None |
| 3 | `kova c2 run f20 --broadcast` | SSH to workers, run on /mnt/hive | ✓ | None |
| 4 | `kova c2 run f21` | Tunnel update (local only) | ✓ | None |
| 5 | Path mapping | ~/hive-vault → /mnt/hive on workers | ✓ to_worker_path | None |

**Precondition:** Bootstrap. For broadcast: SSH to lf/gd/bt/st.  
**Execute:** `kova c2 nodes`, `kova c2 run f20`, etc.  
**Recommendation:** None.

---

## Sim 6: bootstrap — `kova bootstrap`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | Creates ~/.kova/ | Directory exists | ✓ | None |
| 2 | Creates prompts/ | system.md, persona.md | ✓ | None |
| 3 | Creates config.toml | Paths, models, cursor | ✓ | None |
| 4 | Creates backlog.json | `{"items":[]}` | ✓ | None |
| 5 | Idempotent | Re-run does not corrupt | ✓ | None |

**Precondition:** None.  
**Execute:** `kova bootstrap`  
**Recommendation:** None.

---

## Sim 7: prompts — `kova prompts`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | With prompts_enabled | Prints baked + external rules | ✓ Fixed: CursorSection defaults true | None |
| 2 | Without prompts_enabled | "Prompts disabled" | ✓ | None |
| 3 | Content | blocking, augment, tokenization, compression_map | ✓ | None |
| 4 | Workspace rules | .cursor/rules/*.mdc appended | ✓ | None |

**Precondition:** Bootstrap done. Optional: project with .cursor/rules.  
**Execute:** `kova prompts`  
**Recommendation:** None. Fixed: config without [cursor] now defaults prompts_enabled=true.

---

## Sim 8: model — `kova model install`, `kova model list`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | model list | router, coder, fix paths; orchestration config | ✓ | None |
| 2 | model install | Downloads from Hugging Face (bartowski) | ✓ | None |
| 3 | Model exists | Skips download | ✓ | None |
| 4 | Without inference | Bails | ✓ | None |

**Precondition:** Bootstrap. For install: network.  
**Execute:** `kova model list`, `kova model install`  
**Recommendation:** None.

---

## Sim 9: recent — `kova recent`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | Default (cwd, 30 min) | f86 + f87 output | ✓ | None |
| 2 | --project DIR | Scoped to project | ✓ | None |
| 3 | --minutes N | Custom window | ✓ | None |
| 4 | No changes | "No files modified" | ✓ | None |

**Precondition:** Project with recent edits (or empty).  
**Execute:** `kova recent`, `kova recent --minutes 60`  
**Recommendation:** None. Use --project for large trees (cwd=HOME can be slow).

---

## Sim 10: autopilot — `kova autopilot "prompt"`

| Step | Action | Expected | Observed | Gap |
|------|--------|----------|----------|-----|
| 1 | With autopilot feature | Types prompt into Cursor composer | ✓ | None |
| 2 | Without autopilot feature | Bails | ✓ | None |
| 3 | Cursor focused | enigo sends keystrokes | ✓ | None |

**Precondition:** Cursor focused. `--features autopilot`.  
**Execute:** `kova autopilot "add a test"`  
**Recommendation:** None.

---

## Execution Instructions

**Sequential order (no parallel):**

1. Run Sim 1. Fill Expected/Observed/Gap. Document Recommendation.
2. Run Sim 2. Same.
3. … through Sim 10.

**After all sims:**

- Aggregate gaps by severity.
- Prioritize fixes.
- Update TRIPLE_SIMS_KOVA.md if findings overlap.
- Re-run kova-test to ensure no regression.

**Prompt for AI executor:**

> "Execute PLAN_GAP_ANALYSIS_SIMULATION.md sequentially. For each Sim 1–10: run the command, fill Expected/Observed/Gap/Recommendation. Use release binary. No parallel tasking."

---

## Execution Summary (2026-03-09)

**Fixes implemented:**

1. **CursorSection default** — Config without `[cursor]` section previously caused `prompts_enabled` to default to `false` (serde Default for bool). Added `impl Default for CursorSection` with `prompts_enabled: default_prompts_enabled()` (= true). `kova prompts` now prints baked + external rules when config is minimal.

2. **Prompts disabled message** — Clarified: "Prompts disabled (config [cursor] prompts_enabled = false or no rules found)".

**Gaps closed:** 1 (prompts default). GUI cursor_prompts was already implemented via `build_system_prompt_impl`.

**kova-test:** Passed (clippy, TRIPLE SIMS, release build, bootstrap smoke, serve smoke).
