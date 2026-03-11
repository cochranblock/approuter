# User Story Analysis: Recursive Academy & Cursor Prompts

**Tool:** Kova — local AI orchestration. This doc defines the "pros of the pros" for Recursive Academy, Cursor prompts, trace/explain, and DDI.

**Date:** 2026-03-04

---

## 1. Vision

A senior engineer uses Kova. When something fails or feels opaque, they get an explanation grounded in what actually happened — not generic docs. The models that power Kova use the same conventions the engineer already maintains in Cursor (rules, skills, protocol_map). The fix loop stops before it degrades. The system teaches itself from real failures.

**"Pros of the pros"** = highest-leverage user value: explainability, convention alignment, and self-improvement from traces.

---

## 2. User Personas

### Primary: Senior Rust Engineer (Kova User)

- **Goals:** Understand why Kova did X. Fix failures faster. Trust that generated code matches workspace conventions.
- **Frustrations:** "What just happened?" after a pipeline run. Models ignore tokenization, blocking rules, project structure. Fix loop spirals into worse code.
- **Context:** Uses Cursor with rules, AGENTS.md, compression_map. Wants Kova to respect them.
- **Success:** "I clicked Explain and got a clear answer. The code it generated used fN naming. When it failed, it stopped at 2 retries and offered a fresh approach."

### Secondary: Architect / Tech Lead

- **Goals:** Single source of truth for conventions. Cursor rules = Kova context. No duplicate maintenance.
- **Context:** Maintains .cursor/rules, protocol_map, compression_map. Wants models to ingest them automatically.
- **Success:** "I added a rule. Kova picked it up. No copy-paste into system.md."

### Tertiary: Onboarding Engineer

- **Goals:** Learn how Kova works. Understand failure modes. Self-serve answers.
- **Context:** New to workspace. Needs "why did cargo check fail?" answered from real examples.
- **Success:** "Recursive Academy showed me a trace like mine. I fixed it myself."

---

## 3. Epics & User Stories

