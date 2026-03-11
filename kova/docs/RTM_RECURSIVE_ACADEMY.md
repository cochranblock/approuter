# Requirements Traceability Matrix (RTM)

**Project:** Kova — Recursive Academy, Cursor Prompts, Trace/Explain, DDI

**Source:** USER_STORY_RECURSIVE_ACADEMY.md

**Date:** 2026-03-04

---

## Traceability: User Story → Requirement → Design → Test

| User Story | Requirement ID | Design Element | Test Case ID |
|------------|----------------|----------------|--------------|
| US-7.1 | REQ-701 | cursor_prompts::load_cursor_prompts, pipeline injection | TC-701 |
| US-7.2 | REQ-702 | cursor_prompts discovery order, no ~/.kova edit | TC-702 |
| US-7.3 | REQ-703 | cursor_prompts protocol_map, compression_map paths | TC-703 |
| US-7.4 | REQ-704 | config [cursor] prompts_enabled | TC-704 |
| US-8.1 | REQ-801 | trace::LastTrace, academy::explain_trace, POST /api/explain/run | TC-801 |
| US-8.2 | REQ-802 | academy system prompt + cursor_prompts | TC-802 |
| US-8.3 | REQ-803 | assets/app.html Explain button | TC-803 |
| US-8.4 | REQ-804 | pipeline last_trace write on all exits | TC-804 |
| US-8.5 | REQ-805 | api_explain_run 404, message | TC-805 |
| US-9.1 | REQ-901 | config orchestration max_fix_retries default 2 | TC-901 |
| US-9.2 | REQ-902 | pipeline strategic_fresh_start (future) | TC-902 |
| US-9.3 | REQ-903 | academy explain prompt DDI reference | TC-903 |
| US-10.1 | REQ-1001 | academy modules, kova academy generate | TC-1001 |
| US-10.2 | REQ-1002 | RAG DocumentTable, query flow | TC-1002 |
| US-10.3 | REQ-1003 | failures.json, trace-to-module | TC-1003 |
| US-10.4 | REQ-1004 | feedback.json, thumbs up/down | TC-1004 |

---

## Reverse Traceability: Requirement → User Story

| Requirement ID | User Story | Source Acceptance Criteria |
|----------------|------------|----------------------------|
| REQ-701 | US-7.1 | Coder receives ~/.cursor/rules + project rules. Output reflects conventions. |
| REQ-702 | US-7.2 | Add/change rules → Kova picks up. No ~/.kova edit. |
| REQ-703 | US-7.3 | Coder and Fix receive protocol_map, compression_map. fN/tN/sN applied. |
| REQ-704 | US-7.4 | prompts_enabled = false → no injection. |
| REQ-801 | US-8.1 | Explain last run → model returns plain-English. |
| REQ-802 | US-8.2 | Explanation cites conventions when relevant. |
| REQ-803 | US-8.3 | Web GUI button → POST → explanation displayed. |
| REQ-804 | US-8.4 | LastTrace on every exit (success or failure). |
| REQ-805 | US-8.5 | No trace → "No trace. Run a pipeline first." |
| REQ-901 | US-9.1 | max_fix_retries default = 2. Config comment. |
| REQ-902 | US-9.2 | After 2 retries: strategic fresh start. |
| REQ-903 | US-9.3 | Academy explains DDI. |
| REQ-1001 | US-10.1 | Modules: What Happened, Why, How to Fix, Exercise. |
| REQ-1002 | US-10.2 | RAG query → top-k → answer with citations. |
| REQ-1003 | US-10.3 | academy generate from failures.json. |
| REQ-1004 | US-10.4 | Thumbs up/down stored. |

---

## Coverage Summary

| Epic | Stories | Requirements | Design Elements | Test Cases |
|------|---------|--------------|-----------------|------------|
| 7. Cursor Prompts | 4 | 4 | cursor_prompts.rs, config | 4 |
| 8. Trace & Explain | 5 | 5 | trace.rs, academy.rs, serve, app.html | 5 |
| 9. DDI | 3 | 3 | config, pipeline, academy | 3 |
| 10. Full Academy | 4 | 4 | academy modules, RAG, failures.json | 4 |
| **Total** | **16** | **16** | — | **16** |

---

## Artifact Chain

| Artifact | File |
|----------|------|
| User Stories | USER_STORY_RECURSIVE_ACADEMY.md |
| RTM | RTM_RECURSIVE_ACADEMY.md |
| SRS | SRS_RECURSIVE_ACADEMY.md |
| V&V Plan | VV_PLAN_RECURSIVE_ACADEMY.md |
| SDD | SDD_RECURSIVE_ACADEMY.md |
| Test Plan | TEST_PLAN_RECURSIVE_ACADEMY.md |
