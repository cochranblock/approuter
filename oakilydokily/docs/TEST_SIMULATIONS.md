<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Test Simulations — UI/UX, Feature Gap, User Analysis

**Target:** oakilydokily.com (Kaylie Cochran veterinary professional services)  
**Method:** Three separate simulation loops, each iterating through assertions  
**Quality:** Tests checked for coverage, determinism, industry relevance  
**Date:** 2026-03

---

## Overview

| Simulation | Purpose | Iteration | Output |
|------------|---------|-----------|--------|
| **Sim UI/UX** | Layout, tap targets, viewports, accessibility | Per-page, per-viewport | Pass/fail + recommendations |
| **Sim Feature Gap** | Current vs acceptance criteria | Per-criterion | Gap list + priority |
| **Sim User Analysis** | Persona-driven flows, pain points | Per-persona | User story coverage |

---

## Sim 1: UI/UX Testing Simulation

**Scope:** Layout, typography, tap targets, viewports, semantic HTML, focus order

### Iteration Matrix

| Page | Viewport | Checks |
|------|----------|--------|
| / | 375px | Hero readable, CTAs tappable, skip-link, main landmark |
| / | 769px | Full hero, nav inline, hover states |
| /about | 375px | Resume content, Print button, tap targets |
| /about | 769px | Experience cards, hover border |
| /contact | 375px | mailto CTA, Book a Call, 44px tap |
| /contact | 769px | Trust badge, micro copy |
| /waiver | 375px | Terms 280px, form labels, submit disabled |
| /waiver | 769px | Steps visible, scroll hint |
| /waiver/confirmed | * | Done link, retention note |

### UI/UX Assertions (HTTP + HTML parse)

- `main` landmark present
- Skip-link `href="#main"`, focus visible
- `.btn` min-height ≥ 42px (44px ideal)
- Nav links reachable, aria-expanded on mobile
- Waiver terms box max-height 280px on mobile
- Form labels `for`/`id` linked
- No overflow on 320px viewport

### Quality Checks for UI/UX Tests

- [ ] Each assertion has expected + observed
- [ ] Viewport-specific (no desktop-only assumptions)
- [ ] Accessibility: skip-link, focus, landmarks
- [ ] No flaky (no timing-dependent)

---

## Sim 2: Feature Gap Analysis Simulation

**Scope:** Acceptance criteria vs current implementation

### Criteria Matrix

| Criterion | Expected | Severity | Gap Action |
|-----------|----------|----------|------------|
| Home: service offering | Veterinary professional services | P0 | Assert in HTML |
| Home: location/availability | Maryland, flexible | P0 | Assert hero-status |
| About: full resume | Work history, certs | P0 | Assert content |
| About: Print Resume | Button, print styles | P1 | Assert onclick/print |
| Contact: email | mailto | P0 | Assert href |
| Contact: Book a Call | mailto | P1 | Assert subject |
| Waiver: terms, ESIGN, agree | All present | P0 | Assert form fields |
| Waiver: scroll prompt | "Scroll through entire document" | P1 | Assert text |
| Waiver: POST → redirect | 302/303 → /waiver/confirmed | P0 | Assert redirect |
| Waiver: DB persistence | full_name, email, signed_at, terms_hash | P0 | Assert DB row |
| Waiver: 400 on invalid | Missing consent, empty name/email | P1 | Assert status |
| Mobile responsive | No horizontal scroll 320px | P1 | Assert viewport |
| Favicon | SVG | P2 | Assert 200, Content-Type |

### Feature Gap Iteration

1. Load criterion
2. Fetch page / perform action
3. Compare observed vs expected
4. Record gap (criterion, expected, observed, severity)
5. Output: prioritized gap list

### Quality Checks for Feature Gap Tests

- [ ] Each criterion has severity
- [ ] Gaps are actionable (not vague)
- [ ] No duplicate criteria
- [ ] Industry-relevant (waiver ESIGN, DB audit)

---

## Sim 3: User Analysis and Simulation

**Scope:** Persona-driven flows, pain points, user story coverage

### Personas

| Persona | Goal | Key Flow |
|---------|------|----------|
| Clinic Manager | Hire coverage | Home → services → About → Contact |
| Kennel Owner | Staffing | Home → kennel experience → Waiver → Contact |
| Recruiter | Verify credentials | About → Print Resume → Contact |
| Facility Owner | Sign waiver | Nav Waiver → read terms → sign → confirm |
| Parent/Guardian | Waiver for minor's pet | Waiver → age 18+ → sign |

### User Flow Assertions

- **Clinic Manager:** Home has "Serving Maryland", CTAs, About has resume, Contact has mailto
- **Kennel Owner:** Resume has kennel experience, Waiver reachable, DB roundtrip
- **Recruiter:** About has Print Resume, resume content, contact email
- **Facility Owner:** Waiver has terms, ESIGN, agree, scroll hint, confirmation
- **Parent:** Waiver terms mention 18+, confirmation has retention note

### User Analysis Iteration

1. Select persona
2. Execute flow (GET/POST sequence)
3. Assert each step outcome
4. Record pain points (missing field, unclear copy)
5. Output: user story coverage table

### Quality Checks for User Analysis Tests

- [ ] Each persona has ≥3 steps
- [ ] Flows are realistic (not synthetic)
- [ ] Pain points are specific
- [ ] Coverage map: US → status

---

## Test Quality Checklist

Before marking tests done:

- [ ] **Determinism:** No random, no timing flakiness
- [ ] **Industry:** Waiver ESIGN, DB audit, 400 on invalid
- [ ] **Coverage:** All pages, all personas, all criteria
- [ ] **Actionable:** Failures point to fix
- [ ] **Triple run:** 3 sequential passes, all pass

---

## Commands

```bash
# Run all simulations (when oakilydokily-test exists)
cargo run --bin oakilydokily-test --features tests -- --test

# Run 3 passes (TRIPLE SIMS)
for i in 1 2 3; do cargo run --bin oakilydokily-test --features tests -- --test || exit 1; done

# HTTP simulation against running server (base URL)
BASE=http://127.0.0.1:3000 ./scripts/sim-http.sh
```
