<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: Mobile & Desktop Viewports

**Target:** cochranblock.org — mobile (≤768px) and desktop (≥769px)  
**Method:** Sim 1 → Sim 2 → Sim 3 → Sim 4 per viewport  
**Date:** 2026-02-27

---

## Mobile Viewport (≤768px, 375px primary)

### Sim 1: User Story (Mobile)

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Hero readable, CTAs tappable | ✓ Hero 3rem padding; verify tap targets |
| 2 | Navigate | Hamburger works, links accessible | ✓ nav-toggle; nav-links collapse |
| 3 | Products page | Product cards stack; images legible | ✓ grid auto-fit → 1 col; product-img 180px |
| 4 | Book a call | Calendar + slots usable | ✓ 44px min on day/slot; single col slots |
| 5 | Contact | Form usable, links tappable | ✓ Form full-width; verify CTA tap |

**Gap:** Hero CTAs, footer links, product card tap areas — verify 44px min.

---

### Sim 2: Feature Gap (Mobile)

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Tap targets | 44px min (WCAG) | .btn ~24px height | Add min-height: 44px on mobile |
| Product images | Aspect ratio preserved | object-fit: cover, max-height 180px | OK; consider aspect-ratio for consistency |
| Hero padding | Less on narrow | 3rem at 768px | ✓ |
| Footer nav | No awkward wrap | May wrap on 375px | Add flex-wrap or stacked |
| Product card padding | Comfortable tap | 1.5rem | OK |

---

### Sim 3: UI/UX (Mobile)

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | .btn tap target | min-height: 44px; padding 0.75rem 1.5rem on mobile |
| 2 | Product card tap | Ensure whole card or CTA is tappable; min 44px |
| 3 | Footer nav | Stack or horizontal scroll on ≤375px |
| 4 | Content padding | .content 1rem at 375px ✓ |

---

### Sim 4: Imagery (Mobile)

| Asset | Mobile concern | Screenshot | Recommendation |
|-------|----------------|------------|----------------|
| rogue-repo.png | Legible at small size | [products](../screenshots/products.png) | 400×300; object-fit cover — OK |
| ronin-sites.png | Same | [products](../screenshots/products.png) | OK |
| kova.png | Same | [products](../screenshots/products.png) | OK |
| Favicon/logo | Readable in nav | [index](../screenshots/index.png) | ✓ SVG scales |

---

## Desktop Viewport (≥769px)

### Sim 1: User Story (Desktop)

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Full hero, clear hierarchy | ✓ |
| 2 | Navigate | All links visible | ✓ nav-links inline |
| 3 | Products | 2–3 cards per row | ✓ grid minmax(260px) |
| 4 | About tabs | Side-by-side legacy | ✓ 2-col at 769px |
| 5 | Book | Calendar + sidebar | ✓ gcal layout |

---

### Sim 2: Feature Gap (Desktop)

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Product grid | 2–3 cols | auto-fit minmax(260px) | ✓ |
| Max width | Comfortable read | 900px | ✓ |
| Hover states | Clear feedback | accent border/color | ✓ |
| Developer nav | Expandable | summary + sub-links | ✓ |

---

### Sim 3: UI/UX (Desktop)

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Product cards | Adequate spacing; hover subtle | OK |
| 2 | Hero CTAs | Primary vs secondary clear | ✓ |
| 3 | Footer | Multi-col or inline | Check wrap |

---

### Sim 4: Imagery (Desktop)

| Asset | Desktop concern | Screenshot | Recommendation |
|-------|-----------------|------------|----------------|
| Product images | Full 400×300 visible | [products](../screenshots/products.png) | max-height 180px crops; consider aspect-ratio 4/3 |
| Logo | Footer clear | [index](../screenshots/index.png) | ✓ |

---

## Implementation Summary

**Executed:** 2026-02-27  
**TRIPLE SIMS run:** 2026-02-27 — @t @b @go (79 tests pass, 7 screenshots)

| # | Item | Viewport | Done |
|---|------|----------|------|
| 1 | Rogue Repo image | Both | ✓ Fixed (app store, heavy black, cyan) |
| 2 | .btn min-height 44px, padding 0.75rem | Mobile ≤768px | ✓ |
| 3 | Product card tap area | Mobile | N/A — cards informational; CTAs have 44px |
| 4 | Footer nav narrow | Mobile ≤375px | ✓ flex-wrap, 44px min tap on links |
| 5 | Product img aspect-ratio | Both | ✓ 4/3 for consistency |
