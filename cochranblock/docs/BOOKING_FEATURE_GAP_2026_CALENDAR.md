<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Feature Gap Analysis — Google Calendar Clone (2026)

**Analysis date:** 2026-02-25  
**Method:** Current implementation vs Google Calendar feature set  
**Reference:** calendar.js, pages.rs, main.css

---

## Google Calendar Features vs Current

| Feature | Google | Current | Gap |
|---------|--------|---------|-----|
| Month view | ✓ | ✓ | None |
| Week view | ✓ | ✓ | None |
| Day view | ✓ | ✓ | None |
| Agenda view | ✓ | ✓ | None |
| Today button | ✓ | ✓ | None |
| Prev/Next nav | ✓ | ✓ | None |
| View switcher | ✓ | ✓ | None |
| Mini calendar | ✓ | ✗ | **Gap** |
| Current time indicator | ✓ | ✗ | **Gap** |
| Create event | ✓ | N/A (booking) | Out of scope |
| Event colors | ✓ | N/A | Out of scope |
| Time zone display | ✓ | ET in legend | Minor |
| All-day events | ✓ | N/A | Out of scope |

---

## Prioritized Gaps

### Gap 1: Mini calendar (date picker)
**Ideal:** Sidebar mini calendar to jump to a date.  
**Current:** Only prev/next; must click through to reach distant dates.  
**Severity:** Medium — UX polish.

### Gap 2: Current time indicator
**Ideal:** In Week/Day view, a horizontal line at current time.  
**Current:** No visual indicator.  
**Severity:** Medium — helps orientation.

### Gap 3: Keyboard shortcuts
**Ideal:** T = Today, M = Month, W = Week, etc.  
**Current:** None.  
**Severity:** Low — power users.

### Gap 4: Responsive week view
**Ideal:** On narrow screens, week view may need horizontal scroll or simplified layout.  
**Current:** 7 columns may squeeze on mobile.  
**Severity:** Medium — mobile UX.

---

## Recommendations

| # | Recommendation | Priority |
|---|----------------|----------|
| 1 | Add mini calendar sidebar (collapsible on mobile) | High |
| 2 | Add current-time indicator in Week/Day view | High |
| 3 | Add keyboard shortcut: T = Today | Medium |
| 4 | Improve week view mobile layout (horizontal scroll with sticky time col) | Medium |
| 5 | Add "Now" button or link to scroll to current time in Day view | Low |
