<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# UI/UX Simulation — Google Calendar Clone (2026)

**Analysis date:** 2026-02-25  
**Context:** Full calendar with Month, Week, Day, Agenda views  
**Reference:** pages.rs, calendar.js, main.css

---

## Visual Hierarchy

| Element | Current | Assessment |
|---------|---------|------------|
| Toolbar (Today, prev/next, date label) | Left-aligned, Orbitron | ✓ Clear |
| View switcher (Month/Week/Day/Agenda) | Right-aligned, tab-style | ✓ Standard |
| Month grid | 7-column, weekday headers | ✓ Google-like |
| Week grid | Time column + 7 day columns | ✓ Time-slot layout |
| Day grid | Time column + single day | ✓ Single-day focus |
| Agenda list | Date + time slots per row | ✓ List format |
| Time panel | Below calendar, mailto slots | ✓ Visible when date selected |

---

## Layout & Spacing

| Area | Observation |
|------|-------------|
| Max-width | 900px — improved from 520px |
| Toolbar | Flex wrap, gap 1rem — good |
| Month grid gap | 0.35rem — adequate |
| Week/Day time slots | 48px height — readable |
| Agenda items | Grid 140px + 1fr — balanced |

---

## Interactivity Cues

| Element | Cue | Assessment |
|---------|-----|------------|
| View buttons | .active = accent bg | ✓ Clear |
| Today button | Hover accent | ✓ |
| Nav arrows | Hover accent | ✓ |
| Available dates (month) | Accent border, pointer | ✓ |
| Week/Day available cells | Accent links | ✓ |
| Agenda slots | Inline links | ✓ |

---

## Accessibility

| Check | Status |
|-------|--------|
| aria-label on toolbar | ✓ |
| aria-label on nav | ✓ |
| role="tablist" on view group | ✓ |
| role="tabpanel" on panes | ✓ |
| aria-selected on view tabs | ✓ (JS) |
| aria-hidden on inactive panes | ✓ (JS) |
| Skip to calendar | ✓ #gcal-main |
| role="grid" on month | ✓ |

---

## Recommendations (UI/UX Simulation)

1. **Current-time indicator** — In Week/Day view, show a horizontal line at current time.
2. **Mini calendar sidebar** — Optional collapsible mini calendar for date pick (Google has this).
3. **Loading state** — Brief fade-in when switching views (already smooth).
4. **Focus management** — When switching views, move focus to active pane for screen readers.
5. **Touch targets** — Ensure 44px minimum on mobile for view buttons and date cells.
6. **Week view day headers** — Make clickable day headers more obviously interactive (underline on hover).
7. **Agenda empty state** — Already present; verify styling.
8. **Keyboard nav** — Arrow keys to move between dates in month view (enhancement).
