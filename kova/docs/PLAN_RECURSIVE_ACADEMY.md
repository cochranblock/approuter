# Plan: Recursive Academy — Trace-Driven, Failure-Fed Curriculum

**Approach:** Kova generates its own training content from real execution traces and failure modes. Grounded in actual behavior, not generic docs.

**Audience:** Engineers using Kova. "Why did it do that?" and "How do I fix this?" answered from real data.

**Date:** 2026-03-04

**User stories:** See `USER_STORY_RECURSIVE_ACADEMY.md` — validate before build.

---

## Why This Option

| Alternative | Downside |
|-------------|----------|
| One-shot curriculum generation | Hallucination, generic content, no grounding |
| Static docs | Drift from actual Kova behavior |
| Human-only curriculum | Slow, doesn't scale with Kova changes |

**Best option:** Trace-driven content + failure-fed curriculum + RAG. Real traces and failures become the curriculum. Models explain what actually happened.

---

## Phase -1: Cursor Prompts as Training Data

**Goal:** Inject all Cursor prompts into Kova model context so Coder, Fix, Router, and Academy use workspace conventions.

**Sources:**
- `~/.cursor/rules/*.mdc` — workspace rules (blocking, augment-not-intent, tokenization, etc.)
- `{workspace}/.cursor/rules/*.mdc` — project rules
- `~/.cursor/skills-cursor/*/SKILL.md`, `~/.cursor/skills/*/SKILL.md`
- `AGENTS.md`, `protocol_map.md`, `compression_map.md`

**Injection points:** Coder system prompt, Fix system prompt, Router (if relevant), Academy explain prompt.

**Config:** `[cursor] prompts_enabled = true`

**Exit:** `load_cursor_prompts(workspace_root)` returns concatenated prompts. Pipeline and fix loop inject them.

---

## Phase 0: Trace Instrumentation

**Goal:** Capture intent → router → pipeline → output as structured traces.

- [ ] Add trace events to pipeline (f81): `intent_received`, `router_classified`, `coder_started`, `cargo_check`, `cargo_check_failed`, `fix_retry`, `output_ready`
- [ ] Trace format: JSONL to `~/.kova/traces/YYYY-MM-DD/` (one file per session or rolling)
- [ ] Fields: `ts`, `event`, `intent`, `model_role`, `stderr`, `stdout`, `retry_count`, `outcome`
- [ ] Optional: `--trace` flag or `config.toml` `trace_enabled = true`

**Exit:** Running a pipeline writes trace events. No UI yet.

---

## Phase 1: Failure Extraction

**Goal:** From traces, extract "failure modes" — patterns that led to retries or errors.

- [ ] `kova trace failures` — scan trace dirs, find events with `outcome = failed` or `retry_count > 0`
- [ ] Group by pattern: `cargo_check_failed` + stderr snippet hash, `router_needs_clarification`, etc.
- [ ] Output: `~/.kova/academy/failures.json` — `{ "pattern": "...", "count": N, "example_stderr": "...", "trace_ids": [...] }`
- [ ] Human review optional: `failures.json` is the seed for curriculum

**Exit:** `kova trace failures` produces a failure catalog from real runs.

---

## Phase 2: Trace-to-Explanation Pipeline

**Goal:** Given a trace (or trace segment), model generates "what happened" and "how to fix."

- [ ] New intent or subcommand: `generate_academy_module` or `kova academy explain <trace_id>`
- [ ] Input: trace JSON (or path)
- [ ] System prompt: "You are Recursive Academy. Explain this Kova execution trace. What did the user want? What did the router decide? What failed? How would a user fix it?"
- [ ] Output: Markdown doc (or structured JSON for later rendering)
- [ ] Use Coder model (or dedicated "explainer" if we add one)

**Exit:** `kova academy explain <trace_id>` produces a human-readable explanation of that run.

---

## Phase 3: Failure-Fed Curriculum Generator

**Goal:** From failures.json, generate curriculum modules automatically.

