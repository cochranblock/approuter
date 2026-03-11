<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Triple Sims: CochranBlock (cochranblock.org)

**Target:** CochranBlock business site — Rust SaaS + consulting  
**Method:** Sequential analysis (User Story → Feature Gap → UI/UX)  
**Date:** 2026-02-27

---

## Sim 1: User Story Analysis

**Personas:** Potential customer (product interest), Clinic/enterprise (consulting), Investor/partner (evaluating business)

---

### Simulation 1: Potential Customer (Product Interest)

**Scenario:** Developer or small team frustrated with enterprise SaaS pricing. Wants offline-first tools, creative mode, fair pricing.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on home | Clear value prop | ✓ "Rust-only SaaS that goes after the big guys — and their greed" |
| 2 | Understand product pillars | Offline, creative mode, pricing | ✓ hero-stats: Offline-first · Creative mode · Superior pricing |
| 3 | See what's built | Product details | ✓ Services page has Product section (Rust-Only SaaS, Offline-First, Superior Pricing) |
| 4 | Find contact path | How to reach out | ✓ Get in Touch, Book a Call on home and contact |
| 5 | Understand roadmap | Is product live? Coming soon? | ⚠ No explicit "coming soon" or "in development" — could confuse |

**Pain points:** No product launch status. Visitor may assume product exists today; copy says "building" but hero doesn't. No waitlist or early-access CTA for product interest.

---

### Simulation 2: Clinic / Enterprise (Consulting)

**Scenario:** CTO or eng lead needs systems engineering, vulnerability research, or integration work. Evaluating CochranBlock for consulting.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Find consulting offering | Clear consulting section | ✓ Services has "Consulting" section with Systems Engineering, Vuln Research, API |
| 2 | Understand availability | When can I engage? | ✓ "Once the product is up and running, I take on select engagements" |
| 3 | See credentials | 11+ years, enterprise | ✓ About → Credentials tab has resume; Consulting blurb mentions 11+ years |
| 4 | Book a call | Discovery call flow | ✓ /book with calendar, mailto slots |
| 5 | Trust signal | Testimonial, LinkedIn | ✓ Testimonial on contact; LinkedIn in footer |

**Pain points:** "Once the product is up" may read as "not available now" — some visitors may leave. No explicit "consulting open" vs "consulting waitlist" state. Resume is Michael's — fine for consulting, but "CochranBlock" is plural "we" in some copy, singular in others.

---

### Simulation 3: Investor / Partner (Evaluating Business)

**Scenario:** Angel or potential partner evaluating CochranBlock as a business. Wants market position, differentiation, team.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Understand market position | Who we compete with | ✓ "big guys," "enterprise greed," "reset software market" |
| 2 | See differentiation | Offline, creative mode, pricing | ✓ Clear in hero and services |
| 3 | Understand team | Who runs it | ⚠ About says "I" and "CochranBlock" — Michael is founder; no team page |
| 4 | Find traction / roadmap | Stage of business | ⚠ No "coming soon," "beta," "launch" — ambiguous |
| 5 | Contact for partnership | How to reach | ✓ Contact, Book a Call |

**Pain points:** No explicit stage (pre-revenue, bootstrapped, etc.). No product demo or screenshots — product is described but not shown. "We" vs "I" inconsistency (CochranBlock vs Michael).

---

## User Story Coverage Summary

| US | Story | Status |
|----|-------|--------|
| US1 | Customer understands product value (offline, creative mode, pricing) | ✓ Strong |
| US2 | Consulting prospect finds offering and credentials | ✓ Good; availability ambiguous |
| US3 | Investor evaluates market position and differentiation | ✓ Good; stage/team unclear |
| US4 | Visitor books a call or contacts | ✓ Book + Contact work |
| US5 | Mobile/accessibility | ✓ Semantic HTML; cosmic theme responsive |
| US6 | Product launch status clarity | ⚠ Missing — "building" vs "live" unclear |

---

## Recommendations from User Story Simulation

