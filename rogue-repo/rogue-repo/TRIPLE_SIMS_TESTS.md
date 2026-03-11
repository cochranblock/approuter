<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: Rogue Repo — Tests & API (d6)

**Target:** Rogue Repo — sovereign ISO 8583 payment engine + Rogue Bucks economy  
**Method:** d1 (User Story) → d2 (Feature Gap) → d3 (UI/UX), then implementation  
**Date:** 2026-02-27

---

## Current Tests Evaluation

**Finding:** Rogue Repo has f30 aggregator; f49 (unit), f50 (integration), f51 (HTTP) implemented.

| Component | Expected (per CI rules) | Current |
|-----------|-------------------------|---------|
| `--test` CLI flag | Binary runs self-evaluation | ✓ f30 |
| f49 (Unit) | Vault, switch, core logic | ✓ vault_*, switch_* |
| f50 (Integration) | Real DB, isolated per test | ✓ ledger f14, f15, f16 |
| f51 (HTTP) | Real server, real requests | ✓ GET /, f5, f92, f93, f91, f94, f95, f102, f103, POST f87–f89 |
| f30 (run_tests) | Aggregator, exit 0/1 | ✓ |

---

## d1: User Story Analysis

**Personas:** Game developer (API integration), FinOps/auditor (verifying flows), End user (buying bucks, provisioning)

---

### Simulation 1: Game Developer (API Integration)

**Scenario:** Developer integrating a game client with Rogue Repo. Needs to call f87, f88, f89 and understand responses.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on f4 | API overview or docs | ✓ Landing; no API docs |
| 2 | POST f87 (buy-bucks) | 200 + credits c1 | ✓ 200; t84 placeholder |
| 3 | POST f88 (provision-app) | 200 + entitlement | ✓ 200; t84 placeholder |
| 4 | POST f89 (add-device) | 200 + new device | ✓ 200; t84 placeholder |
| 5 | Invalid JSON / missing field | 422 or 400 | ✓ f51 tests |
| 6 | Find API spec | OpenAPI, schema | ⚠ None |

**Pain points:** f87–f89 placeholder. No API spec. f30 validates routes.

---

### Simulation 2: FinOps / Auditor (Verifying Flows)

**Scenario:** Auditor verifying payment flows, PAN handling, t4 (Ledger) integrity.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Verify PAN never logged | No plaintext in logs | ✓ Vault encrypts |
| 2 | Verify ISO 8583 format | f12 produces valid message | ✓ f49 switch_* |
| 3 | Verify add-device non-destructive | Old devices remain | ✓ t4 f14 inserts |
| 4 | Run test suite | CI passes | ✓ f30 |
| 5 | Check transaction audit | transactions table | ✓ Schema; not populated by handlers |

**Pain points:** f87–f89 don't call t4 yet. f49/f50 prove vault, switch, ledger.

---

### Simulation 3: End User (Buying Bucks, Provisioning)

**Scenario:** User buys entry (c0), provisions game (c2), adds device (c3).

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Visit f4 | Clear value prop | ✓ |
| 2 | Understand economy | c4 bucks = $1, etc. | ✓ Economy table |
| 3 | Call f87 | Credits c1 | Placeholder |
| 4 | Call f88 | Deducts c2, grants entitlement | Placeholder |
| 5 | Call f89 | Deducts c3, adds fingerprint | Placeholder |
| 6 | Auth first | f97, f98 | ✓ f102, f103; f97–f101 |

**Pain points:** f87–f89 placeholder. Auth (f97–f103) in place.

---

## User Story Coverage Summary

| US | Story | Status |
|----|-------|--------|
| US1 | Developer integrates API | ⚠ f87–f89 placeholder; f30 validates |
| US2 | Auditor verifies flows | ✓ f49, f50 |
| US3 | User understands economy | ✓ c0–c4 on landing |
| US4 | Test suite validates behavior | ✓ f30 |
| US5 | Landing page conveys value | ✓ |
| US6 | Auth (register, login, verify) | ✓ f97–f103 |

---

## d2: Feature Gap Analysis

**Method:** Current implementation vs acceptance criteria + CI rules  
**Reference:** repo-vault, repo-switch, repo-ledger, rogue-repo

---

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Test binary | `cargo run -- --test` | ✓ f30 | None |
| f49 unit tests | Vault, switch | ✓ | None |
| f50 integration | DB isolated | ✓ | None |
| f51 HTTP | Real server, routes | ✓ f4, f5, f87–f89, f94, f95, f102, f103 | None |
| API handlers | Real logic | f87–f89 placeholder | **High** |
| Landing economy | c4 = $1 | ✓ | None |
| f5 health | GET /health | ✓ | None |
| Auth | f97–f103 | ✓ | None |
| Rogue Runner | f94, f95 | ✓ | None |

---

### Feature Gaps (Ideal vs Current)

#### Gap 1: f87, f88, f89 placeholder
**Ideal:** Call t4 (Ledger), vault, switch.  
**Current:** Return t84 static.  
**Severity:** High.

#### Gap 2: No API docs
**Ideal:** OpenAPI or README spec.  
**Current:** None.  
**Severity:** Medium.

---

### Prioritized Recommendations

| # | Recommendation | Priority |
|---|----------------|----------|
| 1 | Wire f87, f88, f89 to t4 | High |
| 2 | API docs (README or /api) | Medium |

---

## d3: UI/UX Analysis

**Based on:** pwa.rs (f90), routes, auth (f102, f103)  
**Context:** FinTech API + landing; developer and auditor audiences

---

### Current Implementation

- **Landing (f4):** f90 dark theme. "Rogue Repo" + tagline + economy (c0–c4) + f94/f95 app cards.
- **API routes:** f87, f88, f89 — t83, t6, t86 in; t84 out. Placeholder.
- **Auth:** f102, f103 (login, register); f97, f98, f100, f101.
- **Games:** f94 (HTML), f95 (WASM) Rogue Runner.

---

### Findings

#### Strengths
- Aesthetic matches mission (secure, high-tech).
- Semantic structure, responsive layout.
- f30 validates all routes.
- f97–f103 auth flow complete.

#### Gaps & Recommendations

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | f87–f89 placeholder | Wire to t4 post-auth |
| 2 | No API spec | Document in README |
| 3 | t14 error schema | Use consistently |

---

## d4: Implementation Summary

**Executed:** After all 3 analyses (sequential)

| # | Item | Done |
|---|------|------|
| 1 | --test flag + f30 | ✓ |
| 2 | f49 unit tests (vault, switch) | ✓ |
| 3 | f51 HTTP tests | ✓ |
| 4 | f50 integration (minimal/conditional) | ✓ |
| 5 | Economy table on landing (c0–c4) | ✓ |
| 6 | f5 health | ✓ |
| 7 | f94 Rogue Runner (HTML) | ✓ |
| 8 | f95 Rogue Runner (WASM) | ✓ |
| 9 | f97–f103 Auth | ✓ |
| 10 | f102, f103 Login/Register | ✓ |
