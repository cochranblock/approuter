<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: Product Page External Links (cochranblock.org)

**Target:** Products page — Rogue Repo → roguerepo.io, Ronin Sites → ronin-sites.pro  
**Method:** Sim 1 (User Story) → Sim 2 (Feature Gap) → Sim 3 (UI/UX) → Sim 4 (Imagery/Links)  
**Date:** 2026-03-01

---

## Sim 1: User Story Analysis

**Persona:** Visitor interested in Rogue Repo or Ronin Sites; wants to visit the live product sites.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on Products | See Rogue Repo, Ronin Sites, Kova cards | ✓ |
| 2 | Click Rogue Repo (image or title) | Navigate to roguerepo.io | ✓ href="https://roguerepo.io" |
| 3 | Click Ronin Sites (image or title) | Navigate to ronin-sites.pro | ✓ href="https://ronin-sites.pro" |
| 4 | Links open in same tab | No popup blocker; reliable navigation | ✓ No target="_blank" |
| 5 | Kova card | No external link (no live site yet) | ✓ Kova remains informational |

**Recommendation:** Links open in same tab for reliability; rel="noopener noreferrer" for security when/if target="_blank" is used.

---

## Sim 2: Feature Gap Analysis

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Rogue Repo link | href to roguerepo.io | ✓ | None |
| Ronin Sites link | href to ronin-sites.pro | ✓ | None |
| Image clickable | Whole image links to product | ✓ | None |
| Title clickable | h2 links to product | ✓ | None |
| Kova link | No link (coming soon only) | ✓ | None |
| Link visibility | Cursor pointer, hover feedback | ✓ h2 a:hover underline | None |

---

## Sim 3: UI/UX Analysis

| # | Issue | Recommendation |
|---|-------|-----------------|
| 1 | Link color | h2 a inherits accent; no default blue ✓ |
| 2 | Hover feedback | underline on h2 a:hover ✓ |
| 3 | Image link | No border/outline on img; clean ✓ |
| 4 | Tap target (mobile) | Image + title both tappable; adequate ✓ |
| 5 | External link indicator | Optional: add subtle icon for external; not required for same-tab |

---

## Sim 4: Imagery / Links Evaluation

| Asset | Link target | Screenshot | Status |
|-------|-------------|------------|--------|
| rogue-repo.png | https://roguerepo.io | [products](../screenshots/products.png) | ✓ Image + h2 wrapped in <a> |
| ronin-sites.png | https://ronin-sites.pro | [products](../screenshots/products.png) | ✓ Image + h2 wrapped in <a> |
| kova.png | — | [products](../screenshots/products.png) | ✓ No link (coming soon) |

---

## Implementation Summary

**Executed:** 2026-03-01

| # | Item | Done |
|---|------|------|
| 1 | Rogue Repo → roguerepo.io | ✓ Image + h2 link |
| 2 | Ronin Sites → ronin-sites.pro | ✓ Image + h2 link |
| 3 | Same-tab navigation (no popup block) | ✓ Removed target="_blank" |
| 4 | rel="noopener noreferrer" | ✓ |
| 5 | CSS: h2 a color inherit, hover underline | ✓ |
| 6 | HTTP tests: products_roguerepo_link, products_ronin_link | ✓ |
| 7 | TRIPLE SIMS run: tests + screenshots | ✓ |