1. **Product status line** — Add one line: "Product in development. Consulting: limited capacity." or similar.
2. **Consulting availability** — Clarify: "Consulting open now" vs "Join waitlist" so visitors know.
3. **Consistent voice** — Decide "we" (CochranBlock) vs "I" (Michael); apply consistently.
4. **Product waitlist CTA** — If product isn't live, add "Get notified when we launch" or "Early access" for product interest.
5. **Team / founder** — About has Mission; consider brief "Founded by Michael Cochran" for investor scan.

---

## Sim 2: Feature Gap Analysis

**Method:** Current implementation vs acceptance criteria + ideal UX  
**Reference:** pages.rs, router.rs, main.css

---

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Home value prop | Rust SaaS, big guys, greed | ✓ | None |
| Product pillars | Offline, creative mode, pricing | ✓ | None |
| Consulting section | Systems, vuln research, API | ✓ | None |
| Book a call | Calendar, mailto | ✓ | None |
| Contact paths | Email, Book, Resume | ✓ | None |
| About mission | Goal, what we build, consulting | ✓ | None |
| Product status | In dev / live / beta | — | Missing |
| Consulting availability | Open / waitlist | — | Missing |
| Nav: Services, About, Contact, Book | ✓ | None |
| Mobile responsive | Yes | ✓ (inherited) | — |
| Accessibility | Skip link, focus | ✓ | — |

---

### Feature Gaps (Ideal vs Current)

#### Gap 1: No product launch status
**Ideal:** Visitor knows if product is live, beta, or in development.  
**Current:** Copy says "building" in services; hero doesn't.  
**Severity:** Medium — reduces clarity for product prospects.

#### Gap 2: No consulting availability signal
**Ideal:** "Consulting: open" or "Join waitlist" so enterprises know.  
**Current:** "Once the product is up" implies later.  
**Severity:** Medium — may lose consulting leads.

#### Gap 3: We/I voice inconsistency
**Ideal:** Consistent "we" (CochranBlock) or "I" (Michael) throughout.  
**Current:** Hero "we"; About "I" in Consulting; Contact "we" and "Michael's Resume."  
**Severity:** Low — polish.

#### Gap 4: No product waitlist / early access
**Ideal:** Product prospects can sign up for launch notification.  
**Current:** Only contact and book — no product-specific CTA.  
**Severity:** Medium — if product is key funnel.

#### Gap 5: No founder/team clarity for investors
**Ideal:** "Founded by Michael Cochran" or brief team line.  
**Current:** Implied in About; not explicit.  
**Severity:** Low — About has Credentials.

---

### Prioritized Recommendations

| # | Recommendation | Source | Priority |
|---|----------------|--------|----------|
| 1 | Add product status line (in dev / beta / live) | User story, Feature gap | High |
| 2 | Add consulting availability (open / waitlist) | User story, Feature gap | High |
| 3 | Unify we/I voice | User story, Feature gap | Medium |
| 4 | Add product waitlist or early-access CTA | User story, Feature gap | Medium |
| 5 | Add "Founded by Michael Cochran" for investor scan | User story | Low |

---

## Sim 3: UI/UX Analysis

**Based on:** pages.rs, main.css, cosmic theme (Orbitron, Rajdhani, accent/purple)  
**Context:** Business site; product + consulting; cosmic/cyber aesthetic  
**Date:** 2026-02-27

---

### Current Implementation

- **Layout:** Hero (center), Services (Product + Consulting cards), About (Mission + Credentials tabs), Contact (testimonial, CTAs), Book (calendar).
- **Nav:** CochranBlock logo, Home, Services, About, Contact, Book.
- **Typography:** Orbitron (headings), Rajdhani (body). Cosmic theme.
- **CTAs:** What We Build, Get in Touch, Book a Call, About CochranBlock on home.

---

### Findings

#### Strengths
- Clear hierarchy: hero → services → about → contact → book.
- Product vs Consulting separation in services.
- Tabbed About (Mission vs Credentials) keeps resume accessible.
- Booking calendar with mailto slots — low friction.
- Skip link, focus states, semantic HTML.
- Consistent cosmic/cyber aesthetic.

