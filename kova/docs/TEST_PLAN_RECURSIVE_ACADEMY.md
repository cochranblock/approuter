# Test Plan

**Project:** Kova — Recursive Academy, Cursor Prompts, Trace/Explain, DDI

**Source:** SRS_RECURSIVE_ACADEMY.md, VV_PLAN_RECURSIVE_ACADEMY.md, RTM_RECURSIVE_ACADEMY.md

**Date:** 2026-03-04

---

## 1. Scope

This plan defines the test strategy, environments, and test case specifications for the Recursive Academy feature set.

---

## 2. Test Strategy

| Level | Method | Scope |
|-------|--------|-------|
| Unit | Rust `#[test]` | cursor_prompts, trace, academy (mock inference) |
| Integration | `cargo test -p kova --features tests` | Pipeline + trace, serve routes |
| System | Manual / curl / browser | Full flow: pipeline → explain → GUI |

---

## 3. Environment

| Item | Value |
|------|-------|
| Platform | Linux (WSL2 acceptable) |
| Build | `cargo build -p kova --features "serve,inference"` |
| Test build | `cargo test -p kova --features "tests,inference"` |
| Serve | `cargo run -p kova --bin kova --features "serve,inference" -- serve` |
| Base URL | `http://127.0.0.1:3002` |

---

## 4. Test Cases

### 4.1 Epic 7: Cursor Prompts

**TC-701** — Cursor rules loaded and injected  
- **Req:** REQ-701  
- **Type:** Integration  
- **Steps:** 1. Ensure `~/.cursor/rules/*.mdc` and `{workspace}/.cursor/rules/*.mdc` exist. 2. Run pipeline with `prompts_enabled=true`. 3. Assert Coder system prompt contains rule content.  
- **Pass:** Rule text present in prompt.  
- **Fail:** Rule text absent.

**TC-702** — Runtime discovery, no ~/.kova edit  
- **Req:** REQ-702  
- **Type:** Demonstration  
- **Steps:** 1. Add `.mdc` to `.cursor/rules`. 2. Run pipeline. 3. Verify new content used.  
- **Pass:** New rule picked up; no edit to ~/.kova.  
- **Fail:** Requires ~/.kova edit.

**TC-703** — protocol_map, compression_map in Coder/Fix  
- **Req:** REQ-703  
- **Type:** Integration  
- **Steps:** 1. Place both docs in workspace `docs/`. 2. Run pipeline. 3. Assert both in Coder and Fix prompts.  
- **Pass:** Both present.  
- **Fail:** One or both absent.

**TC-704** — prompts_enabled = false disables injection  
- **Req:** REQ-704  
- **Type:** Integration  
- **Steps:** 1. Set `[cursor] prompts_enabled = false`. 2. Run pipeline. 3. Assert no Cursor rules in prompt.  
- **Pass:** No injection.  
- **Fail:** Rules still injected.

---

### 4.2 Epic 8: Trace & Explain

**TC-801** — Explain returns plain-English  
- **Req:** REQ-801  
- **Type:** Integration  
- **Steps:** 1. Run pipeline. 2. POST `/api/explain/run`. 3. Assert `explanation` field present and readable.  
- **Pass:** Explanation returned.  
- **Fail:** No explanation or malformed.

**TC-802** — Academy prompt includes Cursor prompts  
- **Req:** REQ-802  
- **Type:** Inspection  
- **Steps:** Inspect `academy.rs` explain system prompt.  
- **Pass:** `load_cursor_prompts` or equivalent in prompt.  
- **Fail:** No Cursor prompts in explain.

**TC-803** — Web GUI Explain button  
- **Req:** REQ-803  
- **Type:** System  
- **Steps:** 1. Open GUI. 2. Run pipeline. 3. Click "Explain last run". 4. Observe stream area.  
- **Pass:** Explanation displayed.  
- **Fail:** Button missing or no display.

**TC-804** — LastTrace on every exit  
- **Req:** REQ-804  
- **Type:** Integration  
- **Steps:** 1. Run pipeline to success. 2. GET `/api/explain` → trace. 3. Run pipeline to failure. 4. GET `/api/explain` → trace.  
- **Pass:** Trace in both cases.  
- **Fail:** Trace missing.

