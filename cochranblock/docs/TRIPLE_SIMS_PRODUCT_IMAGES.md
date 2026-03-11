<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: Product Images (cochranblock.org)

**Target:** Product card images — Rogue Repo, Ronin Sites, Kova  
**Method:** Sim 1 (User Story) → Sim 2 (Feature Gap) → Sim 3 (UI/UX)  
**Date:** 2026-02-27

---

## Sim 1: User Story Analysis

**Persona:** Visitor scanning Products page for value and differentiation.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | See product cards | Each product has a visual that conveys its purpose | Current: generic/placeholder images |
| 2 | Aesthetic coherence | Images match site (cosmic, cyber, dark) | Gap: images may clash with theme |
| 3 | Brand recognition | CochranBlock visual language consistent | Gap: product images not tied to site palette |

**Recommendation:** Product images must use cochranblock.org palette (--bg #050508, --accent #00d9ff, --purple #9d4edd, --orange #ff6b35, --cyber-teal #00ffcc) and cosmic/cyber aesthetic.

---

## Sim 2: Feature Gap Analysis

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Rogue Repo image | Rust/offline/enterprise visual | Placeholder | Needs cosmic-themed SaaS visual |
| Ronin Sites image | Shop/creator/subdomain visual | Placeholder | Needs cosmic-themed platform visual |
| Kova image | AI/execute/flow visual | Placeholder | Needs cosmic-themed AI visual |
| Aspect ratio | 400×300 (product card) | — | All images must be 400×300 |
| File format | PNG | PNG | OK |

**Recommendation:** Generate three product images: (1) Rogue Repo — Rust, offline, enterprise; (2) Ronin Sites — shop, creators, subdomains; (3) Kova — AI, execution, flow. All dark bg, accent/purple/teal accents.

---

## Sim 3: UI/UX Analysis

**Context:** Product cards use .product-img (max-height 180px, object-fit cover, border-radius). Cards have dark gradient bg, accent border, cyber-teal headings.

| # | Issue | Recommendation |
|---|-------|-----------------|
| 1 | Images may clash with dark cards | Use dark backgrounds (#050508–#0d0d14) in images |
| 2 | Accent colors should echo CSS | Use #00d9ff, #9d4edd, #00ffcc, #ff6b35 in imagery |
| 3 | Visual hierarchy | Product name/icon prominent; no competing bright elements |
| 4 | Consistency | All three images share same visual language |

**Recommendation:** Each image: dark cosmic background, subtle starfield or gradient, product-specific icon/mark in accent colors. No bright white or off-palette elements.

---

## Implementation Summary

**Executed:** 2026-02-27

| # | Item | Done | Screenshot |
|---|------|------|------------|
| 1 | rogue-repo.png | ✓ Regenerated (TRIPLE SIMS); heavy black, single cyan #00d9ff; app store concept | [products](../screenshots/products.png) |
| 2 | ronin-sites.png | ✓ Generated; cosmic theme; shop/creator visual | [products](../screenshots/products.png) |
| 3 | kova.png | ✓ Generated; cosmic theme; AI/execute visual | [products](../screenshots/products.png) |
| 4 | Applied to assets/img/ | ✓ | — |
| 5 | Rebuild + tests × 3 | ✓ | — |
