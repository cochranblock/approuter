# Plan: Build Kova (GUI-First)

**Approach:** Single binary, egui GUI. No serve mode initially. Ship something usable.

**Audience:** Beginning of tech journey. Simplicity over scale.

**Date:** 2026-03-02

---

## Phase 0: Make It Build ✓

- [x] Add kova to workspace (`$HOME/Cargo.toml` members)
- [x] `cargo build -p kova` succeeds
- [x] `kova gui` opens a window
- [x] GUI default, serve optional

**Exit:** Window opens. Chat input visible. No inference yet.

---

## Phase 1: Prompts & Config ✓

- [x] Create `~/.kova/` on first run (bootstrap)
- [x] Create `~/.kova/prompts/` with:
  - `system.md` — base system prompt (augment, protocols, style)
  - `persona.md` — your persona (Senior Systems Architect, etc.)
- [x] Create `~/.kova/config.toml` — paths, bind (for future serve)
- [x] Load prompts in GUI; show in collapsible "Prompts" panel

**Exit:** `kova gui` creates ~/.kova, prompts load, user can edit them.

---

## Phase 2: Chat Shell (No LLM Yet) ✓

- [x] Chat input → append to conversation log
- [x] "You:" and "Assistant:" styling (blue/purple per THEME)
- [x] Mock response: "Run full-pipeline? (y/n)" etc. from keywords
- [x] Conversation persists in sled (`conv:default:messages`)
- [x] Restart → conversation restored

**Exit:** Chat UI works. Mock responses. Persistence.

---

## Phase 3: Intent Parsing (Keyword) ✓

- [x] f62_parse_intent: map keywords → intent
  - "full" / "pipeline" → full-pipeline
  - "tunnel" → tunnel-update
  - "setup rogue/repo" → setup-roguerepo
  - "cloudflare" / "purge" / "cache" → cloudflare-purge
  - "test" → test
  - "compile" / "build" → compile
  - "fix warn" → fix-warnings
- [x] When intent matches → show "Run {intent}? (y/n)" in chat
- [ ] User confirms → execute (Phase 4)

**Exit:** Natural-ish input maps to intents. No LLM.

---

## Phase 4: Execution ✓

- [x] Restore plan.rs (intent → action DAG)
- [x] Restore compute.rs (run cargo, approuter, etc.)
- [x] Wire f62 match → plan → execute (y/n confirm)
- [x] Output to GUI (append to log)
- [x] Show ✓/✗ when done

**Exit:** "run pipeline" → "Run full-pipeline? (y/n)" → "y" → cargo check, cargo test.

---

## Phase 5: Backlog in GUI ✓

- [x] Load backlog from `~/.kova/backlog.json`
- [x] List view: intent, project, cmd
- [x] "Run" button per item
- [x] "Run all" button
- [x] Refresh on each view (reload from disk)

**Exit:** Backlog visible. Run single or all.

---

## Phase 7a: Model Registry ✓

- [x] `[models]` router, coder, fix in config.toml
- [x] `[orchestration]` router_resident, max_fix_retries, run_clippy
- [x] `f78(ModelRole)` — path lookup: env > config > default
- [x] `kova model list` — show configured models
- [x] Bootstrap creates models_dir

**Exit:** Config-driven model roles. Ready for 7b router.

---

## Phase 7b: Router Model ✓

- [x] `router.rs` — f79 classify, RouterResult enum
- [x] Classification: code_gen, refactor, explain, fix, run, custom, needs_clarification
- [x] When f62 returns None: router → classify → coder or clarification
- [x] GUI: "Classifying…" then stream or "Could you clarify?"

**Exit:** Non-keyword requests go through router first. Ready for 7c.

---

## Phase 7c: Coder Pipeline ✓

- [x] `pipeline.rs` — f81 run_code_gen_pipeline
- [x] Flow: generate → extract ```rust blocks → cargo check in temp dir → on failure, fix model → retry (max 2)
- [x] `inference.rs` — f80 chat_complete (non-streaming)
- [x] GUI: code_gen uses pipeline instead of streaming; pipeline_receiver state

**Exit:** Code gen goes through validate→fix loop. Ready for 7d context loading.

---

## Phase 7d: Context Loading ✓

- [x] `default_project()` — KOVA_PROJECT > config [paths] project > cwd
- [x] `context_loader.rs` — f82 load_project_context
- [x] Load Cargo.toml from project dir (US-4.4)
- [x] Extract .rs filenames from user input, load from src/ or project root (US-4.2)
- [x] Pipeline injects context block into coder prompt

**Exit:** Model sees Cargo.toml deps and mentioned file contents. Ready for 7e.

---

## Phase 7e: Fix Loop + Clippy/Test ✓

- [x] After cargo check: run clippy (if orchestration.run_clippy)
- [x] After clippy: run cargo test
- [x] Clippy/test failures → fix model with stderr → retry (shared max_retries)
- [x] Refinement chain in output: "Attempt 1: compile failed → Attempt 2: ✓"

**Exit:** Full validation chain. Ready for 7f output.

---

## Phase 7f: Output (Diff, Apply, Copy) ✓

- [x] Copy button — extracts ```rust block, copies to clipboard
- [x] Apply button — writes to target file (hint from user msg or src/lib.rs)
- [x] Show diff — collapsible unified diff (current vs generated)
- [x] output.rs — f84 format_diff, f85 resolve_target_path

**Exit:** One-click copy, diff before apply, apply to file. Ready for 7g.

---

## Phase 6: Local LLM ✓

- [x] Add `inference` feature (Kalosm)
- [x] `kova model install` — download Qwen2.5-Coder-0.5B-Instruct Q4_K_M GGUF
- [x] When user sends message: if f62 matches → use keyword path; else → call LLM with prompts
- [x] Stream tokens to chat
- [ ] LLM can suggest intent → user confirms (future)

**Exit:** Real chat with local model. Fallback to keyword when offline or no model.

---

## Phase 7: Polish

- [ ] Theme: neon blue/teal/purple per THEME.md
- [ ] Error messages: "Model not found. Run kova model install."
- [ ] `kova bootstrap` — one-command setup
- [ ] Help text in GUI

**Exit:** Looks good. Clear errors. One-command bootstrap.

---

## Out of Scope (For Now)

- HTTP API / serve mode
- Remote access
- Multi-user
- Gamification (per P23)

---

## File Layout (Target)

```
kova/
├── Cargo.toml          # gui default, serve optional, inference optional
├── src/
│   ├── lib.rs
│   ├── main.rs
│   ├── config.rs
│   ├── backlog.rs
│   ├── intent.rs
│   ├── storage.rs
│   ├── gui.rs          # egui app
│   ├── plan.rs         # intent → actions (Phase 4)
│   ├── compute.rs      # execute actions (Phase 4)
│   └── serve.rs        # optional, Phase 7+
├── docs/
│   ├── PLAN_BUILD_KOVA.md
│   └── THEME.md
└── ...
```

---

## Milestones

| M | Phase | Target |
|---|-------|--------|
| M0 | 0 | Builds, window opens |
| M1 | 1–2 | Prompts, chat shell, persistence |
| M2 | 3–4 | Keyword intent → execution |
| M3 | 5 | Backlog in GUI |
| M4 | 6–7 | LLM optional, polish |

---

## Commands (Final)

```bash
kova              # or kova gui — open GUI (default)
kova bootstrap    # create ~/.kova, prompts, config
kova model install # download GGUF (if inference feature)
```
