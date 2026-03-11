# Build Steps: Recursive Academy

**Ordered, actionable steps.** Each step has file path, change, and acceptance criteria.

**User stories:** `USER_STORY_RECURSIVE_ACADEMY.md` — Epics 7–10. Validate before implementing.

**MVP target:** Trace capture + Explain last run + DDI-aware fix loop. ~4–6 hours.

---

## Step 0: Cursor Prompts as Training Data

**Goal:** Inject all Cursor prompts (rules, skills, AGENTS, protocol_map, compression_map) into Kova model context so Coder, Fix, Router, and Academy use them.

**Sources (discovered in order):**
1. `~/.cursor/rules/*.mdc` — workspace rules (blocking, augment-not-intent, tokenization, etc.)
2. `{workspace}/.cursor/rules/*.mdc` — project rules (e.g. cochranblock, kova)
3. `~/.cursor/skills-cursor/*/SKILL.md` — built-in Cursor skills
4. `~/.cursor/skills/*/SKILL.md` — user skills
5. `{workspace}/AGENTS.md` — agent references
6. `{workspace}/docs/protocol_map.md` or `protocol_map.md`
7. `{workspace}/docs/compression_map.md` or `compression_map.md`

**File:** `src/cursor_prompts.rs` (new)

**Add:**
```rust
//! Load Cursor prompts for injection into model context.

use std::path::Path;

/// Discover and concatenate all Cursor prompts. Returns empty if disabled or not found.
pub fn load_cursor_prompts(workspace_root: &Path) -> String {
    let mut out = String::new();
    // 1. ~/.cursor/rules
    if let Ok(home) = std::env::var("HOME") {
        let rules = Path::new(&home).join(".cursor/rules");
        if rules.exists() {
            for e in std::fs::read_dir(&rules).into_iter().flatten().flatten() {
                if e.path().extension().map_or(false, |e| e == "mdc") {
                    if let Ok(c) = std::fs::read_to_string(&e.path()) {
                        out.push_str(&format!("\n\n--- {} ---\n{}", e.path().display(), c));
                    }
                }
            }
        }
    }
    // 2. workspace .cursor/rules
    let wr = workspace_root.join(".cursor/rules");
    if wr.exists() { /* same pattern */ }
    // 3. skills-cursor, skills
    // 4. AGENTS.md
    if let Ok(c) = std::fs::read_to_string(workspace_root.join("AGENTS.md")) {
        out.push_str(&format!("\n\n--- AGENTS.md ---\n{}", c));
    }
    // 5. protocol_map, compression_map (kova/docs or workspace docs)
    for name in ["protocol_map.md", "compression_map.md"] {
        for base in [workspace_root.join("docs"), workspace_root.to_path_buf()] {
            let p = base.join(name);
            if p.exists() {
                if let Ok(c) = std::fs::read_to_string(&p) {
                    out.push_str(&format!("\n\n--- {} ---\n{}", name, c));
                    break;
                }
            }
        }
    }
    out
}
```

**Config:** `config.toml` add `[cursor]` section:
```toml
[cursor]
prompts_enabled = true   # default true
# Optional: override workspace root for prompt discovery
# workspace_root = "$HOME"  # or /Users/mcochran (macOS) / /home/mcochran (Linux)
```

**Wire:** `src/lib.rs` — `pub mod cursor_prompts;`

**Injection points:**
- **Pipeline (Coder):** `format!("{}\n\n--- Cursor rules ---\n{}", system_prompt, load_cursor_prompts(&project_dir))`
- **Fix loop:** Add cursor prompts to fix system prompt
- **Router:** Add if relevant (e.g. augment-not-intent)
- **Academy explain_trace:** Add to system prompt so explanations reference conventions

**Accept:** `load_cursor_prompts(workspace_root)` returns non-empty string when Cursor rules exist. Pipeline receives augmented system prompt.

---

## Step 1: Add `LastTrace` struct

**File:** `src/trace.rs` (new)

**Add:**
```rust
//! Last pipeline run for "Explain" feature. In-memory only.

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct LastTrace {
    pub intent: String,
    pub user_msg: String,
    pub stage: String,           // "compile" | "clippy" | "tests"
    pub stderr: String,
    pub retry_count: u32,
    pub outcome: String,        // "success" | "failed"
    pub chain: Vec<String>,     // "Attempt 1: compile failed" etc.
}
```

**Wire:** `src/lib.rs` — `pub mod trace;`

