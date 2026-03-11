# Triple Sims: Kova vs Original Intent (AI-Assisted Evaluation)

**Target:** Evaluate Kova against USER_STORY_PERFECT_RUST.md vision and epics  
**Method:** Sim 1 (User Story) → Sim 2 (Feature Gap) → Sim 3 (Implementation)  
**Reference:** USER_STORY_PERFECT_RUST.md, PLAN_BUILD_KOVA.md  
**Date:** 2026-03-04

---

## Original Intent (Summary)

**Vision:** Senior engineer describes what they need. Kova orchestrates local models to: Understand → Plan → Validate → Refine → Print. Output: compiles, tests pass, clippy clean, matches project style.

**"Perfect" Rust:** cargo check ✓, cargo test ✓, clippy ✓, idiomatic, compression_map (fN/tN/sN), no banned words, Result/Option correct.

---

## Sim 1: User Story Analysis

**Personas:** Senior Rust Engineer (primary), Architect (secondary)

---

### Simulation 1: Senior Engineer (Code Gen Flow)

**Scenario:** Engineer says "add exponential backoff to the retry in compute.rs". Expects Kova to generate code that compiles and fits project style.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Send message via GUI or serve | Request accepted | ✓ GUI and serve accept; FullPipeline or router path |
| 2 | Router classifies | code_gen | ✓ Router returns CodeGen for "add" patterns |
| 3 | Coder receives context | system + persona + Cursor rules + compute.rs + Cargo.toml | ✓ serve: cursor_prompts injected; GUI: no cursor_prompts in CodeGen path |
| 4 | Generate → validate | cargo check → fix loop | ✓ f81 pipeline: check → clippy → test |
| 5 | Max 2 retries (DDI) | Fix loop caps at 2 | ✓ max_fix_retries default 2 |
| 6 | Output | Code in ```rust blocks, diff, Apply/Copy | ✓ GUI has Copy, Apply, diff; serve streams raw output |
| 7 | Project conventions | compression_map, tokenization in prompt | ✓ Baked rules; serve injects. GUI CodeGen path does NOT inject cursor_prompts |

**Pain points:** GUI CodeGen path builds `system + persona` only—no Cursor rules. Serve path has full injection. Inconsistency.

---

### Simulation 2: Architect (Conventions Enforcement)

**Scenario:** Architect wants generated code to follow compression_map, protocols, no banned words.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Cursor rules in model context | Baked + external rules | ✓ Baked: blocking, augment, tokenization, compression_map |
| 2 | prompts_enabled toggle | Config to disable | ✓ [cursor] prompts_enabled |
| 3 | protocol_map, compression_map | In Coder/Fix prompts | ✓ Baked compression_map; protocol_map only if in workspace docs |
| 4 | Project-specific rules | workspace .cursor/rules | ✓ Appended when present |

**Pain points:** GUI does not inject cursor_prompts for CodeGen. Architect's edits to .cursor/rules apply in serve but not in GUI CodeGen path.

---

### Simulation 3: Engineer (Clarification Flow)

**Scenario:** Engineer says "fix the bug". Expects Kova to ask "Which file?" before generating.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Router sees ambiguous input | needs_clarification | ✓ Router returns NeedsClarification |
| 2 | GUI shows question | "Which file? (compute.rs / plan.rs / lib.rs)" | ✓ clarification_question() provides canned fallback |
| 3 | User clarifies, re-sends | Router → code_gen | ✓ Flow exists |
| 4 | Keyword bypass | "run pipeline" → no router | ✓ f62 matches FullPipeline, etc. |

**Pain points:** None for this flow. Works.

---

### User Story Coverage Summary (Sim 1)

| Epic | Key Stories | Status |
|------|-------------|--------|
| 1. Intent → Routing | US-1.1 classify, US-1.2 keyword bypass, US-1.3 clarification | ✓ |
| 2. Code Gen Pipeline | US-2.1 system+persona, US-2.3 validate, US-2.4 clippy, US-2.5 test | ✓ (GUI has cursor_prompts via build_system_prompt_impl) |
| 3. Model Orchestration | US-3.1–3.4 config, resident, on-demand | ✓ |
| 4. Context | US-4.1 project, US-4.2 files, US-4.3 compression_map | ✓ |
| 5. Feedback Loop | US-5.1 stderr→fix, US-5.2 test failure, US-5.3 chain | ✓ |
| 6. Output | US-6.1 Copy, US-6.2 diff, US-6.3 backlog, US-6.4 stream | ✓ GUI; serve streams only |
| 7–10. Recursive Academy | Cursor prompts, trace, explain, DDI | ✓ (recent) |

---

## Sim 2: Feature Gap Analysis

**Method:** Acceptance criteria vs current implementation.

---

### Epic 2: Code Generation Pipeline

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| US-2.1 Coder receives system+persona | Yes | ✓ serve; ✓ GUI (cursor_prompts via build_system_prompt_impl) | None |
| US-2.2 Target file/region | "add to plan.rs" → model gets file | ✓ context_loader extracts .rs from input | None |
| US-2.3 Validate before output | cargo check → fix → retry | ✓ | None |
| US-2.4 Clippy | After check | ✓ run_clippy config | None |
| US-2.5 Tests pass | cargo test in loop | ✓ | None |

---

### Epic 4: Context & Project Awareness

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| US-4.1 Default project | config or cwd | ✓ default_project() | None |
| US-4.2 Read relevant files | "add to plan.rs" → plan.rs | ✓ f82 extracts, loads | None |
| US-4.3 compression_map in prompt | Coder sees fN/tN/sN | ✓ serve; ✓ GUI | None |
| US-4.4 Cargo.toml deps | Model sees deps | ✓ context_loader | None |

---

### Epic 6: Output & Integration

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| US-6.1 Copy | One-click | ✓ GUI Copy button | Serve: no Copy (stream only) |
| US-6.2 Diff | Before apply | ✓ GUI diff | Serve: no diff UI |
| US-6.3 Backlog | "Run later" | ✓ Backlog in GUI | Serve: no backlog API for add |
| US-6.4 Stream | Tokens stream | ✓ Both | None |

---

### Prioritized Gaps

| # | Gap | Severity | Fix |
|---|-----|----------|-----|
| 1 | GUI CodeGen path omits cursor_prompts | High | ✓ Closed (build_system_prompt_impl) |
| 2 | Serve: no diff, no Apply, no Copy | Medium | Web GUI has diff, Apply, Copy (parity) |
| 3 | Serve: no backlog add | Low | POST /api/backlog exists |

---

## Sim 3: Implementation Deep Dive

**Focus:** Code paths, consistency, robustness.

---

### Cursor Prompts Injection Points

| Location | Injects cursor_prompts? |
|----------|-------------------------|
| serve.rs api_intent (FullPipeline) | ✓ |
| gui.rs CodeGen (RouterResult::CodeGen) | ✗ |
| gui.rs use_coder (refactor, explain, fix, custom) | ✗ |
| pipeline fix_loop | ✓ (receives project_dir) |
| academy explain_trace | ✓ |

**Finding:** GUI uses `format!("{}\n\n{}", system_prompt, persona)` for all Coder paths. Serve uses `format!("{}\n\n{}\n\n--- Cursor rules ---\n{}", system, persona, cursor)`. Unify.

---

### Pipeline Flow (f81)

- ✓ system_prompt + context_block → code_gen_prompt
- ✓ extract_rust_block → temp dir → cargo_check → fix_and_retry
- ✓ clippy, cargo_test in loop
- ✓ max_retries from config
- ✓ LastTrace written on all exits
- ✓ DDI: default max_fix_retries = 2

---

### Router (f79)

- ✓ Classifies: code_gen, refactor, explain, fix, run, custom, needs_clarification
- ✓ needs_clarification|Question format
- ✓ clarification_question() fallback
- ✓ use_coder() for code_gen, refactor, explain, fix, custom

---

### Recommendations (Sim 3)

1. **Inject cursor_prompts in GUI CodeGen** — Match serve. In gui.rs CodeGen branch, add `load_cursor_prompts(&project)` to system prompt.
2. **Inject in GUI use_coder path** — refactor, explain, fix, custom also need conventions when generating.
3. **Web GUI enhancements** — Add diff view, Copy button for serve mode. Optional.
4. **Run TRIPLE SIMS** — After fixes, run `cargo run -p kova-test --features tests` (baked-in) and re-run this AI-assisted eval (Sim 1–3) for regression.

---

## Execution Instructions

**To run this evaluation (AI-assisted):**

1. Open this document in Cursor with Kova workspace.
2. Prompt: "EVALUATE KOVA AGAINST ORIGINAL INTENT WITH TRIPLE SIMS — run Sim 1, 2, 3. Update Expected/Observed, Gaps, Recommendations based on current codebase."
3. AI walks through each Sim, inspects code, fills tables, proposes fixes.
4. Re-run after fixes to verify.

**To run baked-in TRIPLE SIMS (deterministic):**

```bash
cargo run -p kova-test --features tests
```

Runs: clippy, cargo test 3x, release build, bootstrap smoke, serve smoke.