**TC-805** — No trace → clear message  
- **Req:** REQ-805  
- **Type:** Integration  
- **Steps:** 1. Fresh serve, no prior run. 2. POST `/api/explain/run`. 3. Assert 404 + "No trace. Run a pipeline first." or equivalent.  
- **Pass:** 404 + clear message.  
- **Fail:** 200 or unclear.

---

### 4.3 Epic 9: DDI

**TC-901** — max_fix_retries default 2, DDI comment  
- **Req:** REQ-901  
- **Type:** Inspection  
- **Steps:** Inspect `config.rs` / default config.  
- **Pass:** Default = 2; comment references DDI.  
- **Fail:** Default ≠ 2 or no comment.

**TC-902** — Strategic fresh start (Phase 1)  
- **Req:** REQ-902  
- **Type:** Integration  
- **Steps:** 1. Force retry limit. 2. Assert Coder re-prompted with summary.  
- **Pass:** Fresh start invoked.  
- **Fail:** Full chain passed.

**TC-903** — Academy prompt DDI reference  
- **Req:** REQ-903  
- **Type:** Inspection  
- **Steps:** Inspect academy explain prompt.  
- **Pass:** DDI or retry cap mentioned.  
- **Fail:** No reference.

---

### 4.4 Epic 10: Full Academy

**TC-1001** — Modules browsable, structure correct  
- **Req:** REQ-1001  
- **Type:** Demonstration  
- **Steps:** Browse modules. Inspect structure.  
- **Pass:** What Happened, Why, How to Fix, Exercise.  
- **Fail:** Structure incomplete.

**TC-1002** — RAG query with citations  
- **Req:** REQ-1002  
- **Type:** Integration  
- **Steps:** 1. Add traces/modules. 2. Query "why did Kova do X?". 3. Assert citations.  
- **Pass:** Answer with citations.  
- **Fail:** No citations.

**TC-1003** — academy generate from failures.json  
- **Req:** REQ-1003  
- **Type:** Integration  
- **Steps:** 1. Populate failures.json. 2. Run `kova academy generate`. 3. Inspect modules.  
- **Pass:** Modules from traces.  
- **Fail:** Hallucinated content.

**TC-1004** — Thumbs up/down stored  
- **Req:** REQ-1004  
- **Type:** Demonstration  
- **Steps:** 1. Rate explanation. 2. Inspect feedback.json.  
- **Pass:** Rating stored.  
- **Fail:** No storage.

---

## 5. Coverage Summary

| Epic | Test Cases | Automated | Manual |
|------|------------|-----------|--------|
| 7. Cursor Prompts | 4 | TC-701, 703, 704 | TC-702 |
| 8. Trace & Explain | 5 | TC-801, 804, 805 | TC-802, 803 |
| 9. DDI | 3 | TC-902 | TC-901, 903 |
| 10. Full Academy | 4 | TC-1002, 1003 | TC-1001, 1004 |
| **Total** | **16** | **9** | **7** |

---

## 6. Test Commands

```bash
# Unit + integration
cargo test -p kova --features "tests,inference"

# Serve (manual/system tests)
cargo run -p kova --bin kova --features "serve,inference" -- serve

# API smoke
curl http://127.0.0.1:3002/api/explain
curl -X POST http://127.0.0.1:3002/api/explain/run
```

---

## 7. Artifact Chain

| Artifact | Status |
|----------|--------|
| USER_STORY_RECURSIVE_ACADEMY | ✓ |
| RTM_RECURSIVE_ACADEMY | ✓ |
| SRS_RECURSIVE_ACADEMY | ✓ |
| VV_PLAN_RECURSIVE_ACADEMY | ✓ |
| SDD_RECURSIVE_ACADEMY | ✓ |
| TEST_PLAN_RECURSIVE_ACADEMY | ✓ |

**Implementation:** BUILD_STEPS_RECURSIVE_ACADEMY.md (Steps 0–8 for MVP).
