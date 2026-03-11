<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Test Quality Checks

**Purpose:** Ensure tests are industry-relevant, deterministic, and actionable.  
**Apply:** Before marking any test task done.  
**Date:** 2026-03

---

## Quality Criteria

| # | Criterion | Check |
|---|-----------|-------|
| 1 | **Determinism** | No random, no sleep-based timing, no flaky order |
| 2 | **Industry relevance** | Waiver ESIGN, DB audit trail, 400 on invalid input |
| 3 | **Coverage** | All pages (/, /about, /contact, /waiver, /waiver/confirmed) |
| 4 | **Actionable failures** | Fail message points to fix (e.g. "missing scroll hint") |
| 5 | **No self-licking** | Tests assert behavior, not implementation details |
| 6 | **Viewport-aware** | Mobile (375px) and desktop (769px) where applicable |
| 7 | **Accessibility** | Skip-link, main landmark, focus, labels |
| 8 | **Triple run** | 3 sequential passes, all pass |

---

## Pre-Commit Checklist

- [ ] `cargo run --bin oakilydokily-test --features tests -- --test` passes
- [ ] Run 3 times in a row — all pass
- [ ] No new `#[ignore]` without comment
- [ ] New tests have `/// fN=` doc for traceability
- [ ] Waiver tests: DB roundtrip, terms_hash, audit (IP/UA)

---

## Simulation Iteration Quality

### UI/UX Simulation

- [ ] Each page has viewport-specific checks
- [ ] Tap targets ≥ 42px (44px ideal)
- [ ] Skip-link and main landmark present
- [ ] No overflow at 320px

### Feature Gap Simulation

- [ ] Each criterion has severity (P0/P1/P2)
- [ ] Gaps are specific (not "page broken")
- [ ] No duplicate criteria

### User Analysis Simulation

- [ ] Each persona has ≥3 flow steps
- [ ] Flows are realistic (not synthetic)
- [ ] Pain points are specific and fixable

---

## Anti-Patterns to Avoid

| Anti-pattern | Fix |
|--------------|-----|
| Test passes because server not running | Require server up, or spawn in test |
| Flaky due to timing | Use explicit wait or poll, not sleep |
| Asserting HTML structure | Assert content/behavior, not DOM shape |
| Missing edge cases | Add 400 tests for waiver invalid input |
| No DB assertion | waiver_roundtrip_db must verify row |
