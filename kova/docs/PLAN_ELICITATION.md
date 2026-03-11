# Implementation Plan: Human Elicitation

**Goal:** Tease information out of vague users before acting. Ask when ambiguous, offer choices, confirm before generate.

**Ref:** [USER_STORY_PERFECT_RUST.md](USER_STORY_PERFECT_RUST.md) — Epics E1, E2, E4.

---

## 1. Elicitor Module

**Location:** `src/elicitor.rs` (new)

**Responsibilities:**
- Format questions with choices: `"Which file? (a) compute.rs (b) plan.rs (c) other"`
- Parse short replies: `y`/`yes`, `n`/`no`/`cancel`, `a`/`1`/`compute.rs`
- Build restatement: `"I'll add X to Y. Proceed? (y/n)"`
- Detect cancel: `cancel`, `n`, `no`, `stop` → abort flow

**Types:**
```rust
pub struct ElicitorQuestion {
    pub question: String,
    pub choices: Option<Vec<String>>,  // ["compute.rs", "plan.rs", "other"]
}

pub enum ElicitorReply {
    Choice(usize),      // user picked (a) → 0
    Freeform(String),   // user typed "compute.rs"
    Confirm(bool),      // y/n
    Cancel,
}
```

**Functions:**
- `format_question(question: &str, choices: Option<&[String]>) -> String` — "(a) X (b) Y (c) Z"
- `parse_reply(input: &str, num_choices: Option<usize>) -> ElicitorReply`
- `build_restatement(action: &str, target: &str) -> String` — "I'll add X to Y. Proceed? (y/n)"

**Integration:** Router's `NeedsClarification` + `clarification_question()` feed into Elicitor. Router prompt extended to optionally return `choices` (Phase 2).

---

## 2. Router: needs_clarification Variant

**Current state:** `RouterResult::NeedsClarification(Option<String>)` exists. `clarification_question()` provides canned fallback.

**Changes:**
1. **Extend RouterOutput** (when `router_structured`):
   - Add `choices: Option<Vec<String>>` to JSON schema
   - Parse `{"classification": "needs_clarification", "question": "Which file?", "choices": ["compute.rs", "plan.rs"]}`

2. **Extend RouterResult:**
   - `NeedsClarification { question: Option<String>, choices: Option<Vec<String>> }`
   - Or keep `NeedsClarification(Option<String>)` and add `RouterResult::clarification_choices()` returning `Option<Vec<String>>` from a separate field

3. **Update CLASSIFY_PROMPT:**
   - "If needs_clarification, include 'question' and optionally 'choices' (array of 2–5 options)."

4. **Canned fallbacks** in `clarification_question()`:
   - For "fix"/"bug": choices `["compute.rs", "plan.rs", "lib.rs", "other"]`
   - For "add"/"implement": choices from project's `.rs` files or `["lib.rs", "main.rs", "other"]`

---

## 3. GUI: Inline Clarification & Short-Reply Handling

**Current state:** `clarification_pending`, question shown inline, next user input concatenated with original and re-routed.

**Changes:**
1. **Inline clarification messages:**
   - Already inline. Ensure message bubble shows question + choices when available.
   - Use Elicitor's `format_question()` for display.

2. **Short-reply handling:**
   - Before sending to router, parse with `Elicitor::parse_reply()`.
   - If `Cancel` → abort, clear `clarification_pending`, show "Cancelled."
   - If `Choice(n)` → map to choice string, pass to router as enriched context.
   - If `Confirm(false)` → abort.
   - If `Confirm(true)` → treat as proceed (only after restatement step).

3. **Restatement step (new):**
   - When router returns `CodeGen`/`Fix`/etc., **before** invoking coder:
   - Elicitor builds restatement: "I'll add retry helper to compute.rs in kova. Proceed? (y/n)"
   - Show in chat, set `restatement_pending = true`.
   - Next user input: if `Confirm(true)` → run coder; if `Confirm(false)` or `Cancel` → abort.

4. **Cancel at any step:**
   - `cancel`, `n`, `no`, `stop` → clear pending state, no generation.

---

## 4. Serve API: Same Flow for Web Client

**Current state:** `api_intent` accepts `IntentRequest`, runs FullPipeline or stores for WebSocket. No router, no clarification.

**Changes:**
1. **New endpoint or extend IntentResponse:**
   - Option A: `POST /api/route` — raw user message → Router → returns `{ classification, needs_clarification?, suggested_question?, choices? }`
   - Option B: Extend `POST /api/intent` to accept freeform text, run router, return same.

2. **IntentResponse schema:**
   ```json
   {
     "accepted": true,
     "intent_name": "code_gen",
     "needs_clarification": true,
     "suggested_question": "Which file?",
     "choices": ["compute.rs", "plan.rs", "lib.rs"]
   }
   ```

3. **Clarification flow:**
   - Client sends message → API runs router.
   - If `needs_clarification`: return `needs_clarification`, `suggested_question`, `choices`. Client renders inline, user replies.
   - Client sends follow-up with `prior_message` + `reply` in body.
   - API re-routes with enriched context: `"{} in {}"` or `"{} {}"`.

4. **Restatement:**
   - When router returns actionable (code_gen, fix, etc.), API can optionally return `restatement: "I'll add X to Y. Proceed?"`.
   - Client shows, user confirms. Next request includes `confirmed: true`.

5. **WebSocket:** If used for streaming, same flow: server can send `{"type": "needs_clarification", "question": "...", "choices": [...]}` before streaming tokens.

---

## 5. Implementation Order

| Step | Task | Depends |
|------|------|---------|
| 1 | Create `src/elicitor.rs`: `format_question`, `parse_reply`, `build_restatement`, `ElicitorReply` | — |
| 2 | Extend Router: `choices` in output, canned choices in `clarification_question()` | — |
| 3 | GUI: Use Elicitor for display, parse short replies, handle Cancel | 1 |
| 4 | GUI: Add restatement step before coder invocation | 1, 3 |
| 5 | Serve: Add route/response for `needs_clarification` | 2 |
| 6 | Serve: Clarification follow-up with prior context | 5 |
| 7 | Web client (app.html): Render clarification, short-reply UI | 5 |

---

## 6. File Touch List

| File | Changes |
|------|---------|
| `src/elicitor.rs` | New. Elicitor module. |
| `src/lib.rs` | `pub mod elicitor;` |
| `src/router.rs` | `choices` in RouterOutput/RouterResult, prompt update |
| `src/gui.rs` | Elicitor integration, restatement step, cancel handling |
| `src/serve.rs` | Router in API path, `needs_clarification` response, follow-up |
| `assets/app.html` | Clarification UI, choice buttons, confirm/cancel |

---

## 7. Testing

- Unit: `Elicitor::parse_reply("y")` → `Confirm(true)`, `"a"` → `Choice(0)`, `"cancel"` → `Cancel`
- Unit: `format_question` with/without choices
- Integration: "fix the bug" → needs_clarification → "compute.rs" → restatement → "y" → code gen
