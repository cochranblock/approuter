<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# UI/UX Analysis Simulation — Booking Page

**Based on:** screenshots/book.png (captured 2026-02-26)  
**Viewport:** 1200×900 (screenshot config)  
**Context:** Braintrust discovery call booking

---

## Visual Hierarchy

| Element | Current | Assessment |
|---------|---------|------------|
| H1 "Schedule a Discovery Call" | Blue-green gradient, large | ✓ Strong primary focus |
| Intro text | Gray, smaller | ✓ Appropriate secondary |
| Braintrust context | Italic, muted | ✓ Credibility without competing |
| Calendar wrapper | Dark card, subtle border | ✓ Contained; could use more elevation |
| Month header | "February 2026" white | ✓ Clear |
| Nav arrows | Square buttons, accent | ✓ Recognizable |
| Weekday headers | Small, muted | ✓ Standard |
| Date cells | Numbers; available = subtle; unavailable = faded | ⚠ Available vs unavailable contrast weak |
| Selected date (26) | Purple border | ✓ Visible |
| Footer text | Muted, "Email me" link | ✓ Clear fallback |

---

## Layout & Spacing

| Area | Observation |
|------|-------------|
| Max-width | 520px — narrow; may feel cramped on desktop |
| Calendar padding | 1.25rem — adequate |
| Grid gap | 0.25rem — tight; dates could use more breathing room |
| Time panel | Not visible in screenshot (below fold or hidden until date selected) |
| Vertical rhythm | Intro → context → calendar → note → fallback — logical |

---

## Color & Contrast

| Element | Colors | Issue |
|---------|--------|-------|
| Available dates | White text, subtle accent border | Border may not read on all displays |
| Unavailable dates | opacity 0.35, muted | May look "loading" or "disabled" — needs stronger disabled cue |
| Today | Purple border | ✓ Distinct |
| Selected | Accent outline + glow | ✓ Good |
| Time slots (when visible) | Accent, gradient bg | Assumed from CSS |

---

## Typography

| Element | Font | Size | Assessment |
|---------|------|------|------------|
| H1 | Orbitron | 1.75rem | ✓ On-brand |
| Intro | Rajdhani | 1.1rem | ✓ Readable |
| Context | Rajdhani | 0.9rem | ✓ |
| Month | Orbitron | 1.1rem | ✓ |
| Weekdays | Orbitron | 0.7rem | Small but acceptable |
| Date numbers | Rajdhani | 0.9rem | ✓ |

---

## Interactivity Cues

| Element | Cue | Assessment |
|---------|-----|------------|
| Available dates | Cursor pointer, border | ⚠ Could be stronger (e.g. dot, checkmark) |
| Unavailable dates | Disabled, muted | ⚠ Needs clearer "not available" treatment |
| Nav buttons | Hover state | ✓ |
| Time slots | Link styling | ✓ (from CSS) |

---

## Accessibility

| Check | Status |
|-------|--------|
| aria-label on calendar | ✓ |
| aria-label on nav buttons | ✓ |
| role="grid", role="gridcell" | ✓ |
| Skip to main | ✓ (global) |
| Skip to calendar | ⚠ Not present |
| Visible keyboard hint | ⚠ "Select a date, then a time" not shown |

---

## Recommendations (UI/UX Simulation)

1. **Available-day badge** — "4 available" or "Tue & Thu" in month header.
2. **Legend** — "Available Tue & Thu · 2–4pm ET" above grid.
3. **Unavailable styling** — Stronger disabled state: lower opacity, optional strikethrough or "—" instead of number, or distinct background.
4. **Available styling** — Add small dot or checkmark on available dates; stronger accent border.
5. **Calendar fade-in** — `animation: booking-fade-in 0.3s ease` on load.
6. **Visible hint** — "Select a date, then a time" below context or as aria-describedby.
7. **Grid spacing** — Slightly increase gap (0.35rem–0.5rem) for breathing room.
8. **Skip to calendar** — Add anchor and optional skip link for keyboard users.
