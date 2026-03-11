# Plan: All Three — Overlap Analysis & Prioritization

**Date:** 2026-03-11

---

## The Three Tasks

| # | Task | Scope |
|---|------|-------|
| 1 | **FullPipeline fallback** | When inference unavailable, run plan path (f14 → f15) instead of 503 |
| 2 | **Verify execution paths** | Run PLAN_GAP_ANALYSIS sims (Sim 1–10) |
| 3 | **PLAN_ELICITATION** | Elicitor, Router choices, GUI restatement, Serve API clarification |

---

## Overlap Analysis

### Task 1 + Task 2

- **Fallback** changes `api_intent` and `api_backlog_run` — both are exercised in **Sim 3** (serve) and indirectly in backlog flow.
- **Verification** will catch any regressions from the fallback.
- **Order:** Implement fallback first, then verify (fallback becomes part of what we verify).

### Task 1 + PLAN_BUILD_KOVA

- Phase 6: "Fallback to keyword when offline or no model" — FullPipeline fallback is exactly this.
- Implementing fallback completes that checkbox.

### Task 2 + PLAN_GAP_ANALYSIS

- **Same activity** — Running Sim 1–10 *is* the verification.
- PLAN_GAP_ANALYSIS already documents Expected/Observed; we run the commands and confirm.

### Task 3 + PLAN_ELICITATION

- **Partial overlap** — Elicitor module (format_question, parse_reply, build_restatement) is **already implemented**.
- Router uses format_question and has canned choices.
- GUI uses parse_reply for Cancel/Choice.
- **Gaps remaining:**
  - GUI: Restatement step before coder (build_restatement, confirm before invoke)
  - Serve: Router in API path, needs_clarification response, follow-up flow
  - app.html: Clarification UI, choice buttons, confirm/cancel

---

## Prioritization

| Priority | Task | Rationale |
|----------|------|-----------|
| **P0** | FullPipeline fallback | Small, self-contained. Enables use without models. Unblocks verification. |
| **P1** | Verify execution paths | Validates all flows including fallback. PLAN_GAP_ANALYSIS protocol. |
| **P2** | PLAN_ELICITATION gaps | Restatement, serve API, app.html. Elicitor core already done. |

---

## Implementation Order

1. **FullPipeline fallback** — serve.rs: api_intent, api_backlog_run. When FullPipeline + no coder/fix, run plan path instead of 503.
2. **Verification** — Run Sim 1–10 from PLAN_GAP_ANALYSIS. Document any new gaps.
3. **PLAN_ELICITATION** — GUI restatement → Serve API clarification → app.html UI.
