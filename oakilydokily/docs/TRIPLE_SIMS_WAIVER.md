<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: Waiver Page Only

**Target:** oakilydokily.com/waiver  
**Scope:** Waiver page only — no other pages  
**Method:** Sim 1 (User Story) → Sim 2 (Feature Gap) → Sim 3 (UI/UX) → Sim 4 (Imagery)  
**Date:** 2026-02-28

---

## Sim 1: User Story Analysis (Waiver)

**Personas:** Clinic manager (signing before engagement), Facility owner (signing for boarding), Parent/guardian (signing for minor's pet)

---

### Simulation 1: Clinic Manager (Signing Before Engagement)

**Scenario:** Clinic wants to engage OakilyDokily for overnight care. Must sign waiver before services begin.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on /waiver | Clear liability release | ✓ "Service Waiver & Liability Release" |
| 2 | Read terms | Scrollable, readable | ✓ Terms in scrollable box (max-height 320px) |
| 3 | Understand ESIGN consent | Explicit electronic consent | ✓ Checkbox: "I consent to conduct this transaction electronically" |
| 4 | Sign | Type name, email, check boxes | ✓ Full name, email, 2 checkboxes |
| 5 | Submit | Button enabled when valid | ✓ JS disables until all fields + checkboxes filled |
| 6 | Confirmation | Recorded, retained | ✓ Redirect to /waiver/confirmed; "7 years" note |

**Pain points:** No "scroll to bottom" prompt — user might sign without reading full terms. No email confirmation sent (confirmation page says "Contact us if you need a copy" — no automated copy). Terms in `<pre>` — monospace can feel dense on mobile.

---

### Simulation 2: Facility Owner (Signing for Boarding)

**Scenario:** Boarding facility owner signing waiver before OakilyDokily provides kennel support.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Find waiver | Nav link | ✓ Nav: Waiver |
| 2 | Read risks | Animal bites, disease, etc. | ✓ Section 1–2 cover risks |
| 3 | Understand indemnification | Hold harmless | ✓ Section 4 |
| 4 | Sign as entity rep | "Authority to bind" | ✓ Section 8: "authority to bind myself and any entity I represent" |
| 5 | Submit | Success | ✓ POST → redirect |

**Pain points:** No field for "Signing on behalf of [entity name]" — entity representation is in terms but not captured in form. Single signer model.

---

### Simulation 3: Parent/Guardian (Signing for Minor's Pet)

**Scenario:** Parent bringing pet to facility where OakilyDokily will provide care. Must sign waiver.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Age requirement | 18+ | ✓ Section 8: "I am at least 18 years of age" |
| 2 | Sign as parent | Legal capacity | ✓ Typed name = signature |
| 3 | Contact if copy needed | How to get copy | ✓ "Contact us if you need a copy" |

**Pain points:** No explicit "I am the owner/guardian of the animal(s)" — implied by context. Minor's pet not explicitly called out in form.

---

## User Story Coverage Summary (Waiver)

| US | Story | Status |
|----|-------|--------|
| US1 | User finds waiver, reads terms | ✓ Good |
| US2 | User consents electronically (ESIGN) | ✓ Explicit checkbox |
| US3 | User agrees to terms | ✓ Explicit checkbox |
| US4 | User signs (typed name + email) | ✓ Good |
| US5 | Submit disabled until valid | ✓ JS validation |
| US6 | Confirmation + retention notice | ✓ Good |
| US7 | Scroll-to-read prompt | ⚠ Missing |
| US8 | Copy of signed waiver | ⚠ Manual (contact us) |

---

## Sim 2: Feature Gap Analysis (Waiver)

**Reference:** waiver.rs, waiver_terms.txt, waiver CSS

---

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Terms displayed | Full waiver text | ✓ | None |
| ESIGN consent | Explicit checkbox | ✓ | None |
| Agreement checkbox | Explicit | ✓ | None |
| Full name (signature) | Required | ✓ | None |
| Email | Required | ✓ | None |
| Submit disabled until valid | Yes | ✓ | None |
| Audit trail (IP, UA, timestamp) | Stored | ✓ | None |
| Terms version/hash | Stored | ✓ | None |
| Confirmation page | Yes | ✓ | None |
| Scroll-to-bottom prompt | Optional | — | Missing |
| Email copy to signer | Optional | — | Missing |
| Entity/company field | Optional | — | Missing |
| Mobile: terms readable | Yes | ✓ | — |
| Accessibility: labels, focus | Yes | ✓ | — |

---

### Feature Gaps (Waiver Only)

#### Gap 1: No scroll-to-bottom prompt
**Ideal:** "Please scroll to the bottom of the terms before signing."  
**Current:** User can check boxes without scrolling.  
**Severity:** Medium — legal best practice to encourage reading.

#### Gap 2: No email copy to signer
**Ideal:** Signer receives confirmation email with copy or reference ID.  
**Current:** "Contact us if you need a copy."  
**Severity:** Medium — common for e-sign flows.

#### Gap 3: No entity/company field
**Ideal:** "Signing on behalf of [company name]" for facility owners.  
**Current:** Single signer; terms mention "entity I represent" but form doesn't capture.  
**Severity:** Low — optional for B2B.

---

## Sim 3: UI/UX Analysis (Waiver)

**Reference:** waiver.rs HTML, main.css waiver section

---

### Current Implementation

- **Layout:** H1 → intro → terms box (scrollable) → fieldset (name, email, checkboxes, submit) → note
- **Typography:** Quicksand (headings), Nunito (body). Terms in pre, 0.9rem.
- **Form:** Labels, inputs, checkboxes. Submit disabled until valid.
- **Theme:** Tropical (pink, orange, purple on black). Matches site.

---

### Findings

#### Strengths
- Clear hierarchy: H1 → intro → terms → form
- ESIGN consent and agreement are separate checkboxes
- Submit disabled until valid — prevents accidental empty submit
- Terms scrollable (320px max-height) — readable on desktop
- Fieldset with legend — semantic
- Focus states on inputs (purple border, box-shadow)
- Confirmation page clear: "recorded with timestamp and audit trail"

#### Gaps & Recommendations

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Terms in `<pre>` — monospace, dense | Consider `white-space: pre-wrap` with proportional font; already in CSS. Verify line-height. |
| 2 | No "scroll to bottom" prompt | Add text above form: "Please scroll through the entire agreement above before signing." |
| 3 | Labels may run together on narrow viewports | Verify `.waiver-form label:not(.waiver-check)` is `display: block` — already is |
| 4 | Checkbox size | `accent-color: var(--purple)` — 1.25rem size. Good. |
| 5 | Mobile: terms height | 240px on mobile — may be short; consider 280px |
| 6 | Confirmation: single CTA | "Return to Home" — could add "Sign Another" if multi-signer flow later |

---

## Sim 4: Imagery Evaluation (Waiver)

**Scope:** Waiver page only — no hero animals, no product images.

---

### Findings

| Element | Expected | Current | Status |
|---------|----------|---------|--------|
| Favicon | Tropical/brand | favicon.svg | ✓ Inherited |
| Waiver-specific imagery | None required | None | ✓ Legal docs typically text-only |
| Form styling | On-brand | Purple border, pink/orange gradient on H1 | ✓ Consistent |
| Confirmation page | Minimal | Text + Return to Home | ✓ Appropriate |

**Conclusion:** Waiver page is text/form focused. No imagery gaps. Brand consistency via CSS variables.

---

## Prioritized Recommendations (Waiver Only)

| # | Recommendation | Source | Priority |
|---|----------------|--------|----------|
| 1 | Add "scroll to bottom" prompt above form | Sim 1, 2, 3 | High |
| 2 | Consider email confirmation to signer (optional) | Sim 1, 2 | Medium |
| 3 | Optional "Signing on behalf of [entity]" field | Sim 1, 2 | Low |
| 4 | Mobile: increase terms max-height to 280px | Sim 3 | Low |

---

## Implementation Summary

**Executed:** 2026-02-28

| # | Item | Done |
|---|------|------|
| 1 | Scroll-to-bottom prompt | ✓ "Please scroll through the entire agreement above before signing." |
| 2 | Mobile terms height | ✓ 280px (was 240px) |
| 3 | Email confirmation | — Deferred |
| 4 | Entity field | — Deferred |