- [ ] `kova academy generate` — reads `failures.json`, for each pattern:
  - Load example trace
  - Call trace-to-explanation (Phase 2)
  - Add "Exercise: Reproduce this failure, then fix it" section
  - Write to `~/.kova/academy/modules/<pattern_slug>.md`
- [ ] Module structure: `# Title`, `## What Happened`, `## Why`, `## How to Fix`, `## Exercise`
- [ ] Optional: Run the exercise through Kova, verify it passes after "fix" — if not, add to failures for next iteration

**Exit:** `kova academy generate` produces a set of markdown modules from real failures.

---

## Phase 4: RAG Over Traces + Academy

**Goal:** When user asks "why did Kova do X?" or "how do I fix Y?", retrieve relevant traces and academy modules.

- [ ] Embedding: use a small local embedder (e.g. `fastembed`, `sentence-transformers` via Python bridge, or Kalosm if it has embeddings) for:
  - Trace event summaries (intent + outcome + stderr snippet)
  - Academy module titles + key sections
- [ ] Store: `~/.kova/academy/index/` — vector store (sled? or simple JSON + cosine sim for MVP)
- [ ] Query: user question → embed → top-k traces + top-k modules → inject into prompt
- [ ] New intent or chat flow: "explain" / "why" / "how fix" → RAG retrieval → model answers with citations

**Exit:** Asking "why did cargo check fail?" retrieves relevant traces and modules, model answers with context.

---

## Phase 5: Recursive Academy UI

**Goal:** Surface academy in GUI and serve mode.

- [ ] GUI: "Academy" tab or panel — list modules, click to view; "Explain last run" button
- [ ] Serve: `GET /academy/modules`, `GET /academy/modules/:slug`, `POST /academy/explain` (body: trace or question)
- [ ] Web GUI: add Academy section — browse modules, paste trace ID or stderr for explanation

**Exit:** Users can browse and query Recursive Academy from GUI and API.

---

## Phase 6: Feedback Loop (Optional)

**Goal:** User feedback improves the curriculum.

- [ ] "Was this helpful?" thumbs up/down on explanations
- [ ] Store in `~/.kova/academy/feedback.json`
- [ ] `kova academy generate` can weight patterns by feedback (prioritize low-rated explanations for regeneration)
- [ ] Future: fine-tune or prompt-tune on high-rated (intent, trace, explanation) triples

**Exit:** Feedback influences which modules get regenerated.

---

## Dependencies

| Phase | Deps |
|------|-----|
| 0 | None |
| 1 | 0 |
| 2 | 0 |
| 3 | 1, 2 |
| 4 | 0, 1, 2, 3 (or 2 only for MVP) |
| 5 | 2, 3, 4 |
| 6 | 5 |

---

## MVP Scope (Fastest Path)

**Phases 0 + 2 only:** Trace instrumentation + trace-to-explanation. No RAG, no auto-generation.

- Run pipeline → traces written
- `kova academy explain <trace_id>` → model explains that run

**Time:** ~1 day. Delivers immediate value: "what just happened?"

---

## Full Scope

**Phases 0–5.** Trace → failures → curriculum → RAG → UI.

**Time:** ~1–2 weeks. Self-improving academy grounded in real Kova behavior.

---

## Research-Backed Additions

See `RESEARCH_RECURSIVE_ACADEMY.md` for sources.

- **DDI (Debugging Decay Index):** Fix loop loses 60–80% effectiveness by attempt 2–3. Cap retries; add *strategic fresh start* — after N failures, re-prompt from scratch with summary, not full stderr chain.
- **Trace as optimization:** Structure traces for "back-propagation" into prompts; not just logging.
- **ExIt autocurriculum:** One module per failure pattern; chain at inference. Don't generate full curriculum at once.
- **Kalosm RAG:** DocumentTable + SurrealDB for trace/module retrieval when ready.

---

## Out of Scope (For Now)

- Fine-tuning on traces (needs more infra)
- Multi-model "teacher" role split (adds complexity; single Coder can do Phase 2)
- Public Recursive Academy site (local-first first)