**Accept:** `cargo build -p kova` succeeds.

---

## Step 2: Thread `LastTrace` through pipeline

**File:** `src/pipeline/mod.rs`

**Change:**
- Add param `last_trace: Option<Arc<Mutex<LastTrace>>>` to `f81` and `run_pipeline`
- At start of `run_pipeline`: init `LastTrace { intent: user_input, user_msg: user_input, .. }`
- On each `cargo_check`/`cargo_clippy`/`cargo_test` failure: update `last_trace` with `stage`, `stderr`, `retry_count`, `chain`
- On success: set `outcome = "success"`
- On final failure (after max retries): set `outcome = "failed"`, write final state
- Before returning: if `last_trace` is Some, write the struct

**Accept:** Pipeline run updates `LastTrace`. No UI yet.

---

## Step 3: Store `LastTrace` in serve `AppState`

**File:** `src/serve.rs`

**Change:**
- Add `last_trace: Arc<Mutex<Option<crate::trace::LastTrace>>>` to `AppState`
- In `api_intent`, when spawning pipeline: pass `state.last_trace.clone()` to `f81`
- Pipeline callback or channel: when pipeline completes, write `LastTrace` into `state.last_trace`

**Design:** Pipeline takes `last_trace: Option<Arc<Mutex<LastTrace>>>`. Inside `run_pipeline` (runs in thread), at every `return`, if `last_trace` is Some: lock, build `LastTrace` from current state, write, drop. Pass `Arc::clone` from serve's AppState into `f81`. GUI can pass `None` for now.

**Change:**
- `f81(..., last_trace: Option<Arc<Mutex<LastTrace>>>)` 
- In `run_pipeline`, before every `return`, if `last_trace` is Some, lock and write.
- `serve.rs` `AppState`: `last_trace: Arc<Mutex<Option<LastTrace>>>`
- `api_intent`: pass `state.last_trace.clone()` to `f81`

**Accept:** FullPipeline run stores `LastTrace` in AppState. Read via debug or new endpoint.

---

## Step 4: Add `POST /api/explain` endpoint

**File:** `src/serve.rs`

**Add:**
```rust
async fn api_explain(State(state): State<AppState>) -> impl IntoResponse {
    let guard = state.last_trace.lock().await;
    match guard.as_ref() {
        Some(t) => Json(serde_json::to_value(t).unwrap_or_default()),
        None => (StatusCode::NOT_FOUND, Json(serde_json::json!({"error":"no trace"}))),
    }
}
```

**Route:** `.route("/api/explain", get(api_explain))` — GET is fine (idempotent).

**Accept:** `curl http://127.0.0.1:3002/api/explain` returns trace JSON or 404.

---

## Step 5: Add explain inference path

**File:** `src/inference.rs` or new `src/academy.rs`

**Add:** `pub async fn explain_trace(trace: &LastTrace, model_path: &Path) -> Result<String, String>`

**Logic:**
- Format trace as readable text (or JSON)
- System prompt: "You are Recursive Academy. Explain this Kova execution trace. What did the user want? What failed? Why? How would a user fix it? Be concise."
- User message: trace content
- Call `f80` with Coder model
- Return response string

**Accept:** `explain_trace(&trace, &model_path)` returns explanation string.

---

## Step 6: Add `POST /api/explain/run` — run model to explain

**File:** `src/serve.rs`

**Add:**
```rust
async fn api_explain_run(State(state): State<AppState>) -> impl IntoResponse {
    let trace = { state.last_trace.lock().await.clone() };
    let trace = match trace {
        Some(t) => t,
        None => return (StatusCode::NOT_FOUND, Json(serde_json::json!({"error":"no trace"}))),
    };
    let model = match crate::f78(crate::ModelRole::Coder) {
        Some(p) => p,
        None => return (StatusCode::SERVICE_UNAVAILABLE, Json(serde_json::json!({"error":"no model"}))),
    };
    match crate::academy::explain_trace(&trace, &model).await {
        Ok(s) => (StatusCode::OK, Json(serde_json::json!({"explanation": s}))),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"error": e}))),
    }
}
```

**Route:** `.route("/api/explain/run", post(api_explain_run))`

**Wire:** `src/lib.rs` — `#[cfg(feature = "inference")] pub mod academy;`  
**Serve:** `api_explain_run` and routes only compiled with `#[cfg(feature = "inference")]`

