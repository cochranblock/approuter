# Software Requirements Specification (SRS)

**Project:** Kova — Recursive Academy, Cursor Prompts, Trace/Explain, DDI

**Source:** USER_STORY_RECURSIVE_ACADEMY.md, RTM_RECURSIVE_ACADEMY.md

**Date:** 2026-03-04

---

## 1. Scope

This SRS defines the software requirements for the Recursive Academy feature set: Cursor prompts as training data, trace capture, explain-last-run, DDI-aware fix loop, and full academy curriculum generation.

---

## 2. Requirements

### 2.1 Epic 7: Cursor Prompts as Training Data

**REQ-701**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-7.1  

The system shall load Cursor rules from `~/.cursor/rules/*.mdc` and `{workspace}/.cursor/rules/*.mdc` and inject them into the Coder model system prompt when Cursor prompts are enabled.

**Verification:** Test (TC-701)

---

**REQ-702**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-7.2  

The system shall discover Cursor rules at runtime such that adding or changing files in `.cursor/rules` causes Kova to pick up the changes on the next pipeline run without requiring edits to `~/.kova/prompts`.

**Verification:** Demonstration (TC-702)

---

**REQ-703**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-7.3  

The system shall load `protocol_map.md` and `compression_map.md` from `{workspace}/docs` or `{workspace}` and inject them into the Coder and Fix model system prompts when Cursor prompts are enabled.

**Verification:** Test (TC-703)

---

**REQ-704**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-7.4  

The system shall support a configuration option `[cursor] prompts_enabled` such that when set to `false`, no Cursor prompts are injected and the pipeline behavior is unchanged from the pre-Cursor-prompts baseline.

**Verification:** Test (TC-704)

---

### 2.2 Epic 8: Trace & Explain

**REQ-801**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-8.1  

The system shall provide an "Explain last run" capability that, when invoked, passes the last pipeline trace (intent, user message, stage, stderr, retry count, outcome, chain) to a model and returns a plain-English explanation.

**Verification:** Test (TC-801)

---

**REQ-802**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-8.2  

The system shall include Cursor prompts in the Academy explain system prompt such that the model may reference conventions (e.g., tokenization, compression_map) when relevant in its explanation.

**Verification:** Inspection (TC-802)

---

**REQ-803**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-8.3  

The system shall provide an "Explain last run" button in the web GUI that, when clicked, issues POST to `/api/explain/run` and displays the returned explanation in the stream area.

**Verification:** Demonstration (TC-803)

---

**REQ-804**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-8.4  

The system shall write the LastTrace data structure on every pipeline exit, whether the outcome is success or failure.

**Verification:** Test (TC-804)

---

**REQ-805**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-8.5  

The system shall return a clear message when "Explain last run" is invoked and no prior pipeline run exists (e.g., "No trace. Run a pipeline first." or HTTP 404 with equivalent body).

**Verification:** Test (TC-805)

---

### 2.3 Epic 9: DDI-Aware Fix Loop

**REQ-901**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-9.1  

The system shall default `orchestration.max_fix_retries` to 2, with a config comment referencing the Debugging Decay Index (DDI) research.

**Verification:** Inspection (TC-901)

---

**REQ-902**  
**Type:** Functional  
**Priority:** P1  
**Source:** US-9.2  

The system shall implement a strategic fresh start when the fix loop exceeds the retry limit: re-prompt the Coder model with the original user intent and a summary of the failure (e.g., first 500 chars of stderr), not the full stderr chain.

**Verification:** Test (TC-902)  
**Note:** Deferred to Phase 1.

---

**REQ-903**  
**Type:** Functional  
**Priority:** P0  
**Source:** US-9.3  

The system shall include in the Academy explain prompt a reference to DDI such that explanations may state that the fix loop loses effectiveness after 2–3 attempts and that retries are capped to avoid worse output.

**Verification:** Inspection (TC-903)

---

### 2.4 Epic 10: Recursive Academy (Full)

**REQ-1001**  
**Type:** Functional  
**Priority:** P2  
**Source:** US-10.1  

The system shall provide Academy modules, one per failure pattern, with structure: What Happened, Why, How to Fix, Exercise. Modules shall be browsable via GUI or API.

**Verification:** Demonstration (TC-1001)

---

**REQ-1002**  
**Type:** Functional  
**Priority:** P2  
**Source:** US-10.2  

The system shall support RAG queries over traces and Academy modules such that a user question (e.g., "why did Kova do X?") retrieves top-k relevant documents and the model answers with citations.

**Verification:** Test (TC-1002)

---

**REQ-1003**  
**Type:** Functional  
**Priority:** P2  
**Source:** US-10.3  

The system shall provide an `kova academy generate` command that reads `failures.json`, for each pattern loads an example trace, generates an explanation via the model, and writes a module to `~/.kova/academy/modules/<pattern_slug>.md`. Content shall be derived from real traces only; no hallucinated content.

**Verification:** Test (TC-1003)

---

**REQ-1004**  
**Type:** Functional  
**Priority:** P2  
**Source:** US-10.4  

The system shall support thumbs up/down feedback on explanations, storing ratings in `~/.kova/academy/feedback.json`. Future use: prioritize low-rated explanations for regeneration.

**Verification:** Demonstration (TC-1004)

---

## 3. Requirements Summary

| ID | Type | Priority | Verification |
|----|------|----------|--------------|
| REQ-701 | Functional | P0 | Test |
| REQ-702 | Functional | P0 | Demonstration |
| REQ-703 | Functional | P0 | Test |
| REQ-704 | Functional | P0 | Test |
| REQ-801 | Functional | P0 | Test |
| REQ-802 | Functional | P0 | Inspection |
| REQ-803 | Functional | P0 | Demonstration |
| REQ-804 | Functional | P0 | Test |
| REQ-805 | Functional | P0 | Test |
| REQ-901 | Functional | P0 | Inspection |
| REQ-902 | Functional | P1 | Test |
| REQ-903 | Functional | P0 | Inspection |
| REQ-1001 | Functional | P2 | Demonstration |
| REQ-1002 | Functional | P2 | Test |
| REQ-1003 | Functional | P2 | Test |
| REQ-1004 | Functional | P2 | Demonstration |

---

## 4. Artifact Chain

| Artifact | File |
|----------|------|
| User Stories | USER_STORY_RECURSIVE_ACADEMY.md |
| RTM | RTM_RECURSIVE_ACADEMY.md |
| SRS | SRS_RECURSIVE_ACADEMY.md |
| V&V Plan | VV_PLAN_RECURSIVE_ACADEMY.md |
| SDD | SDD_RECURSIVE_ACADEMY.md |
| Test Plan | TEST_PLAN_RECURSIVE_ACADEMY.md |
