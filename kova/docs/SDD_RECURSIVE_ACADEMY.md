# Software Design Description (SDD)

**Project:** Kova — Recursive Academy, Cursor Prompts, Trace/Explain, DDI

**Source:** SRS_RECURSIVE_ACADEMY.md, RTM_RECURSIVE_ACADEMY.md, BUILD_STEPS_RECURSIVE_ACADEMY.md

**Date:** 2026-03-04

---

## 1. Scope

This SDD describes the design elements for the Recursive Academy feature set. It maps requirements to modules, interfaces, and data flows.

---

## 2. Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────┐
│  serve (AppState)                                                       │
│  - pipeline_rx, last_trace                                              │
│  - Routes: /api/intent, /api/explain, /api/explain/run, /ws/stream      │
└───────────────────────────────┬─────────────────────────────────────────┘
                                │
        ┌───────────────────────┼───────────────────────┐
        ▼                       ▼                       ▼
┌───────────────┐     ┌─────────────────┐     ┌─────────────────┐
│ cursor_prompts│     │ pipeline       │     │ academy         │
│ load_cursor_  │────▶│ f81, run_      │────▶│ explain_trace   │
│ prompts()     │     │ pipeline       │     │                 │
└───────────────┘     └───────┬────────┘     └────────┬────────┘
        │                     │                       │
        │                     ▼                       │
        │             ┌───────────────┐               │
        │             │ trace::      │◀──────────────┘
        │             │ LastTrace    │   (trace input)
        │             └──────────────┘               │
        │                     │                       │
        └─────────────────────┼───────────────────────┘
                              │
                              ▼
                    ┌─────────────────┐
                    │ inference::f80   │
                    │ (Coder model)   │
                    └─────────────────┘
```

---

## 3. Module Design

### 3.1 cursor_prompts (new)

**File:** `src/cursor_prompts.rs`

**Purpose:** Load Cursor rules, skills, AGENTS.md, protocol_map, compression_map for injection into model prompts.

**Interface:**
```rust
pub fn load_cursor_prompts(workspace_root: &Path) -> String
```

**Discovery order:**
1. `~/.cursor/rules/*.mdc`
2. `{workspace}/.cursor/rules/*.mdc`
3. `~/.cursor/skills-cursor/*/SKILL.md`
4. `~/.cursor/skills/*/SKILL.md`
5. `{workspace}/AGENTS.md`
6. `{workspace}/docs/protocol_map.md` or `{workspace}/protocol_map.md`
7. `{workspace}/docs/compression_map.md` or `{workspace}/compression_map.md`

**Returns:** Concatenated content with section headers, or empty if disabled/not found.

**Config dependency:** `[cursor] prompts_enabled` — when false, returns empty.

---

### 3.2 trace (new)

**File:** `src/trace.rs`

**Purpose:** In-memory representation of last pipeline run for Explain.

**Data structure:**
```rust
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LastTrace {
    pub intent: String,
    pub user_msg: String,
    pub stage: String,        // "compile" | "clippy" | "tests"
    pub stderr: String,
    pub retry_count: u32,
    pub outcome: String,     // "success" | "failed"
    pub chain: Vec<String>,  // "Attempt 1: compile failed" etc.
}
```

**Ownership:** Held in `AppState.last_trace: Arc<Mutex<Option<LastTrace>>>`. Pipeline writes; serve reads.

---

### 3.3 academy (new)

**File:** `src/academy.rs`

**Purpose:** Explain trace via model inference.

**Interface:**
```rust
pub async fn explain_trace(trace: &LastTrace, model_path: &Path) -> Result<String, String>
```

**Logic:**
- Format trace as readable text
- System prompt: Recursive Academy persona + Cursor prompts (via `load_cursor_prompts`) + DDI reference
- User message: trace content
- Call `inference::f80` with Coder model
- Return response string

---

### 3.4 pipeline (modified)

**File:** `src/pipeline/mod.rs`

**Changes:**
- Add param `last_trace: Option<Arc<Mutex<LastTrace>>>` to `f81` and `run_pipeline`
- At start: init `LastTrace { intent, user_msg, .. }`
- On each failure: update `stage`, `stderr`, `retry_count`, `chain`
- On success/failure exit: set `outcome`, write to `last_trace` if Some
- Inject `load_cursor_prompts(project_dir)` into Coder system prompt when enabled

**File:** `src/pipeline/fix_loop.rs`

**Changes:**
- Inject `load_cursor_prompts` into Fix system prompt when enabled

---

### 3.5 config (modified)

**File:** `src/config.rs`

**New section:**
```toml
[cursor]
prompts_enabled = true   # default
```

**Existing:** `orchestration.max_fix_retries` — default 2 (already present). Add comment referencing DDI.

---

### 3.6 serve (modified)

**File:** `src/serve.rs`

**AppState addition:**
```rust
last_trace: Arc<Mutex<Option<crate::trace::LastTrace>>>
```

**New routes:**
- `GET /api/explain` — return trace JSON or 404
- `POST /api/explain/run` — run `explain_trace`, return `{"explanation": "..."}` or 404

**Pipeline wiring:** Pass `state.last_trace.clone()` to `f81` when spawning FullPipeline.

---

### 3.7 assets/app.html (modified)

**Change:** Add "Explain last run" button. On click: POST `/api/explain/run`, display `explanation` in stream area.

---

## 4. Data Flows

| Flow | Source | Sink | Data |
|------|--------|------|------|
| Cursor prompts → Coder | cursor_prompts | pipeline | String (concatenated rules) |
| Cursor prompts → Fix | cursor_prompts | fix_loop | String |
| Cursor prompts → Academy | cursor_prompts | academy | String |
| Pipeline → LastTrace | pipeline | AppState.last_trace | LastTrace |
| LastTrace → Explain | AppState | academy::explain_trace | LastTrace |
| Explain → Client | academy | HTTP response | `{"explanation": "..."}` |

---

## 5. Phase 1 (Deferred) Design Elements

| Element | Description |
|---------|-------------|
| strategic_fresh_start | After max retries: re-prompt Coder with summary, not full stderr chain |
| RAG DocumentTable | Kalosm DocumentTable over traces + modules for REQ-1002 |
| failures.json | Structured failure log for `kova academy generate` |
| feedback.json | Thumbs up/down storage for REQ-1004 |

---

## 6. Traceability: Design → Requirement

| Design Element | Requirements |
|----------------|--------------|
| cursor_prompts::load_cursor_prompts | REQ-701, 702, 703 |
| config [cursor] prompts_enabled | REQ-704 |
| trace::LastTrace | REQ-801, 804 |
| academy::explain_trace | REQ-801, 802, 903 |
| POST /api/explain/run | REQ-801, 805 |
| pipeline last_trace write | REQ-804 |
| assets/app.html Explain button | REQ-803 |
| config max_fix_retries = 2 | REQ-901 |
| academy DDI reference in prompt | REQ-903 |

---

## 7. Artifact Chain

| Artifact | File |
|----------|------|
| User Stories | USER_STORY_RECURSIVE_ACADEMY.md |
| RTM | RTM_RECURSIVE_ACADEMY.md |
| SRS | SRS_RECURSIVE_ACADEMY.md |
| V&V Plan | VV_PLAN_RECURSIVE_ACADEMY.md |
| SDD | SDD_RECURSIVE_ACADEMY.md |
| Test Plan | TEST_PLAN_RECURSIVE_ACADEMY.md |
