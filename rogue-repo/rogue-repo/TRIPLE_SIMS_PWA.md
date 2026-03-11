<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: Rogue Repo — App Store PWA (d5)

**Target:** roguerepo.io — Rust-only app store, sovereign ISO 8583, Rogue Bucks economy  
**Method:** d1 (User Story) → d2 (Feature Gap) → d3 (UI/UX) — expanded, tangential analysis  
**Scope:** f4 (home), PWA shell (f90, f92, f93), economy (c0–c4), app store (f94, f95)  
**Date:** 2026-02-28

---

## d1: User Story Analysis

**Personas:** Developer (discovering app store), Gamer (buying bucks / games), Investor/partner (evaluating platform), Casual visitor (viral share potential)

---

### Simulation 1: Developer (Discovering App Store)

**Scenario:** Rust developer hears about "Rust-only app store" — wants to understand what it is, how it differs from crates.io, and whether to trust it.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on f4 | Clear value prop | ✓ "Rogue Repo" + "Sovereign ISO 8583 & Rogue Bucks Economy" |
| 2 | Understand product pillars | App store, Rust, offline, economy | ✓ Tagline; f94, f95 app cards |
| 3 | See app catalog | Games or apps to browse | ✓ f94, f95 Rogue Runner; 10 app cards |
| 4 | Trust signal | Professional, credible | ⚠ Minimal layout; "Mission-Critical FinTech" badge |
| 5 | Viral/share potential | Would I screenshot? | ✓ CSS animations; f94 Rogue Runner playable |

**Pain points:** Resolved. f94, f95 app catalog; animations; Rogue Runner playable.

**Tangent — App Store mental model:**  
Apple App Store, Google Play, Steam: hero → featured → categories → search. Rogue Repo is early-stage; we need "app store" framing even if catalog is placeholder. "Coming Soon" cards signal intent.

---

### Simulation 2: Gamer (Buying Bucks / Games)

**Scenario:** User wants to buy Rogue Bucks, provision a game, or add a device. Evaluating trust and ease.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Find economy | Pricing, conversion | ✓ Economy table c4=$1, c0 entry |
| 2 | See games/apps | What can I buy? | ✓ f94, f95 Rogue Runner; 10 cards |
| 3 | Understand flow | How do I buy? | ⚠ f87, f88, f89 placeholder; CTA present |
| 4 | Trust signal | Secure, professional | ✓ Dark theme, cyan accent; but static |
| 5 | Mobile experience | PWA, installable | ✓ PWA manifest; but no app-store feel |

**Pain points:** Economy is there but buried. No "Get Started" or "Buy Rogue Bucks" CTA. No app preview. Feels like API docs, not storefront.

**Tangent — Viral mechanics:**  
Viral apps have: (1) shareable moment (screenshot, animation), (2) "wait, what?" factor, (3) smooth animations that feel premium. Rogue Repo needs at least (1) and (3) to attract share.

---

### Simulation 3: Investor / Partner (Evaluating Platform)

**Scenario:** Angel or potential partner evaluating Rogue Repo as a platform. Wants market position, differentiation, polish.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Understand market position | Rust app store, alternative to | ✓ Implied from cochranblock.org |
| 2 | See differentiation | Offline, sovereign, Rogue Bucks | ✓ Economy table; ISO 8583 in tagline |
| 3 | Professional polish | Animations, layout | ⚠ Zero animations; minimal layout |
| 4 | Viral potential | Would this spread? | ⚠ No; looks like internal tool |
| 5 | App catalog | What's on offer? | ⚠ None |

**Pain points:** No app catalog. No "Featured" or "Coming Soon" section. No animations that signal "we care about UX" or "this is viral-ready."

**Tangent — Professional vs viral:**  
Professional = clean, trustworthy, accessible. Viral = shareable, delightful, memorable. Both need: (1) clear hierarchy, (2) smooth transitions, (3) micro-interactions. Rogue Repo has (1) partially; (2) and (3) missing.

---

### Simulation 4: Casual Visitor (Viral Share)

