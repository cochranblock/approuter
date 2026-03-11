<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Feature Gap Analysis — Booking Page

**Analysis date:** 2026-02-26  
**Method:** Current implementation vs acceptance criteria + ideal UX  
**Reference:** screenshots/book.png, pages.rs, booking.js, main.css

---

## Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Page title | "Schedule a Discovery Call" | ✓ | None |
| Subheading | "30 minutes · Discuss your mission and how I can help" | ✓ | None |
| Braintrust context | "Shared via Braintrust" or similar | ✓ | None |
| Calendar shows 2 weeks | Tue/Thu 2–4pm ET | ✓ ~3 weeks, 6 dates | None |
| Slots open mailto | "Discovery Call Request — [date] [time]" | ✓ | None |
| Matches site aesthetic | Cosmic/cyber | ✓ | None |
| Mobile-responsive | Yes | ✓ min 44px | None |
| Nav includes Book | Yes | ✓ | None |

---

## Feature Gaps (Ideal vs Current)

### Gap 1: Availability at a glance
**Ideal:** User sees "4 days available this month" or similar without scanning the grid.  
**Current:** Must visually scan to distinguish available (Tue/Thu) from unavailable.  
**Severity:** Medium — slows recruiters.

### Gap 2: Availability pattern legend
**Ideal:** "Available Tue & Thu · 2–4pm ET" visible above or beside calendar.  
**Current:** Implied in context; not explicit.  
**Severity:** Low — reduces cognitive load.

### Gap 3: Unavailable dates read as disabled
**Ideal:** Unavailable dates clearly look non-interactive (muted, strikethrough, or distinct style).  
**Current:** opacity 0.35; may still look tappable on some screens.  
**Severity:** Medium — can cause misclicks or confusion.

### Gap 4: Calendar fade-in / polish
**Ideal:** Subtle animation when calendar loads so it feels responsive.  
**Current:** Renders immediately; no transition.  
**Severity:** Low — polish only.

### Gap 5: Keyboard hint
**Ideal:** "Select a date, then a time" or aria-describedby for screen readers.  
**Current:** aria-label on region; no visible hint.  
**Severity:** Low — accessibility improvement.

### Gap 6: Time panel visibility on mobile
**Ideal:** Time panel always in view when opened; no horizontal scroll.  
**Current:** scrollIntoView added; mobile layout (single column) may need verification.  
**Severity:** Medium — affects mobile UX.

### Gap 7: Skip link to calendar
**Ideal:** Skip link can target #booking-calendar for keyboard users.  
**Current:** #booking-calendar exists; skip link goes to #main.  
**Severity:** Low — consider adding "Skip to calendar" option.

---

## Prioritized Recommendations

| # | Recommendation | Source | Priority |
|---|----------------|--------|----------|
| 1 | Add available-day badge in month header | User story, Feature gap | High |
| 2 | Add legend: "Tue & Thu · 2–4pm ET" | User story, Feature gap | High |
| 3 | Strengthen unavailable styling (clear disabled state) | User story, Feature gap | High |
| 4 | Strengthen available styling (accent, glow) | User story | High |
| 5 | Add calendar fade-in on load | UI/UX | Medium |
| 6 | Add visible hint: "Select a date, then a time" | UI/UX, Feature gap | Medium |
| 7 | Verify mobile layout (time panel, grid) | User story, Feature gap | Medium |
| 8 | Add skip link to calendar | Feature gap | Low |
