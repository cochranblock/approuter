# Research: Recursive Academy & Trace-Driven Self-Improvement

**Bleeding edge only.** No years in search queries.

---

## 1. Trace as Optimization Signal (Microsoft Trace)

**Source:** Trace is the Next AutoDiff, NeurIPS; Microsoft Trace framework

**Finding:** Execution traces are to workflow optimization what gradients are to neural networks. Traces capture rich feedback (console output, compiler errors, user responses, rewards) that LLMs can interpret to improve parameters.

**Implications for Kova:**
- Pipeline traces (intent → router → coder → cargo_check → fix) are optimization signals
- Don't just log; structure traces so they can be "back-propagated" into prompt/system improvements
- OptoPrime: LLM-based optimizer that learns from traces; could adapt for Rust/Kova (Python bridge or port concepts)

---

## 2. Debugging Decay Index (DDI)

**Source:** Debugging Decay Index paper; Scientific Reports; Emergent Mind

**Finding:** Code LLMs lose 60–80% of debugging effectiveness within 2–3 attempts. By attempts 4–5, performance is worse than random. Decay follows exponential pattern E(t) = E₀e^(-λt).

**Causes:**
- Error context amplification — anchors on failure symptoms, not root cause
- Loss of global invariants — each fix reshapes code without preserving system constraints
- Exploration collapse — refines same broken approach instead of exploring alternatives
- Transformer limitation — no accumulation of understanding; reweights attention over biased context

**Solution: Strategic Fresh Start**
- At optimal cutoff point (t₀), reset context instead of continuing
- Shift from exploitation to exploration
- DDI metrics: E₀, λ, t₀, R²

**Implications for Kova:**
- Kova's fix loop (cargo_check → fix → retry) is subject to DDI
- `orchestration_max_fix_retries` — consider cap at 2–3, not 5+
- After N failures: **strategic fresh start** — discard current code, re-prompt with original intent + "previous attempts failed" summary, not full stderr chain
- Recursive Academy: explain *why* decay happens; teach users to recognize "stuck" and when to reset

---

## 3. Kalosm RAG (DocumentTable)

**Source:** floneum.com/kalosm/docs/guides/retrieval_augmented_generation

**Finding:** Kalosm has built-in RAG via `DocumentTable` — indexes SurrealDB with vector embeddings (BERT default). `document_table.search(query).with_results(5)` returns relevant chunks.

**Setup:**
```rust
let document_table = db.document_table_builder("documents")
    .at("./db/embeddings.db")
    .build()
    .await?;
// Search: document_table.search(&user_question).with_results(5)
```

**Implications for Kova:**
- RAG over traces + academy modules is feasible in Rust with Kalosm
- SurrealDB + embeddings.db — adds dependency; consider sled-only for MVP
- Alternative: simple keyword/BM25 over trace summaries if embeddings are heavy

---

## 4. ExIt (Exploratory Iteration)

**Source:** Bootstrapping Task Spaces for Self-Improvement; Emergent Mind

**Finding:** Autocurriculum that bootstraps task space from **informative single-step transitions**. Trains on high-variance partial histories (learnability score = var(r)); chains at inference for multi-step self-revision.

**Key:** Don't train on full multi-step trajectories (costly). Train improvement operator on single-step transitions; chain at test time.

**Implications for Kova:**
- Recursive Academy: don't generate full curriculum at once
- Generate one module per failure pattern; each module = single-step "what went wrong → how to fix"
- Chain: failure A → module A → failure B → module B → ...

---

## 5. AI Coding Failure Patterns (Augment, Syncause)

**Source:** Augment Code debugging guide; Syncause debugging decay

**Finding:** AI coding failures cluster in predictable patterns: hallucinated APIs, security vulnerabilities, performance anti-patterns, missing edge cases. Quick diagnostic: linter → types → tests (3 min sanity check).

**Implications for Kova:**
- Recursive Academy modules can map to these patterns
- "Explain last run" → classify into pattern → retrieve module for that pattern
- Pipeline order (check → clippy → test) aligns with diagnostic order

---

## 6. Recursive Self-Improvement (STOP, Ouroboros, RISE)

**Source:** Self-Taught Optimizer; Ouroboros Protocol; Recursive Introspection

**Finding:**
- **STOP:** LM writes code that improves itself via multiple queries; selects best. No weight change.
- **Ouroboros:** Training data describes AI's own architecture; recursive self-analysis.
- **RISE:** Fine-tune to detect/correct mistakes across turns; multi-turn MDP; online imitation.

**Implications for Kova:**
- Recursive Academy: Kova generates training content; Kova (or user) consumes it. No weight change — in-context learning.
- RISE-style: teach model to "introspect" on failed runs before retrying — could add "pre-fix reflection" step

---

## 7. Easy-to-Hard Curriculum

**Source:** Task-Centric Theory for Iterative Self-Improvement

**Finding:** Curricula that gradually shift from easy to hard outperform fixed task mixtures. Reward-verified outputs benefit from difficulty scheduling.

**Implications for Kova:**
- Recursive Academy: order modules by failure frequency or severity (easy first)
- Or: order by user progression — "you've seen cargo_check failures; here's clippy"

---

## Summary: What to Apply

| Research | Apply to Kova |
|----------|---------------|
| Trace as optimization | Structure pipeline traces; use for "Explain" and future prompt tuning |
| DDI | Cap fix retries; add strategic fresh start after 2–3 failures |
| Kalosm RAG | Use DocumentTable for RAG over traces when ready; MVP = keyword search |
| ExIt | One module per failure; chain at inference |
| Failure patterns | Map curriculum to known patterns (hallucinated API, etc.) |
| Easy-to-hard | Order curriculum by difficulty |

---

## 8. Cursor Prompts as Training Data

**Source:** `.cursor/rules/`, `.cursor/skills/`, AGENTS.md, protocol_map, compression_map

**Finding:** Cursor rules and workspace docs encode conventions (tokenization, blocking, anti-patterns, project structure). These are high-quality, human-curated prompt data.

**Implications for Kova:**
- Load Cursor prompts and inject into Coder, Fix, Router, Academy system prompts
- Models receive workspace conventions without manual copy-paste
- Recursive Academy explanations can reference these rules ("per tokenization rule…")
- Single source of truth: Cursor rules = Kova context

---

## Fastest Path (Updated)

1. **Cursor prompts** — Load and inject into all model prompts (Phase -1)
2. **Trace capture** — Last run; intent, stderr, retries, outcome
3. **Explain button** — "Explain last run" → model with trace + prompt
4. **DDI-aware fix loop** — After 2–3 retries, strategic fresh start (re-prompt from scratch with summary)
5. **RAG later** — When Kalosm DocumentTable is ready; index traces + modules
