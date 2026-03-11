<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# THROW THE BOOK: Three Sequential TRIPLE SIMS — Mobile

**Viewport:** ≤768px, 375px primary  
**Cycles:** 3 sequential (Sim 1→2→3→4 each)  
**Date:** 2026-02-27

---

## Cycle 1: Mobile

### Sim 1: User Story
| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Hero readable, CTAs tappable | Hero 3rem; .btn 44px |
| 2 | Tap hamburger | Nav expands, links visible | nav-toggle toggles nav-open |
| 3 | Tap Services | /services loads | ✓ |
| 4 | Tap Products | /products loads | ✓ |
| 5 | Tap Book | Calendar usable, slots tappable | 44px min on day/slot |

### Sim 2: Feature Gap
| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Tap targets | 44px min | .btn has it | ✓ |
| Hero padding | Reduced on narrow | 3rem at 768px | ✓ |
| Product cards | Single column | grid auto-fit | ✓ |
| Footer nav | No overflow | flex-wrap at 375px | ✓ |

### Sim 3: UI/UX
| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Hero may feel cramped | Verify line-height, spacing |
| 2 | Product images | aspect-ratio 4/3 ✓ |
| 3 | CTA hierarchy | Primary vs secondary clear |

### Sim 4: Imagery
| Asset | Mobile | OK |
|-------|--------|-----|
| Product images | 400×300, object-fit | ✓ |
| Favicon/logo | SVG scales | ✓ |

---

## Cycle 2: Mobile (Deeper)

### Sim 1
| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Scroll hero → services | Smooth, no jump | scroll-reveal |
| 2 | Tab order | Skip link, nav, main | skip-link present |
| 3 | Form on contact | Labels, inputs usable | ✓ |

### Sim 2
| Criterion | Gap |
|-----------|-----|
| Touch targets on calendar | 44px ✓ |
| Footer CTA tap area | .btn 44px ✓ |

### Sim 3
| # | Issue |
|---|-------|
| 1 | Content padding at 375px — 1rem ✓ |
| 2 | About tabs — stacked on mobile ✓ |

### Sim 4
| Check | OK |
|-------|-----|
| Product images load | ✓ |
| Logo in footer | ✓ |

---

## Cycle 3: Mobile (Final)

### Sim 1
| Step | Expected |
|------|----------|
| Full nav flow | All 6 pages reachable |
| Book calendar | Prev/next, dates, slots |
| Contact mailto | Valid mailto links |

### Sim 2–4
Consolidated: All prior gaps addressed. Verify no regressions.

---

## Implementation Summary

| # | Item | Done |
|---|------|------|
| 1 | Tap targets 44px | ✓ |
| 2 | Footer flex-wrap | ✓ |
| 3 | Product aspect-ratio | ✓ |
| 4 | Nav hamburger | ✓ |
| 5 | Button-follow tests | ✓ buttons_nav_all_200, buttons_hero_ctas_200, etc. |
