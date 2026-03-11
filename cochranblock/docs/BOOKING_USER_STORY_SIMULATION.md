<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# User Story Analysis Simulation — Booking Page

**Simulation date:** 2026-02-26  
**Method:** Persona walkthrough (Hiring Manager Sarah, Recruiter Mike, Mobile User Jordan)  
**Context:** Braintrust talent profile → cochranblock.org/book

---

## Simulation 1: Hiring Manager Sarah (Desktop)

**Scenario:** Sarah is evaluating Michael for a cyber role. She clicks the calendar link from his Braintrust profile.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on /book | Clear "Schedule a Discovery Call" | ✓ Heading visible |
| 2 | Understand call type | 30-min discovery call | ✓ "30 minutes · Discuss your goals" |
| 3 | See availability | Next 1–2 weeks, Tue/Thu | ⚠ Calendar shows full month; available vs unavailable dates not immediately obvious |
| 4 | Pick a date | Click Tue or Thu | ✓ Available dates clickable; others grayed |
| 5 | Pick a time | 2:00, 2:30, 3:00, 3:30 ET | ✓ Time panel appears after date selection |
| 6 | Book | One click → email opens | ✓ mailto with pre-filled subject |
| 7 | Know it's Michael's page | No confusion | ✓ "Shared via Braintrust" + nav shows Michael Cochran |

**Pain points:** Available dates blend with unavailable; no upfront "X days available this month" cue.

---

## Simulation 2: Recruiter Mike (Desktop)

**Scenario:** Mike needs to propose times to a client. He wants to see availability quickly.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Scan availability | See Tue/Thu pattern | ⚠ Must inspect calendar; no legend or badge |
| 2 | Navigate months | Prev/next arrows | ✓ Works |
| 3 | Know timezone | ET | ✓ "All times Eastern (ET)" |
| 4 | Fallback | If no slot fits | ✓ "None of these work? Email me" |

**Pain points:** No "4 days available" badge; must click through to understand availability density.

---

## Simulation 3: Mobile User Jordan (Phone)

**Scenario:** Jordan views the page on a phone during a commute.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Load page | Responsive layout | ✓ (assumed from max-width, touch targets) |
| 2 | Tap date | 44px min touch target | ✓ CSS min-height: 44px on mobile |
| 3 | See time panel | Below fold? | ⚠ scrollIntoView added; needs verification |
| 4 | Tap time slot | Opens mailto | ✓ |

**Pain points:** Calendar grid may be cramped on small screens; time panel could be cut off.

---

## User Story Coverage Summary

| US | Story | Status |
|----|-------|--------|
| US1 | See available slots | ✓ Calendar shows; clarity could improve |
| US2 | Know 30-min call | ✓ |
| US3 | See 1–2 weeks | ✓ 6 date slots (~3 weeks) |
| US4 | One-click book | ✓ mailto |
| US5 | Know it's Michael's page | ✓ |
| US6 | Mobile usable | ✓ Touch targets; layout TBD |

---

## Recommendations from User Story Simulation

1. **Available-day badge** — Show "4 days available" or similar in month header so recruiters scan faster.
2. **Legend or visual cue** — "Tue & Thu · 2–4pm ET" so users understand the pattern without clicking.
3. **Stronger available/unavailable contrast** — Make clickable dates pop; unavailable should read as "not available" not "maybe?"
4. **Mobile verification** — Ensure time panel and calendar fit viewport; test 375px width.