#### Gaps & Recommendations

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | No product status above fold | Add trust-badge style line: "Product in development · Consulting: limited capacity" |
| 2 | Hero CTAs: 4 buttons may overwhelm on mobile | Consider 2 primary (What We Build, Book a Call) + secondary row |
| 3 | Services: 6 details cards — only first open | Open first Product + first Consulting by default for scan |
| 4 | About: Mission tab has 3 subheads; dense | Add spacing or visual break between The Goal, What We Build, Consulting |
| 5 | Contact testimonial — generic | Consider rotating or more specific if available |
| 6 | No sticky CTA on scroll | Optional: "Book a Call" sticky bar on long pages |
| 7 | Footer: single CTA | Consider "Product" vs "Consulting" links if funnels differ |
| 8 | Book page: no link back to services | Ensure nav is visible; already present |

---

### Recommendations to Implement

1. **Product/consulting status badge** — One line above or below hero tagline.
2. **Services: open first Product + first Consulting** — Two `open` details for faster scan.
3. **Hero CTA hierarchy** — Primary: What We Build, Book a Call. Secondary: Get in Touch, About.
4. **Voice consistency** — Audit we/I; pick one for customer-facing copy.
5. **About spacing** — Add margin between profile-subhead blocks for readability.

---

## Sim 4: Imagery Evaluation

**Method:** Audit all imagery for coherence, brand alignment, and product differentiation.  
**Date:** 2026-02-27

---

### Imagery Inventory

| Asset | Location | Purpose | Screenshot |
|-------|----------|---------|------------|
| favicon.svg | Nav, browser tab | CochranBlock mark (diamond) | [index](../screenshots/index.png) |
| cochranblock-logo.svg | Footer | CochranBlock wordmark | [index](../screenshots/index.png) |
| rogue-repo.png | Products card | Rogue Repo product | [products](../screenshots/products.png) |
| ronin-sites.png | Products card | Ronin Sites product | [products](../screenshots/products.png) |
| kova.png | Products card | Kova product | [products](../screenshots/products.png) |

---

### Findings

| # | Criterion | Expected | Current | Gap |
|---|-----------|----------|---------|-----|
| 1 | CochranBlock vs products separation | Logo = company; product images = products | ✓ CochranBlock logo; product cards have product images | None |
| 2 | Product images match site aesthetic | Dark cosmic, accent/purple/teal | Generated cosmic-themed; verify consistency | Verify |
| 3 | Product images differentiate | Each conveys product purpose | Rogue Repo: Rust/SaaS; Ronin: shop; Kova: AI | OK |
| 4 | Alt text accuracy | Descriptive alt for each image | alt="Rogue Repo", etc. | OK |
| 5 | Services → Products link | "Rust-only SaaS" ties to Rogue Repo | Services doesn't name Rogue Repo | Add link or mention |

---

### Recommendations

1. **Services → Products tie:** Add "See our products" or link Rogue Repo by name in Services Product section.
2. **Products intro clarity:** "CochranBlock builds these products" — already clear.
3. **Imagery consistency:** Product images use cosmic palette; logo/favicon distinct (company vs product).

---

## Implementation Summary

**Executed:** 2026-02-27

| # | Item | Done |
|---|------|------|
| 1 | Product status line | ✓ hero-status: "Product in development · Consulting: limited capacity" |
| 2 | Consulting availability | ✓ Services + About + Contact: "Consulting open now — limited capacity" |
| 3 | Voice consistency (we/I) | ✓ Consulting blurb uses "we"; About Consulting unified |
| 4 | Product waitlist CTA | ✓ Contact: "Email with subject 'Product Launch' to get notified" |
| 5 | Services: open 2 details | ✓ First Product + first Consulting card open |
| 6 | Founder line for investors | ✓ About: "Founded By — Michael Cochran" |
| 7 | Hero CTA hierarchy | ✓ What We Build, Book a Call (primary); Get in Touch, About (secondary) |
| 8 | Sim 4: Imagery Evaluation | ✓ Added; Services → Products tie |
| 9 | About spacing | ✓ profile-subhead margin 1.5rem |
