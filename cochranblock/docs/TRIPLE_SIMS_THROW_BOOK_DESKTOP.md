<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# THROW THE BOOK: Three Sequential TRIPLE SIMS — Desktop

**Viewport:** ≥769px  
**Cycles:** 3 sequential (Sim 1→2→3→4 each)  
**Date:** 2026-02-27

---

## Cycle 1: Desktop

### Sim 1: User Story
| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Full hero, clear hierarchy | ✓ |
| 2 | Nav visible | All 6 links inline | nav-links |
| 3 | Products | 2–3 cards per row | grid minmax(260px) |
| 4 | About | 2-col legacy | legacy-tab-content |
| 5 | Book | Calendar + sidebar | gcal layout |

### Sim 2: Feature Gap
| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Max width | Comfortable read | 900px | ✓ |
| Hover states | Clear feedback | accent | ✓ |
| Developer nav | Expandable | summary | ✓ |

### Sim 3: UI/UX
| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Hero CTAs | Primary filled, secondary outline ✓ |
| 2 | Product cards | Adequate spacing |
| 3 | Footer | No awkward wrap |

### Sim 4: Imagery
| Asset | Desktop | OK |
|-------|---------|-----|
| Product images | Full visibility | max-height 180px |
| Logo | Footer clear | ✓ |

---

## Cycle 2: Desktop (Deeper)

### Sim 1
| Step | Expected |
|------|----------|
| Hover nav links | Underline/color change |
| Tab switch About | Mission ↔ Credentials |
| Service cards | Details expand |

### Sim 2–4
| Check | OK |
|-------|-----|
| Content hierarchy | Headings, sections |
| Contrast | Text readable |
| Spacing | Consistent |

---

## Cycle 3: Desktop (Final)

### Sim 1
| Step | Expected |
|------|----------|
| Full flow | All pages, all CTAs |
| Print Resume | Button present |
| Copy buttons (Source) | If on dev page |

### Sim 2–4
Consolidated. UI polish: typography, spacing, visual hierarchy.

---

## Implementation Summary

| # | Item | Done |
|---|------|------|
| 1 | Nav inline desktop | ✓ |
| 2 | Product grid 2–3 col | ✓ |
| 3 | About 2-col | ✓ |
| 4 | Hover states | ✓ |
| 5 | Button-follow tests | ✓ buttons_nav_all_200, buttons_hero_ctas_200, etc. |