### Epic 7: Cursor Prompts as Training Data

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-7.1 | As a senior engineer, I want Kova to use my Cursor rules so generated code matches workspace conventions. | Coder receives ~/.cursor/rules/*.mdc + project .cursor/rules. Output reflects tokenization, blocking, anti-patterns. |
| US-7.2 | As an architect, I want a single source of truth for conventions. | Add/change .cursor/rules → Kova picks up on next run. No edit to ~/.kova/prompts required. |
| US-7.3 | As a senior engineer, I want protocol_map and compression_map in model context. | Coder and Fix receive docs/protocol_map.md, docs/compression_map.md (or workspace equivalents). fN/tN/sN applied. |
| US-7.4 | As a senior engineer, I want to disable Cursor prompts when not needed. | config.toml `[cursor] prompts_enabled = false` → no injection. Pipeline unchanged. |

### Epic 8: Trace & Explain

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-8.1 | As a senior engineer, I want to understand what just happened after a pipeline run. | "Explain last run" → model receives trace (intent, stage, stderr, retries, outcome) → returns plain-English explanation. |
| US-8.2 | As a senior engineer, I want the explanation to reference my conventions. | Academy system prompt includes Cursor prompts. Output cites "per tokenization rule…" or "compression_map says…" when relevant. |
| US-8.3 | As a senior engineer, I want Explain available from the web GUI. | Button "Explain last run" → POST /api/explain/run → explanation in stream area. |
| US-8.4 | As a senior engineer, I want the trace even when the run failed. | LastTrace written on every pipeline exit (success or failure). GET /api/explain returns it. |
| US-8.5 | As a senior engineer, I want a clear message when there's no trace. | "Explain last run" with no prior run → "No trace. Run a pipeline first." |

### Epic 9: DDI-Aware Fix Loop

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-9.1 | As a senior engineer, I want the fix loop to stop before it degrades. | max_fix_retries default = 2. Config comment references DDI. |
| US-9.2 | As a senior engineer, I want a fresh approach when retries fail. | (Future) After 2 retries: strategic fresh start — re-prompt Coder with summary, not full stderr chain. |
| US-9.3 | As a senior engineer, I want to see why we cap retries. | Recursive Academy explains: "Fix loop loses effectiveness after 2–3 attempts (DDI). We stop to avoid worse output." |

### Epic 10: Recursive Academy (Full)

| ID | Story | Acceptance Criteria |
|----|-------|---------------------|
| US-10.1 | As an onboarding engineer, I want to browse failure patterns. | Academy modules: one per failure pattern. Structure: What Happened, Why, How to Fix, Exercise. |
| US-10.2 | As a senior engineer, I want to ask "why did Kova do X?" and get a grounded answer. | RAG over traces + modules. Query → top-k retrieval → model answers with citations. |
| US-10.3 | As an architect, I want the curriculum to come from real failures. | kova academy generate → reads failures.json → produces modules from traces. No hallucinated content. |
| US-10.4 | As a senior engineer, I want to rate explanations. | Thumbs up/down. Feedback stored. Future: low-rated → regenerate. |

---

## 4. Definition of "Right"

| Criterion | Check |
|-----------|-------|
| Cursor prompts loaded | load_cursor_prompts(workspace) returns non-empty when rules exist |
| Conventions applied | Generated code uses fN/tN/sN when compression_map present |
| Explain works | "Explain last run" returns coherent explanation of trace |
| Trace complete | LastTrace has intent, stage, stderr, retries, outcome |
| DDI respected | max_fix_retries ≤ 2 by default |
| No regression | Existing pipeline, GUI, serve unchanged when features disabled |

---

## 5. Success Metrics

| Metric | Target |
|--------|--------|
| Cursor prompt injection | 100% of Coder/Fix calls receive prompts when enabled |
| Explain latency | < 15s for typical trace (model inference) |
| User satisfaction | "I understood what happened" (qualitative) |
| Convention adherence | Generated code passes tokenization checks when rules present |
| Fix loop effectiveness | No degradation beyond attempt 2 (DDI-aligned) |

---

## 6. Edge Cases & Failure Modes

| Case | Handling |
|------|----------|
| No Cursor rules | load_cursor_prompts returns empty. Pipeline runs with system+persona only. |
| Cursor prompts too long | Truncate or summarize. Config: max_cursor_prompts_chars. |
| Explain with no trace | Return 404 / "No trace. Run a pipeline first." |
| Explain with no model | Return 503 / "No model. Run: kova model install" |
| Workspace root ambiguous | Use KOVA_PROJECT or default_project. Document in config. |
| AGENTS.md missing | Skip. No error. |
| protocol_map in different path | Config: cursor.docs_path override. |

---

## 7. Phased Implementation (User Story → Phase)

| Phase | Stories | Priority |
|-------|---------|----------|
| Phase -1: Cursor prompts | US-7.1, US-7.2, US-7.3, US-7.4 | P0 — foundation |
| Phase 0: Trace + Explain (MVP) | US-8.1, US-8.2, US-8.3, US-8.4, US-8.5 | P0 — immediate value |
| Phase 0b: DDI | US-9.1, US-9.3 | P0 — research-backed |
| Phase 1: Strategic fresh start | US-9.2 | P1 |
| Phase 2+: Full Academy | US-10.1–US-10.4 | P2 |

---

## 8. Anti-Goals (What We're NOT Doing)

- Fine-tuning on Cursor prompts (in-context only)
- Replacing Cursor (Kova is complementary)
- Public Recursive Academy site (local-first)
- Parsing agent transcripts as training data (privacy, noise)
- Adding Cursor prompts to Router by default (optional; augment-not-intent is small)

---

## 9. Validation Before Build

Before implementing, confirm:

- [ ] Personas match actual users (senior engineer, architect)
- [ ] US-7.1–US-7.4 cover Cursor prompt injection fully
- [ ] US-8.1–US-8.5 cover trace + explain MVP
- [ ] US-9.1–US-9.3 align with DDI research
- [ ] Edge cases are handled
- [ ] No overlap with USER_STORY_PERFECT_RUST (Epics 1–6)
- [ ] "Pros of the pros" = highest leverage: Cursor prompts + Explain + DDI cap

---

## 10. One-Line Summary

**Cursor prompts** = models use your rules. **Explain** = you understand what happened. **DDI** = fix loop stops before it gets worse. **Recursive Academy** = curriculum from real failures.