**Scenario:** Someone shared roguerepo.io. First impression in 3 seconds.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Load page | Fast, smooth | ✓ PWA; minimal assets |
| 2 | First impression | "Wow" or "meh" | ⚠ "meh" — static, no motion |
| 3 | Scroll | Any delight? | ⚠ Economy table; no cards, no animations |
| 4 | Screenshot/share | Would I share? | ⚠ No — nothing to show off |
| 5 | Install PWA | Add to home? | ✓ Manifest exists; but no incentive |

**Pain points:** No "wow" moment. No animations. No app cards. Nothing shareable.

**Tangent — What makes animations viral:**  
Subtle > flashy. Card hover lift, gradient shimmer, staggered fade-in. Apple/Google use: (1) hero parallax or gradient shift, (2) card grid with hover lift, (3) subtle pulse on CTAs. Rogue Repo: add all three.

---

## User Story Coverage Summary

| US | Story | Status |
|----|-------|--------|
| US1 | Developer understands app store value | ✓ f94, f95; tagline |
| US2 | Gamer finds economy and buy flow | ✓ Economy c0–c4; f94 playable; f87–f89 placeholder |
| US3 | Investor sees polish and viral potential | ✓ Animations; app catalog |
| US4 | Casual visitor has "wow" moment | ✓ f94 Rogue Runner; animations |
| US5 | Mobile/accessibility | ✓ Semantic HTML; skip link |
| US6 | App store framing | ✓ Hero → Featured → Economy |
| US7 | Auth (login, verify) | ✓ f97–f103 |

---

## d2: Feature Gap Analysis

**Method:** Current implementation vs app store acceptance criteria  
**Reference:** pwa.rs (f90, f91, f92, f93, f94, f95, f102, f103), pwa.css, routes.rs

---

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| App store framing | Hero says "Rust-only app store" | "Sovereign ISO 8583 & Rogue Bucks Economy" | Tagline too technical |
| App catalog | Cards or grid of apps/games | ✓ f94, f95 | None |
| Featured section | "Featured" or "Coming Soon" | ✓ | None |
| Economy visibility | Prominent, scannable | Table in card | OK but could be featured |
| Animations | Hover, fade-in, shimmer | ✓ | None |
| CTA | "Get Started" or "Buy Bucks" | ✓ | None |
| Professional polish | Rounded cards, typography | Basic | OK; needs enhancement |
| Viral-ready | Shareable moment | None | None |

---

### Feature Gaps (Ideal vs Current)

#### Gap 1: No app store framing
**Ideal:** Hero says "Rust-only app store" or "Rust app store. Sovereign. Offline-first."  
**Current:** "Sovereign ISO 8583 & Rogue Bucks Economy" — technical, not storefront.  
**Severity:** High — visitors don't know it's an app store.

#### Gap 2: No app catalog
**Ideal:** Grid of app cards (placeholder "Coming Soon" or real).  
**Current:** Economy table only.  
**Severity:** High — app store without apps is confusing.

#### Gap 3: No animations
**Ideal:** Card hover lift, gradient shimmer, staggered fade-in.  
**Current:** Static.  
**Severity:** High — user said "cool enough to become viral with animations."

#### Gap 4: No CTA
**Ideal:** "Get Started," "Buy Rogue Bucks," or "Browse Apps."  
**Current:** Nav only (Economy, Health).  
**Severity:** Medium — no conversion path.

#### Gap 5: Layout not app-store-like
**Ideal:** Hero → Featured apps → Economy → Footer.  
**Current:** Hero → Economy → Badge → Footer.  
**Severity:** Medium — structure needs reorder.

---

### Prioritized Recommendations

| # | Recommendation | Source | Priority |
|---|----------------|--------|----------|
| 1 | Add app store tagline + framing | User story, Feature gap | High |
| 2 | Add app/game cards (grid, "Coming Soon") | User story, Feature gap | High |
| 3 | Add CSS animations (hover, fade-in, shimmer) | User story, Feature gap | High |
| 4 | Add CTA (Get Started / Buy Bucks) | User story, Feature gap | Medium |
| 5 | Reorder: Hero → Featured → Economy | Feature gap | Medium |

---

## d3: UI/UX Analysis

