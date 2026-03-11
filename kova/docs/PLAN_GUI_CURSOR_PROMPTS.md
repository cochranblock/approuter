# Plan: GUI Cursor Prompts Injection

**Goal:** Align GUI with serve — inject Cursor rules (baked + external) into all Coder model paths.

**Source:** TRIPLE_SIMS_KOVA.md Sim 2 Gap #1, Sim 3.

**Date:** 2026-03-04

---

## Problem

| Path | Current | Serve equivalent |
|------|---------|------------------|
| CodeGen (RouterResult::CodeGen) | `system + persona` | `system + persona + cursor` |
| use_coder (refactor, explain, fix, custom) | `system + persona` | `system + persona + cursor` |
| Direct send (no router) | `system + persona` | `system + persona + cursor` |

GUI omits `load_cursor_prompts(&project)` in all three. Serve injects it.

---

## Steps

### Step 1: Add helper in gui.rs

**Location:** Top of `gui.rs` or as method on app state.

**Add:**
```rust
fn build_system_prompt(&self) -> String {
    let project = crate::default_project();
    let cursor = crate::cursor_prompts::load_cursor_prompts(&project);
    if cursor.is_empty() {
        format!("{}\n\n{}", self.system_prompt, self.persona)
    } else {
        format!("{}\n\n{}\n\n--- Cursor rules ---\n{}", self.system_prompt, self.persona, cursor)
    }
}
```

**Alternative:** Inline at each call site (3 places). Helper reduces duplication.

---

### Step 2: CodeGen path (RouterResult::CodeGen)

**File:** `src/gui.rs` ~line 173

**Change:**
```rust
// Before
let system = format!("{}\n\n{}", self.system_prompt, self.persona);

// After
let system = self.build_system_prompt();
```

**Accept:** CodeGen pipeline receives cursor_prompts.

---

### Step 3: use_coder path (refactor, explain, fix, custom)

**File:** `src/gui.rs` ~line 201

**Change:**
```rust
// Before
let system = format!("{}\n\n{}", self.system_prompt, self.persona);

// After
let system = self.build_system_prompt();
```

**Accept:** Streamed Coder responses (non-pipeline) receive cursor_prompts.

---

### Step 4: Direct send path (no router)

**File:** `src/gui.rs` ~line 486

**Change:**
```rust
// Before
let system = format!("{}\n\n{}", self.system_prompt, self.persona);

// After
let system = self.build_system_prompt();
```

**Accept:** Direct-to-model messages receive cursor_prompts.

---

### Step 5: Wire last_trace for GUI (optional)

**Current:** GUI passes `None` to `f81` for last_trace. Explain-last-run is serve-only.

**Defer:** Per BUILD_STEPS Step 9 — "For serve-only MVP, skip GUI." Web GUI has Explain. egui can be Phase 5.

**If doing now:** Add `last_trace: Arc<Mutex<Option<LastTrace>>>` to GUI state, pass to f81. Requires gui.rs to hold async runtime or sync mutex. More involved.

---

## Build Order

| Step | Deps | Est. |
|------|------|------|
| 1. Add build_system_prompt() | — | 5 min |
| 2. CodeGen path | 1 | 2 min |
| 3. use_coder path | 1 | 2 min |
| 4. Direct send path | 1 | 2 min |
| 5. last_trace (defer) | — | — |

**Total:** ~15 min for Steps 1–4.

---

## Verification

1. `cargo build -p kova --features "gui,inference"` — succeeds
2. `cargo test -p kova` — all pass
3. Manual: `kova gui`, send "add a function that returns 42", observe CodeGen. Output should reflect conventions if model follows them.
4. TRIPLE_SIMS_KOVA.md — re-run Sim 1, 2, 3; Gap #1 should be closed.

---

## File Checklist

| File | Change |
|------|--------|
| `src/gui.rs` | Add `build_system_prompt()`, replace 3× `format!(...)` with `self.build_system_prompt()` |