**Accept:** After a pipeline run, `curl -X POST http://127.0.0.1:3002/api/explain/run` returns `{"explanation":"..."}`.

---

## Step 7: Web GUI — "Explain last run" button

**File:** `kova/assets/app.html`

**Add:**
- Button "Explain last run" below Generate
- On click: `POST /api/explain/run`, display `explanation` in stream area (or a dedicated div)

**Accept:** Click "Explain last run" → explanation appears.

---

## Step 8: DDI — Cap retries at 2, add strategic fresh start

**File:** `src/config.rs`

**Change:** Default `max_fix_retries` to 2 (if currently higher). Add `orchestration_ddi_fresh_start_after` = 2.

**File:** `src/pipeline/mod.rs`

**Change:** After `attempt > max_retries` (or when `attempt == 2` per DDI), instead of calling `fix_and_retry` again:
- **Strategic fresh start:** Re-prompt the **Coder** (not Fix) with original `user_input` + summary: "Previous attempts failed at {stage}. Last error: {first 500 chars of stderr}. Generate a fresh approach."
- Do NOT pass the full stderr chain. Reset `attempt` to 0 for this "fresh" branch.
- Only do this once per pipeline run (flag `fresh_start_done`).

**Simpler DDI change:** Just cap `max_fix_retries` at 2 in config default. Document in config.toml. No fresh start logic yet.

**Accept:** `max_fix_retries` default is 2. Config comment references DDI.

---

## Step 9: GUI — wire pipeline to `last_trace`

**File:** `src/gui.rs`

**Change:** AppState holds `last_trace: Option<LastTrace>`. When pipeline completes (from `pipeline_receiver`), parse final message or add a completion callback. If we add a "trace" field to the broadcast message, we can parse it. Alternatively, pipeline writes to a shared `Arc<Mutex<Option<LastTrace>>>` that GUI holds.

**Simpler:** For serve-only MVP, skip GUI. Web GUI has "Explain last run" which hits the API. GUI (egui) can be Phase 5.

**Accept:** Serve mode has full Explain flow. GUI can be deferred.

---

## Step 10: Stream explain output (optional)

**File:** `src/serve.rs`

**Add:** `POST /api/explain/stream` — same as explain/run but streams via WebSocket or SSE. Reuse existing ws/stream pattern: when "explain" intent, don't run pipeline, run explain and stream tokens.

**Accept:** Explain can stream for long outputs.

---

## Build Order Summary

| Step | Deps | Est. |
|------|------|-----|
| 0. Cursor prompts loader | — | 45 min |
| 1. LastTrace struct | — | 15 min |
| 2. Pipeline writes trace | 1 | 45 min |
| 3. Serve AppState holds trace | 2 | 30 min |
| 4. GET /api/explain | 3 | 15 min |
| 5. explain_trace() | — | 30 min |
| 6. POST /api/explain/run | 3, 5 | 20 min |
| 7. Web GUI button | 6 | 20 min |
| 8. DDI cap retries | — | 15 min |
| 9. GUI trace (defer) | — | — |
| 10. Stream explain (optional) | 6 | 30 min |

**MVP (Steps 0–8):** ~4–5 hours.

---

## File Checklist

| File | Action |
|------|--------|
| `src/cursor_prompts.rs` | Create — load Cursor rules, skills, AGENTS, protocol_map, compression_map |
| `src/trace.rs` | Create |
| `src/academy.rs` | Create |
| `src/lib.rs` | Add mod cursor_prompts, mod trace, mod academy |
| `src/pipeline/mod.rs` | Add last_trace param, inject cursor_prompts into system, write at exit |
| `src/pipeline/fix_loop.rs` | Inject cursor_prompts into fix system prompt |
| `src/serve.rs` | AppState.last_trace, routes, handlers |
| `src/config.rs` | Default max_fix_retries = 2, [cursor] prompts_enabled |
| `assets/app.html` | Explain button |

---

## Test Commands

```bash
# Build
cargo build -p kova --features "serve,inference"

# Run serve
cargo run -p kova --bin kova --features "serve,inference" -- serve

# Trigger pipeline (web GUI or curl)
curl -X POST http://127.0.0.1:3002/api/intent -H "Content-Type: application/json" \
  -d '{"s0":{"FullPipeline":null},"s1":"add broken code","s2":[]}'

# Get trace
curl http://127.0.0.1:3002/api/explain

# Run explain
curl -X POST http://127.0.0.1:3002/api/explain/run
```
