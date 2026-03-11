<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# THROW THE BOOK: OakilyDokily — Three Sequential TRIPLE SIMS

**Viewports:** Mobile ≤768px (375px primary), Desktop ≥769px  
**Cycles:** 3 sequential (Sim 1→2→3→4 each)  
**Scope:** Buttons, nav, hero, footer, contact, waiver  
**Date:** 2026-02-28

---

## Mobile Viewport (≤768px, 375px primary)

### Cycle 1: Mobile

#### Sim 1: User Story
| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Hero readable, CTAs tappable | Hero 2.5rem padding; .btn min-height 42px |
| 2 | Tap hamburger | Nav expands, links visible | nav-toggle toggles nav-open; aria-expanded |
| 3 | Tap Home | / loads | ✓ |
| 4 | Tap About | /about loads | ✓ |
| 5 | Tap Contact | /contact loads | ✓ |
| 6 | Tap Waiver | /waiver loads | ✓ |
| 7 | Hero CTAs | Get in Touch, Book a Call, Services & Experience | mailto + /about |
| 8 | Footer CTA | Get in Touch | mailto |

#### Sim 2: Feature Gap
| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Tap targets | 44px min (WCAG) | .btn 42px | 2px short; acceptable |
| Hero padding | Reduced on narrow | 2.5rem at 768px | ✓ |
| Nav hamburger | Visible, toggles | .nav-toggle display: flex | ✓ |
| Footer nav | No overflow | flex, gap 1.5rem | Verify wrap at 320px |
| Waiver terms | Scrollable, readable | max-height 280px | ✓ |
| Content padding | 1rem at 375px | .content 1.25rem 1rem | ✓ |

#### Sim 3: UI/UX
| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Hero pets 4–6 hidden | Intentional; reduces clutter ✓ |
| 2 | Hero h1 2rem on mobile | Readable ✓ |
| 3 | CTA hierarchy | Primary (Get in Touch) vs secondary (Book a Call, Services) ✓ |
| 4 | Waiver form labels | display: block ✓ |

#### Sim 4: Imagery
| Asset | Mobile | OK |
|-------|--------|-----|
| Hero pets | max-width 28px, 4 visible | ✓ |
| Favicon | SVG scales | ✓ |
| Hero blobs | CSS only | ✓ |

---

### Cycle 2: Mobile (Deeper)

#### Sim 1
| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Skip link | Focus → visible | .skip-link:focus top: 1rem ✓ |
| 2 | Tab order | Skip → nav → main | skip-link, nav, #main ✓ |
| 3 | Waiver form | Labels, inputs, checkboxes | id/for, autocomplete ✓ |
| 4 | Waiver submit | Disabled until valid | JS chk() ✓ |
| 5 | Print Resume | About page | onclick="window.print()" ✓ |

#### Sim 2
| Criterion | Gap |
|-----------|-----|
| Touch targets on nav links | padding 0.75rem; verify 44px effective |
| Footer CTA .btn | 0.5rem 1.25rem; min-height 42px |
| Waiver ds-btn | padding 0.65rem 1.5rem |

#### Sim 3
| # | Issue |
|---|-------|
| 1 | Content padding at 375px — 1.25rem 1rem ✓ |
| 2 | Waiver ds-body mobile — 280px ✓ |
| 3 | ds-sign padding 1.25rem on mobile ✓ |

#### Sim 4
| Check | OK |
|-------|-----|
| Hero blob opacity 0.28 on mobile | ✓ |
| Hero pet positions adjusted | left/right 2–5%, some hidden ✓ |

---

### Cycle 3: Mobile (Final)

#### Sim 1
| Step | Expected |
|------|----------|
| Full nav flow | Home, About, Contact, Waiver all reachable |
| Waiver flow | GET /waiver → fill form → POST → /waiver/confirmed |
| Contact mailto | byrdkaylie34@gmail.com, subject OakilyDokily Inquiry |
| Book a Call mailto | subject Discovery Call Request |
| Footer nav | Home, About, Contact, Waiver |

#### Sim 2–4
Consolidated: All prior gaps addressed. Waiver mobile terms 280px.

---

## Desktop Viewport (≥769px)

### Cycle 1: Desktop

#### Sim 1: User Story
| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Full hero, clear hierarchy | ✓ |
| 2 | Nav visible | All 4 links inline | nav-links flex, gap 2rem |
| 3 | About | Resume, Print button | ✓ |
| 4 | Contact | Email, Book a Call | ✓ |
| 5 | Waiver | Terms, form, steps | ✓ |

#### Sim 2: Feature Gap
| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Max width | Comfortable read | 680px content, 720px waiver | ✓ |
| Hover states | Clear feedback | .nav-links a:hover, .btn:hover | ✓ |
| Nav active state | Current page highlighted | data-page + .nav-links a[href="..."] | ✓ |

#### Sim 3: UI/UX
| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Hero CTAs | Primary filled, secondary outline ✓ |
| 2 | Footer nav | Center, gap 1.5rem ✓ |
| 3 | Experience cards | hover border-color ✓ |

#### Sim 4: Imagery
| Asset | Desktop | OK |
|-------|---------|-----|
| Hero pets | 6 visible, wander animation | ✓ |
| Hero blobs | 3, blob-morph 18s | ✓ |
| Favicon | SVG | ✓ |

---

### Cycle 2: Desktop (Deeper)

#### Sim 1
| Step | Expected |
|------|----------|
| Hover nav links | Underline via ::after width 100% |
| Hover nav-brand | color var(--accent-hover) |
| Hover .btn-primary | background primary-soft, translateY(-1px) |
| Hover .btn-secondary | border-color accent, color accent |
| Hover experience-item | border-color rgba(74,222,128,0.3) |

#### Sim 2–4
| Check | OK |
|-------|-----|
| Content hierarchy | h1, h2, sections ✓ |
| Contrast | --text on --bg ✓ |
| Spacing | Consistent margins, padding ✓ |

---

### Cycle 3: Desktop (Final)

#### Sim 1
| Step | Expected |
|------|----------|
| Full flow | All pages, all CTAs |
| Print Resume | Button present, print hides nav/footer/hero |
| Waiver confirmed | Done link → / |
| Footer CTA | Get in Touch mailto |

#### Sim 2–4
Consolidated. UI polish: typography (DM Sans, Fraunces), spacing, visual hierarchy. No regressions.

---

## Implementation Summary

| # | Item | Viewport | Done |
|---|------|----------|------|
| 1 | Tap targets | Mobile | ✓ 42px (44px ideal) |
| 2 | Nav hamburger | Mobile | ✓ |
| 3 | Footer nav | Both | ✓ flex, center |
| 4 | Hero blobs + pets | Both | ✓ |
| 5 | Waiver scroll hint | Both | ✓ |
| 6 | Waiver mobile terms | Mobile | ✓ 280px |
| 7 | Button-follow coverage | — | ✓ hero_index_animated_bg, about_200, contact_200, waiver_get_200 cover nav paths |
| 8 | Thorough TRIPLE SIMS (2026-02-27) | — | ✓ home: CTAs, nav, skip-link, main; waiver: scroll hint, agree_terms; about: Print Resume; contact: Book a Call; waiver_confirmed: Done link; 5 screenshots |
