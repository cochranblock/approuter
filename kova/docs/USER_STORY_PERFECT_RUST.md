# User Story Analysis: Perfect Rust Code Orchestration

**Tool:** Kova — local AI orchestration for Rust, with human elicitation at its core.

**Date:** 2026-03-10

---

## 1. Vision

Real users say **"fix the bug"** or **"add the thing"** — vague, underspecified, omitting file, project, and constraints. Kova must **tease information out of humans** before acting:

1. **Elicit** — Ask when ambiguous. Offer choices (a/b/c). Restate before generating.
2. **Confirm** — "I'll add X to Y. Proceed?" — never assume.
3. **Understand** — Parse intent only after sufficient context is gathered.
4. **Plan & Execute** — Generate, validate, refine, output.

**"Perfect"** = compiles, tests pass, clippy clean, matches project style — but only after the system has **elicited** enough from the user to act correctly.

---

## 2. User Personas

### Primary: Human Who Needs Something Done

- **Goals:** Get help without writing structured prompts. Say what they want in plain language.
- **Frustrations:** Systems guess wrong, generate in the wrong file, or assume context they don't have.
- **Context:** Often omits file path, project, constraints, or even what "fix" or "add" means.
- **Success:** "I said 'fix the bug' and Kova asked which file, then confirmed before changing anything."

### Secondary: Power User (Senior Rust Engineer)

- **Goals:** Ship correct code fast. Provide precise prompts when they want to.
- **Context:** Can be vague when tired or in a hurry; appreciates confirmation before large changes.
- **Success:** "Even when I'm sloppy, Kova asks the right questions and doesn't guess."

---

## 3. Design Principles: Human Elicitation

| Principle | Meaning |
|-----------|---------|
| **Ask, don't assume** | When intent is ambiguous, ask. Never infer file, project, or scope without confirmation. |
| **Choices over open-ended** | Prefer "Which file? (a) compute.rs (b) plan.rs (c) other" over "Which file?" |
| **Confirm before generate** | Restate: "I'll add X to Y. Proceed?" — user says y/n before code is generated. |

---

## 4. Epics & User Stories

### Epic E1: Tease Intent Before Acting

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| E1.1 | As a user, I want the system to ask when my request is ambiguous. | Router detects ambiguity → returns `needs_clarification` + suggested question. No generation until clarified. |
| E1.2 | As a user, I want choices (a/b/c) instead of open-ended questions. | Elicitor offers discrete options: "Which project? (a) kova (b) rogue-repo (c) cochranblock)". |
| E1.3 | As a user, I want the system to restate before generating. | Before code gen: "I'll add exponential backoff to the retry in compute.rs. Proceed?" User confirms (y/n). |
| E1.4 | As a user, I want to cancel at any clarification step. | "Cancel" or "n" aborts the flow. No partial generation. |

### Epic E2: Elicitation UX

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| E2.1 | As a user, I want inline clarification messages. | Questions appear in the chat flow, not a separate modal. Context preserved. |
| E2.2 | As a user, I want short replies (y/n or pick). | Accept "y", "yes", "a", "1", "compute.rs" — minimal typing. |
| E2.3 | As a user, I want to cancel easily. | "cancel", "n", "no", "stop" — clear exit. |
| E2.4 | As a user, I want to see what the system understood. | Restatement visible: "You want: add retry to compute.rs in kova. Proceed?" |

### Epic E3: Code Generation Pipeline (Post-Elicitation)

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-2.1 | As a user, I want generated code to include project conventions. | Coder receives system.md + persona.md + elicited context. |
| US-2.2 | As a user, I want code generated into the right file. | Elicited file path + region passed to model. |
| US-2.3 | As a user, I want validation before I see output. | generate → cargo check → fix loop (max 2) → clippy → test. |
| US-2.4 | As a user, I want to reject and retry manually. | "No, use tokio::time::sleep" → re-run with that constraint. |

### Epic E4: Router Returns needs_clarification

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| E4.1 | As a user, I want the router to detect ambiguity. | Router outputs `needs_clarification` when file, project, or action is unclear. |
| E4.2 | As a user, I want a suggested question from the router. | Router returns `suggested_question` + optional `choices: ["a", "b", "c"]`. |
| E4.3 | As a user, I want the elicitor to use router suggestions. | Elicitor module consumes router output, formats question, presents choices. |
| E4.4 | As a user, I want re-routing after clarification. | User reply → Router re-invoked with enriched context → proceed or ask again. |

### Epic E5: Model Orchestration & Context

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-3.x | Router resident, specialists on demand. | Same as before. |
| US-4.x | Project awareness, file context, compression_map. | Same as before. Elicitation fills gaps. |

### Epic E6: Output & Integration

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-6.x | Diff, apply, copy, streaming. | Same as before. |

---

## 5. Elicitation Flow Diagram

```
User: "fix the bug"  (vague)
    │
    ▼
[Router] → needs_clarification
    │     suggested_question: "Which file has the bug?"
    │     choices: ["compute.rs", "plan.rs", "other"]
    │
    ▼
[Elicitor] → Inline: "Which file has the bug? (a) compute.rs (b) plan.rs (c) other"
    │
    ▼
User: "a"  or  "compute.rs"
    │
    ▼
[Router] (re-invoke with enriched context) → fix
    │
    ▼
[Elicitor] → Restate: "I'll fix the bug in compute.rs. Proceed? (y/n)"
    │
    ▼
User: "y"
    │
    ▼
[Generate] → Fix model + compute.rs + context → ...
```

---

## 6. Definition of "Perfect" Rust Code

| Criterion | Check |
|-----------|-------|
| Compiles | `cargo check` passes |
| Tests pass | `cargo test` passes |
| Clippy clean | `cargo clippy` no warnings (or configurable) |
| Idiomatic | Matches Rust API guidelines |
| Project style | compression_map, protocols |
| Elicited first | No generation without sufficient user-confirmed context |

---

## 7. Technical Flows

### Flow A: Keyword Intent (Existing)

Unchanged. No elicitation for f62 matches.

### Flow B: Elicitation → Code Gen

```
User: "add the thing"
  → Router: needs_clarification (what? where? which project?)
  → Elicitor: "What do you want to add? (a) retry helper (b) test (c) other"
  → User: "a"
  → Elicitor: "Where? (a) compute.rs (b) plan.rs (c) new file"
  → User: "a"
  → Elicitor: "I'll add a retry helper to compute.rs in kova. Proceed? (y/n)"
  → User: "y"
  → Router: code_gen
  → Generate → Validate → Output
```

### Flow C: Serve API (Web Client)

Same flow. API returns `needs_clarification` with `suggested_question` and `choices`. Client renders inline; user replies; next request includes prior context.

---

## 8. Success Metrics

| Metric | Target |
|--------|--------|
| Clarification cycles to actionable intent | ≤ 3 for typical vague requests |
| User can cancel at any step | 100% |
| Confirmation before every code gen | 100% |
| First-token latency (router) | < 2s |

---

## 9. Out of Scope (For Now)

- Remote/cloud models
- Multi-user
- Fine-tuning on user data
- Full IDE integration (LSP)
