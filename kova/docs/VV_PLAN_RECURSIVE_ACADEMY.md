# Verification & Validation Plan (V&V Plan)

**Project:** Kova — Recursive Academy, Cursor Prompts, Trace/Explain, DDI

**Source:** SRS_RECURSIVE_ACADEMY.md, RTM_RECURSIVE_ACADEMY.md

**Date:** 2026-03-04

---

## 1. Scope

This plan defines how each requirement in the SRS will be verified. Verification methods: **Test**, **Demonstration**, **Inspection**.

---

## 2. Verification Procedures

### 2.1 Epic 7: Cursor Prompts

| Req | Method | Procedure | Pass | Fail |
|-----|--------|-----------|------|------|
| REQ-701 | Test | 1. Ensure `~/.cursor/rules/*.mdc` and `{workspace}/.cursor/rules/*.mdc` exist with content. 2. Run pipeline with `prompts_enabled=true`. 3. Assert Coder system prompt contains concatenated rule content. | Coder prompt includes rule text from both locations | Rule text absent or incomplete |
| REQ-702 | Demonstration | 1. Add new `.mdc` file to `.cursor/rules`. 2. Run pipeline without restart. 3. Observe Coder output. | New rule content appears in next run; no `~/.kova/prompts` edit | Requires edit to ~/.kova |
| REQ-703 | Test | 1. Place `protocol_map.md` and `compression_map.md` in workspace `docs/`. 2. Run pipeline. 3. Assert Coder and Fix prompts contain both docs. | Both docs present in Coder and Fix prompts | One or both absent |
| REQ-704 | Test | 1. Set `[cursor] prompts_enabled = false`. 2. Run pipeline. 3. Assert Coder prompt has no Cursor rules section. | No injection; pipeline runs as before | Rules still injected |

---

### 2.2 Epic 8: Trace & Explain

| Req | Method | Procedure | Pass | Fail |
|-----|--------|-----------|------|------|
| REQ-801 | Test | 1. Run pipeline (success or failure). 2. Call POST `/api/explain/run`. 3. Assert response has `explanation` field with plain-English text. | Explanation returned; references trace fields | No explanation or non-plain-English |
| REQ-802 | Inspection | 1. Open `academy.rs` (or equivalent). 2. Inspect `explain_trace` system prompt. | Prompt includes `load_cursor_prompts()` or equivalent | No Cursor prompts in explain prompt |
| REQ-803 | Demonstration | 1. Open web GUI. 2. Run pipeline. 3. Click "Explain last run". 4. Observe stream area. | Explanation displayed in stream area | Button missing or no display |
| REQ-804 | Test | 1. Run pipeline to success. 2. Assert GET `/api/explain` returns trace. 3. Run pipeline to failure. 4. Assert GET `/api/explain` returns trace. | Trace present in both cases | Trace missing on success or failure |
| REQ-805 | Test | 1. Start serve with no prior run. 2. Call POST `/api/explain/run`. 3. Assert 404 or equivalent with message "No trace. Run a pipeline first." | 404 + clear message | 200 or unclear message |

---

### 2.3 Epic 9: DDI

| Req | Method | Procedure | Pass | Fail |
|-----|--------|-----------|------|------|
| REQ-901 | Inspection | 1. Open `config.rs` or default `config.toml`. 2. Inspect `max_fix_retries` default. 3. Inspect comment. | Default = 2; comment references DDI | Default ≠ 2 or no DDI comment |
| REQ-902 | Test | 1. Force pipeline to exceed retry limit. 2. Assert Coder is re-prompted with summary, not full stderr chain. | Fresh start invoked; summary used | Full chain passed or no fresh start |
| REQ-903 | Inspection | 1. Open academy explain system prompt. 2. Search for "DDI" or "fix loop" or "retries". | Prompt explains retry cap and DDI | No DDI reference |

---

### 2.4 Epic 10: Full Academy

| Req | Method | Procedure | Pass | Fail |
|-----|--------|-----------|------|------|
| REQ-1001 | Demonstration | 1. Run `kova academy generate` or load modules. 2. Browse modules via GUI or API. 3. Inspect structure. | Each module has What Happened, Why, How to Fix, Exercise | Structure incomplete |
| REQ-1002 | Test | 1. Add traces and modules. 2. Submit RAG query "why did Kova do X?". 3. Assert response has citations. | Answer with top-k citations | No citations or wrong retrieval |
| REQ-1003 | Test | 1. Populate `failures.json`. 2. Run `kova academy generate`. 3. Inspect `~/.kova/academy/modules/`. | Modules exist; content from traces | Hallucinated or missing content |
| REQ-1004 | Demonstration | 1. Trigger explanation. 2. Click thumbs up/down. 3. Inspect `~/.kova/academy/feedback.json`. | Rating stored | No storage or wrong format |

---

## 3. Verification Summary

| Method | Count | Requirements |
|--------|-------|--------------|
| Test | 10 | REQ-701, 703, 704, 801, 804, 805, 902, 1002, 1003 |
| Demonstration | 4 | REQ-702, 803, 1001, 1004 |
| Inspection | 3 | REQ-802, 901, 903 |

---

## 4. Environment

- **Platform:** Linux (WSL2 acceptable)
- **Build:** `cargo build -p kova --features "serve,inference"`
- **Serve:** `cargo run -p kova --bin kova --features "serve,inference" -- serve`
- **GUI:** Browser at `http://127.0.0.1:3002`

---

## 5. Artifact Chain

| Artifact | File |
|----------|------|
| User Stories | USER_STORY_RECURSIVE_ACADEMY.md |
| RTM | RTM_RECURSIVE_ACADEMY.md |
| SRS | SRS_RECURSIVE_ACADEMY.md |
| V&V Plan | VV_PLAN_RECURSIVE_ACADEMY.md |
| SDD | SDD_RECURSIVE_ACADEMY.md |
| Test Plan | TEST_PLAN_RECURSIVE_ACADEMY.md |
