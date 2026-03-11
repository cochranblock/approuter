<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# UI/UX Analysis Simulation — Booking Page (Calendar Layout)

**Based on:** Screenshot capture of /book page (calendar + time selector)  
**Context:** Braintrust talent profile calendar link  
**Date:** 2026-02-26

---

## Current Implementation

- **Calendar grid:** Month view with prev/next navigation, weekday headers, date cells
- **Available dates:** Tue/Thu only, highlighted; others disabled
- **Time panel:** Appears on date selection; 4 slots (2:00pm–3:30pm ET)
- **Flow:** Pick date → Pick time → mailto opens with pre-filled subject

---

## Findings

### Strengths
- Real calendar layout (month grid, navigation)
- Clear two-step flow: date then time
- Braintrust context explicit
- 30-min duration stated
- Time zone (ET) shown
- Fallback: "None of these work? Email me"
- Mobile touch targets (min 44px)

### Gaps & Recommendations

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | No visual cue for "available" vs "unavailable" beyond opacity | Add distinct styling: available = accent border/glow; unavailable = muted, crosshatch or disabled |
| 2 | Time panel appears below fold on small viewports | Add scroll-into-view when time panel opens |
| 3 | No loading/ready state for calendar | Calendar renders immediately; consider subtle fade-in for polish |
| 4 | Selected date not visually emphasized | Add selected state styling when a date is chosen |
| 5 | No keyboard hint for power users | Add "Select a date, then a time" or aria-describedby |
| 6 | Calendar could show "available" badge on month nav | Optional: show count of available days in current month |

---

## Recommendations (Implemented)

1. **Selected date state** — ✓ `.booking-calendar-day-selected` accent border/background
2. **Scroll time panel into view** — ✓ `panel.scrollIntoView({ behavior: 'smooth' })` on select
3. **Stronger available/unavailable contrast** — ✓ Available: accent glow; unavailable: muted bg/border
4. **Skip-link target** — ✓ `#booking-calendar` skip-link present