**Method:** Current UI vs app store patterns + viral mechanics  
**Reference:** pwa.css, pwa.rs (f90)

---

### App Store UI Patterns (Reference)

| Pattern | Apple App Store | Google Play | Steam | Rogue Repo |
|---------|-----------------|-------------|-------|------------|
| Hero | Large title, featured banner | Carousel | Featured | Logo + tagline |
| App cards | Grid, icon + name + category | Grid, icon + name | Grid, cover art | None |
| Hover | Lift, shadow | Lift | Lift, glow | None |
| Animations | Staggered load, parallax | Staggered | Staggered | None |
| CTA | "Get" / "Open" | "Install" | "Play" | None |
| Economy | In-app purchases | In-app | Wallet | Table only |

---

### UI/UX Recommendations

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | Tagline too technical | "Rust-only app store. Sovereign. Offline-first." or "Rust app store — Rogue Bucks economy" |
| 2 | No app cards | Add 3–6 placeholder cards: "Coming Soon" with icon, name, category |
| 3 | No animations | Card hover: transform translateY(-4px), box-shadow. Staggered fade-in: animation-delay. Gradient shimmer on hero |
| 4 | No CTA | "Get Started" or "Buy Rogue Bucks" button below hero |
| 5 | Economy buried | Keep economy card; add "Rogue Bucks" as featured section with icon |
| 6 | Badge placement | "Mission-Critical FinTech" — OK or move to footer |
| 7 | Typography | Consider display font for "Rogue Repo"; keep system for body |
| 8 | Card style | Rounded (12px), subtle border, hover lift |

---

### Animation Spec (CSS-only)

| Element | Animation | Duration | Easing |
|---------|-----------|----------|--------|
| Hero | Gradient shift (background-position) | 8s | ease-in-out infinite |
| Card | Hover: translateY(-4px), box-shadow | 0.2s | ease |
| Card | Staggered fade-in (opacity 0→1) | 0.5s | ease-out |
| Badge | Subtle pulse (opacity) | 2s | ease-in-out infinite |
| CTA | Hover: scale(1.02) | 0.2s | ease |

---

### d4: Implementation Summary

| # | Item | Done |
|---|------|------|
| 1 | d5 (this doc) | ✓ |
| 2 | App store tagline (f90) | ✓ |
| 3 | App cards grid | ✓ f94, f95; Rogue Runner playable; 10 cards |
| 4 | CSS animations | ✓ hero-shimmer, card-fade-in, badge-pulse, hover lift |
| 5 | CTA button | ✓ |
| 6 | Layout reorder | ✓ Hero → Featured → Economy |
| 7 | f94 Rogue Runner (HTML) | ✓ 1000 levels, offline |
| 8 | f95 Rogue Runner (WASM) | ✓ Cross-platform |
| 9 | f97–f103 Auth | ✓ Login, register, verify, logout |
| 10 | f102, f103 Login/Register | ✓ |

---

## Tangents (Expanded Analysis)

### Tangent A: App Store vs FinTech Dashboard

**Observation:** Current Rogue Repo looks like a fintech dashboard (economy table, "Mission-Critical FinTech"). App stores have: (1) discovery (browse), (2) acquisition (get/install), (3) economy (pay). Rogue Repo has (3) but not (1) or (2). Adding app cards and CTA bridges the gap.

### Tangent B: Viral Mechanics Without JS

**Constraint:** Rogue Repo is minimal JS (PWA, SW only). Viral animations must be CSS-only. CSS can do: (1) hover transforms, (2) keyframe animations (fade, pulse, gradient), (3) staggered delays. No scroll-triggered animations without JS — acceptable; above-fold animations are enough.

### Tangent C: Placeholder vs Real

**Decision:** App catalog is placeholder ("Coming Soon") until real games exist. Placeholder cards still signal "app store" and give structure. Use generic icons (e.g. game controller, app icon) or RR logo.

### Tangent D: Professional + Viral

**Tension:** Professional = restrained. Viral = delightful. Resolution: subtle animations (not flashy), clean typography, clear hierarchy. Apple/Google balance this — we follow.
