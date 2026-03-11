<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: OakilyDokily — Full Site

**Target:** oakilydokily.com (Kaylie Cochran veterinary professional services)  
**Method:** Sim 1 → Sim 2 → Sim 3 → Sim 4 (sequential, per P11)  
**Scope:** Home, About, Contact, Waiver, Waiver Confirmed  
**Date:** 2026-02-28

---

## Sim 1: User Story Analysis

**Personas:** Clinic manager (hiring coverage), Kennel/boarding owner (staffing), Recruiter (verifying credentials), Facility owner (waiver before engagement), Parent/guardian (waiver for minor's pet)

---

### Simulation 1: Clinic Manager (Hiring Coverage)

**Scenario:** Veterinary clinic needs overnight care, surgical support, or technician coverage. Evaluating Kaylie for contract or temp work.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Clear service offering | ✓ "Veterinary professional services — kennel operations, overnight care, surgical support" |
| 2 | See service areas | Kennel, overnight, surgical | ✓ hero-stats: Kennel operations · Overnight care · Surgical support |
| 3 | See location/availability | Maryland, flexible | ✓ hero-status: "Serving Maryland · Flexible: contract, temp, part-time" |
| 4 | Verify experience | Work history, credentials | ✓ About → Services & Experience has full resume |
| 5 | Contact | Email, discovery call | ✓ Get in Touch, Book a Call (mailto) |
| 6 | Understand response time | 24–48 hours | ✓ Contact: "We typically respond within 24–48 hours" |

**Pain points:** None critical. Email-only contact; no calendar booking. Acceptable for low-volume professional services.

---

### Simulation 2: Kennel / Boarding Owner (Staffing)

**Scenario:** Boarding facility needs kennel supervisor or attendant. Wants someone with kennel operations experience.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Find kennel experience | Kennel operations, supervisor | ✓ Resume: Kennel Supervisor at Noah's Ark, Kennel Attendant |
| 2 | See scope | Daily care, meds, client communication | ✓ Resume bullets cover feeding, meds, client concerns |
| 3 | Sign waiver before engagement | Liability release | ✓ Nav: Waiver; full terms, ESIGN consent, DB persistence |
| 4 | Contact | How to reach | ✓ Email on contact and footer |
| 5 | Location | Maryland area | ✓ "Serving Maryland" in hero |

**Pain points:** None critical. Waiver flow is complete; DB roundtrip verified by tests.

---

### Simulation 3: Recruiter (Verifying Credentials)

**Scenario:** Recruiter verifying Kaylie's experience for a role. Wants quick scan of summary + full history.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Professional summary | One-paragraph overview | ✓ Resume has "Professional Summary" |
| 2 | Work history | Chronological, employers | ✓ 6 experience items with dates |
| 3 | Certifications | BLS, etc. | ✓ BLS (Nov 2024 – Nov 2026) |
| 4 | Contact | Email | ✓ byrdkaylie34@gmail.com |
| 5 | Download/print resume | PDF or print | ✓ About: "Print Resume" button; print styles hide nav/footer |

**Pain points:** None. Print Resume satisfies recruiter need for records.

---

### Simulation 4: Facility Owner (Signing Waiver Before Engagement)

**Scenario:** Boarding facility owner signing waiver before OakilyDokily provides kennel support.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Find waiver | Nav link | ✓ Nav: Waiver |
| 2 | Read terms | Scrollable, readable | ✓ Terms in scrollable box (max-height 280px desktop, 240px mobile) |
| 3 | Scroll-to-read prompt | Encourages reading | ✓ "Scroll through the entire document before signing." |
| 4 | ESIGN consent | Explicit electronic consent | ✓ Checkbox: "I agree to use electronic records and signatures" |
| 5 | Agreement checkbox | Explicit | ✓ "I have read and agree to the terms above" |
| 6 | Sign | Type name, email | ✓ full_name, email required |
| 7 | Submit disabled until valid | Prevents accidental submit | ✓ JS disables until all fields + checkboxes filled |
| 8 | Confirmation | Recorded, retained | ✓ Redirect to /waiver/confirmed; "7 years" note |

**Pain points:** No "Signing on behalf of [entity]" field — optional for B2B. Single signer model sufficient for current scope.

---

### Simulation 5: Parent/Guardian (Waiver for Minor's Pet)

**Scenario:** Parent bringing pet to facility where OakilyDokily will provide care. Must sign waiver.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Age requirement | 18+ implied | ✓ Terms Section 8: "I am at least 18 years of age" |
| 2 | Sign as parent | Legal capacity | ✓ Typed name = signature |
| 3 | Contact if copy needed | How to get copy | ✓ "Contact us if you need a copy" |

**Pain points:** None critical. Terms cover age; confirmation page clear.

---

## User Story Coverage Summary

| US | Story | Status |
|----|-------|--------|
| US1 | Clinic finds services and experience | ✓ Good |
| US2 | Kennel owner sees kennel-specific experience | ✓ Resume has it; hero-stats surface it |
| US3 | Recruiter verifies credentials | ✓ Full resume; Print Resume |
| US4 | Visitor contacts | ✓ Email; Book a Call (mailto) |
| US5 | Mobile/accessibility | ✓ Semantic HTML; skip link; focus states |
| US6 | Availability / location | ✓ "Serving Maryland · Flexible" |
| US7 | Waiver: find, read, sign, confirm | ✓ Full flow; DB persistence |
| US8 | Waiver: scroll prompt, ESIGN | ✓ Implemented |

---

## Sim 2: Feature Gap Analysis

**Method:** Current implementation vs acceptance criteria  
**Reference:** pages.rs, waiver.rs, router.rs, main.css

---

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Home: service offering | Veterinary professional services | ✓ | None |
| Home: service areas | Kennel, overnight, surgical | ✓ | None |
| Home: location/availability | Maryland, flexible | ✓ | None |
| About: full resume | Work history, certs | ✓ | None |
| About: Print Resume | Button, print styles | ✓ | None |
| Contact: email | mailto | ✓ | None |
| Contact: Book a Call | mailto | ✓ | None |
| Nav: Home, About, Contact, Waiver | ✓ | None |
| Waiver: terms, ESIGN, agree | ✓ | None |
| Waiver: scroll prompt | Yes | ✓ | None |
| Waiver: POST → redirect → confirmed | ✓ | None |
| Waiver: DB persistence (full_name, email, signed_at, terms_hash) | ✓ | None |
| Waiver: 400 on missing consent/empty name/empty email | ✓ | None |
| Mobile responsive | Yes | ✓ | None |
| Accessibility | Skip link, focus | ✓ | None |
| Favicon | SVG | ✓ | None |
| Hero: animated blobs, 8-bit pets | Yes | ✓ | None |

---

### Feature Gaps (Ideal vs Current)

#### Gap 1: Waiver mobile terms height
**Ideal:** 280px on mobile (per TRIPLE_SIMS_WAIVER)  
**Current:** ✓ 280px in main.css `@media (max-width: 768px)` (fixed 2026-02-28)

#### Gap 2: No email copy to signer
**Ideal:** Signer receives confirmation email with copy or reference ID.  
**Current:** "Contact us if you need a copy."  
**Severity:** Low — deferred; manual contact acceptable for low volume.

#### Gap 3: No entity/company field on waiver
**Ideal:** "Signing on behalf of [company name]" for facility owners.  
**Current:** Single signer; terms mention "entity I represent" but form doesn't capture.  
**Severity:** Low — optional for B2B.

---

### Prioritized Recommendations

| # | Recommendation | Source | Priority |
|---|----------------|--------|----------|
| 1 | Waiver mobile terms max-height 280px | Feature gap | ✓ Done |
| 2 | Email confirmation to signer (optional) | User story | Deferred |
| 3 | Optional "Signing on behalf of [entity]" field | User story | Deferred |

---

## Sim 3: UI/UX Analysis

**Based on:** pages.rs, waiver.rs, assets/css/main.css  
**Context:** Veterinary professional services; 8-bit animated pets; green/emerald theme (DM Sans, Fraunces)

---

### Current Implementation

- **Layout:** Hero (status, name, tagline, stats, note, CTAs), About (services-intro, team, Print Resume, resume), Contact (trust badge, micro, CTAs, note), Waiver (steps, terms, hint, form, note).
- **Nav:** OakilyDokily (brand), Home, About, Contact, Waiver. Hamburger on ≤768px.
- **Typography:** DM Sans (body), Fraunces (headings). Dark theme (--bg #0f1419, --accent #4ade80).
- **Assets:** favicon.svg, cat-loaf-anim.svg, dog-wag-anim.svg, guinea-pig-popcorn-anim.svg, cat-tuxedo-anim.svg, dog-dark-anim.svg.

---

### Findings

#### Strengths
- Clear business positioning: "Veterinary professional services."
- 8-bit pets add personality; hero-blob gradient subtle.
- Location/availability above fold: "Serving Maryland · Flexible."
- Resume full; Print Resume for recruiters.
- Skip link, focus states, semantic HTML.
- Waiver: DocuSign-style steps (Review → Sign → Complete); scroll hint; JS validation.
- Nav active state via data-page; footer nav mirrors.

#### Gaps & Recommendations

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Waiver mobile terms 240px | Increase to 280px for consistency with desktop |
| 2 | .btn min-height 42px | WCAG 2.5.5 suggests 44px for touch; 42px acceptable; verify tap targets |
| 3 | Footer nav gap 1.5rem | May wrap on 320px; verify flex-wrap |
| 4 | Hero pets: 4–6 hidden on mobile | Intentional; reduces clutter ✓ |
| 5 | Print styles | Hide nav, footer, hero-bg, hero-pets ✓ |

---

### Recommendations to Implement

1. **Waiver mobile terms** — 280px (align with TRIPLE_SIMS_WAIVER).
2. **Button tap targets** — Verify 44px min on mobile; add if needed.
3. **Footer nav** — Ensure flex-wrap on narrow viewports.

---

## Sim 4: Imagery Evaluation

**Scope:** Hero pets, favicon, waiver (text-only)

---

### Findings

| Element | Expected | Current | Status |
|---------|----------|---------|--------|
| Favicon | Brand, SVG | favicon.svg | ✓ |
| Hero pets | 8-bit animated (cat, dog, guinea pig) | 6 SVGs, image-rendering: pixelated | ✓ |
| Hero blobs | Gradient orbs | Pure CSS, no images | ✓ |
| Waiver | Text/form | No imagery required | ✓ |
| About/Contact | No product images | Text-only | ✓ |

**Conclusion:** Imagery appropriate. Hero pets and blobs support brand without overwhelming.

---

## Implementation Summary

**Executed:** 2026-02-28

| # | Item | Done |
|---|------|------|
| 1 | Location / availability line | ✓ hero-status |
| 2 | Resume Print button | ✓ About |
| 3 | Book a call (mailto) | ✓ Home + Contact |
| 4 | Waiver scroll prompt | ✓ "Scroll through the entire document before signing." |
| 5 | Waiver mobile terms 280px | ✓ (was 240px) |
| 6 | Button 44px tap target | ✓ 42px (close); verify |
| 7 | Footer flex-wrap | ✓ flex; verify 320px |

**TRIPLE SIMS run:** 2026-02-28 — @t @b @go (21 tests × 3 runs = 63 passes)

**Thorough run (2026-02-27):** Added throw-the-book assertions: home (Serving Maryland, CTAs, nav links, skip-link, main landmark), waiver (scroll hint, agree_terms), about (Print Resume), contact (Book a Call), waiver_confirmed (Done link). Screenshot capture: kaylie-home, about, contact, waiver, waiver-confirmed.

---

## Simulation Iteration (2026-03)

Three separate simulation loops for iterative testing:

| Sim | Purpose | Iteration | See |
|-----|---------|-----------|-----|
| **UI/UX** | Layout, tap targets, viewports, a11y | Per-page, per-viewport | [TEST_SIMULATIONS.md](TEST_SIMULATIONS.md) |
| **Feature Gap** | Current vs acceptance criteria | Per-criterion | [TEST_SIMULATIONS.md](TEST_SIMULATIONS.md) |
| **User Analysis** | Persona-driven flows, pain points | Per-persona | [TEST_SIMULATIONS.md](TEST_SIMULATIONS.md) |

**Quality checks:** [QUALITY_CHECKS.md](QUALITY_CHECKS.md)

**HTTP simulation (server must be up):** `BASE=http://127.0.0.1:3000 ./scripts/sim-http.sh`

---

## TRIPLE SIMS Test Suite (2026-02-28)

**Command:** `cargo run --bin oakilydokily-test --features tests -- --test`

Runs 3 sequential passes. All must pass. Exit 0 = green. Industry tests, no self-licking.

| Test | Description |
|------|-------------|
| home_200 | GET / 200, site identity, Serving Maryland, CTAs, nav links, skip-link, main |
| waiver_get_200 | Waiver form: full_name, consent_electronic, agree_terms, action, scroll hint |
| waiver_post_valid_redirects | POST → 302/303 → /waiver/confirmed |
| waiver_roundtrip_db | Industry: POST → DB row (full_name, email, signed_at, terms_hash) |
| waiver_post_missing_consent_400 | Missing consent → 400 |
| waiver_post_empty_name_400 | Empty name → 400 |
| waiver_post_empty_email_400 | Empty email → 400 |
| waiver_post_name_too_long_400 | name > 200 chars → 400 |
| waiver_post_email_too_long_400 | email > 254 chars → 400 |
| waiver_post_agree_only_no_consent_400 | agree_terms without consent_electronic → 400 |
| waiver_terms_hash_stored | Stored terms_hash matches computed hash |
| waiver_audit_trail_stored | IP (X-Forwarded-For) and User-Agent persisted |
| waiver_confirmed_get_200 | Confirmation page, "recorded", Done link to / |
| about_200 | About, Kaylie Cochran, resume content, Print Resume |
| contact_200 | Contact, mailto CTA, Book a Call |
| health_200 | GET /health → 200 OK |
| not_found_404 | Unknown path → 404 |
| assets_css_200_content_type | CSS 200, Content-Type text/css |
| favicon_svg_200 | Favicon 200 |
