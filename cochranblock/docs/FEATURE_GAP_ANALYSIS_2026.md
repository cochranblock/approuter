<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Feature Gap Analysis — 2026

**Date:** 2026-02-26  
**Scope:** Full portfolio (Hero, Services, About, Contact, Source, Summary, Rules)  
**Objective:** Identify and close all remaining gaps for a top-tier consultation portfolio.

---

## Current State

| Area | Status |
|------|--------|
| Hero | CTA, stats bar, tagline ✓ |
| Services | Outcome-focused cards ✓ |
| About | Tabs, profile, credentials ✓ |
| Contact | mailto, testimonial, response expectation ✓ |
| Developer pages | Source, Summary, Rules ✓ |
| Nav | 7 links, hamburger, active state ✓ |
| Accessibility | Skip link, focus states ✓ |
| Meta/OG | Description, OG tags ✓ |

---

## Gap Analysis — Legendary Panel

### Don Norman (Cognitive Design)
> "Seven nav links create decision fatigue. Group Developer (Source, Summary, Rules) under one entry. Contact has one CTA — good. Add a secondary path: 'Or add me on LinkedIn' for low-commitment visitors."

### Jakob Nielsen (Usability)
> "Per-page titles for SEO — each page should have a unique <title>. The contact section could use a micro-copy: 'No form, no friction — just email.' Font loading blocks render; add font-display: swap."

### Luke Wroblewski (Mobile)
> "Nav with 7 items on mobile — the hamburger works but the list is long. Consider a 'More' dropdown for Developer links. Contact CTA is prominent — good."

### Julie Zhuo (Product)
> "Add a trust badge line: 'Trusted by USCYBERCOM Title 10 programs' — one line, high impact. The resume has skills chips; ensure they're visible in the About tab. Consider a 'Selected Engagements' placeholder for when consulting launches."

### Jony Ive (Visual Design)
> "font-display: swap — non-negotiable for performance. The cosmic theme is cohesive. Don't add more visual noise."

---

## Recommendations (All to Implement)

| # | Recommendation | Effort |
|---|----------------|--------|
| 1 | font-display: swap on Google Fonts | Low |
| 2 | Per-page <title> for SEO | Low |
| 3 | Nav: group Developer links (dropdown or "Developer" parent) | Medium |
| 4 | Contact: add "Or connect on LinkedIn" secondary path | Low |
| 5 | Contact: micro-copy "No form, no friction — just email." | Low |
| 6 | Trust badge: "Trusted by USCYBERCOM Title 10 programs" | Low |
| 7 | og:url with canonical URL for sharing | Low |

---

## Implementation Order

1. font-display: swap — Already in Google Fonts URL
2. Per-page titles ✓
3. Trust badge ✓
4. Contact micro-copy + LinkedIn secondary ✓
5. og:url ✓
6. Nav grouping (Developer dropdown) ✓

## Implemented

| # | Recommendation | Status |
|---|----------------|--------|
| 1 | font-display: swap | Already present in @import URL |
| 2 | Per-page <title> | f62(page, title) — unique title per route |
| 3 | Nav: Developer dropdown | details/summary groups Source, Summary, Rules |
| 4 | Contact: "Or connect on LinkedIn" | contact-secondary paragraph |
| 5 | Contact: "No form, no friction — just email." | contact-micro |
| 6 | Trust badge | "Trusted by USCYBERCOM Title 10 programs" |
| 7 | og:url | Canonical URL per page in meta |
