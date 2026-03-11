<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Triple Sims: AI Orchestration Page

**Target:** `/ai-orchestration` — Developer-facing page explaining AI-assisted development approach  
**Method:** Sequential analysis (User Story → Feature Gap → UI/UX), then implementation  
**Date:** 2026-02-27

---

## Sim 1: User Story Analysis

**Personas:** Hiring Manager (evaluating AI maturity), CTO/Engineering Lead (assessing methodology), Developer (learning the approach)

---

### Simulation 1: Hiring Manager (Evaluating AI Maturity)

**Scenario:** Hiring manager is evaluating Michael for a role requiring AI-assisted development. They want to understand his methodology and differentiate it from typical usage.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Land on /ai-orchestration | Clear page purpose | ✓ "AI Orchestration" heading; page-intro explains orchestratable implementer |
| 2 | Understand the big idea | Delegation vs helper model | ✓ "delegated implementer" vs "one-off helper" stated |
| 3 | See concrete examples | 12 ways this differs | ✓ 12 collapsible details; first open by default |
| 4 | Understand orchestration hub | cochranblock + cochranblock | ✓ "Orchestration hub" section; binary, rules, prompt reference |
| 5 | Find related content | Links to Cursor Rules, Summary | ✓ CTA buttons at bottom |
| 6 | Navigate from Developer menu | AI Orchestration in nav | ✓ Under Developer dropdown with Source, Summary, Rules |

**Pain points:** All 12 details collapsed except #1—user may not realize there are 11 more. No "TL;DR" or executive one-liner for quick scan. No link to prompt reference doc (CURSOR_PROMPTS_FROM_OTHER_WINDOWS.md) for those who want raw data.

---

### Simulation 2: CTO / Engineering Lead (Assessing Methodology)

**Scenario:** CTO wants to assess whether this approach is reproducible, rule-driven, and production-grade.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Verify rule-driven | Codified orchestration | ✓ "Rules as orchestration" in How It Works |
| 2 | See deploy workflow | Concrete commands | ✓ cargo run --test, go-live-wsl.sh, cochranblock --go-live |
| 3 | Understand test rigor | No mocks | ✓ "No Mocks, Real Systems" in 12 ways; anti-pattern rule referenced |
| 4 | Check cross-project | Reusable infrastructure | ✓ Prompt reference, rules, binaries span projects |
| 5 | Assess executive translation | Non-engineer outputs | ✓ "Executive Translation" in 12 ways |

**Pain points:** "How It Works in Practice" is brief—CTO may want more depth (e.g. what rules exist, where prompt reference lives). No link to actual .cursor/rules or docs. Code blocks in list items may wrap poorly on narrow viewports.

---

### Simulation 3: Developer (Learning the Approach)

**Scenario:** Developer wants to adopt this methodology. They need actionable patterns and entry points.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Scan 12 ways | Expand/collapse each | ✓ details/summary; only #1 open |
| 2 | Get concrete prompts | Example phrases | ✓ "do it for me," "assume I want more work," "WHAT WOULD THE COMPANY DO?" |
| 3 | Find cochranblock specifics | Where rules live | ✓ .cursor/rules/ mentioned; no direct link |
| 4 | Understand prompt reference | Cross-project prompts | ✓ CURSOR_PROMPTS_FROM_OTHER_WINDOWS.md mentioned in markdown source; not linked on page |
| 5 | Mobile/tablet read | Responsive | ✓ developer-page, summary-card; assumed responsive from site-wide CSS |

**Pain points:** No deep link to prompt reference. No "Try it" or "Start here" CTA for developers. 12 details require 12 clicks to read all—no "expand all" option. `.ai-detail` may have no distinct styling vs generic details.

---

## User Story Coverage Summary

| US | Story | Status |
|----|-------|--------|
| US1 | Hiring manager understands AI orchestration vs typical usage | ✓ Content present; TL;DR could help |
| US2 | CTO sees rule-driven, production-grade methodology | ✓ How It Works; could link to rules |
| US3 | Developer learns patterns and entry points | ⚠ No link to prompt reference; no expand-all |
| US4 | User finds related content (Rules, Summary) | ✓ CTA buttons |
| US5 | User navigates from Developer menu | ✓ Nav includes AI Orchestration |
| US6 | Mobile/accessibility | ✓ Semantic HTML; touch targets TBD |

---

## Recommendations from User Story Simulation

1. **TL;DR / executive one-liner** — Add a single sentence above "The Big Idea" for quick scan (e.g. "I orchestrate; the AI implements. Rules, not prompts.").
2. **Link to prompt reference** — Add link to `docs/archive/CURSOR_PROMPTS_FROM_OTHER_WINDOWS.md` (or rendered equivalent) for those who want raw prompts.
3. **Link to Cursor rules** — Add "View rules" link that goes to /cursor-rules or opens .cursor/rules context.
4. **Expand-all control** — Optional button to expand/collapse all 12 details for power users.
5. **Improve discoverability of 12 ways** — Consider "12 ways" as visible count or badge so users know to expand.

---

## Sim 2: Feature Gap Analysis

**Method:** Current implementation vs acceptance criteria + ideal UX  
**Reference:** ai_orchestration.html, pages.rs, main.css, AI_ORCHESTRATION_APPROACH.md

---

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Page title | "AI Orchestration" | ✓ | None |
| Clear value prop | Orchestration vs helper model | ✓ Big Idea section | None |
| 12 ways explained | Expandable details | ✓ 12 details; #1 open | Only 1 open; no expand-all |
| How It Works | Concrete workflow | ✓ 3 bullets | Code blocks may wrap on mobile |
| Links to related | Rules, Summary | ✓ 2 CTA buttons | No link to prompt reference |
| Nav includes page | Developer dropdown | ✓ | None |
| Matches site aesthetic | Cosmic/cyber | ✓ developer-page, summary-card | — |
| Mobile-responsive | Yes | ✓ (inherited) | Not verified |
| Accessibility | Semantic, focus states | ✓ details/summary | No aria-expanded on details? |

---

### Feature Gaps (Ideal vs Current)

#### Gap 1: No TL;DR / executive one-liner
**Ideal:** Hiring manager sees one sentence that captures the approach in 5 seconds.  
**Current:** Must read "The Big Idea" paragraph.  
**Severity:** Medium — slows evaluators.

#### Gap 2: No link to prompt reference
**Ideal:** Developer or CTO can click to see raw prompts (CURSOR_PROMPTS_FROM_OTHER_WINDOWS.md).  
**Current:** Doc mentioned in markdown source; not linked on page.  
**Severity:** High — key artifact is unreachable from the page.

#### Gap 3: Only first detail open
**Ideal:** User can expand all 12 at once, or see a count ("12 ways — click to expand").  
**Current:** Only #1 open; others require 11 clicks.  
**Severity:** Medium — reduces scan-ability.

#### Gap 4: No link to Cursor rules from content
**Ideal:** "Rules as orchestration" links to /cursor-rules.  
**Current:** CTA at bottom links to Rules; in-content mention does not.  
**Severity:** Low — CTA exists; in-content link would reinforce.

#### Gap 5: Code blocks in list may wrap poorly
**Ideal:** `cargo run --test`, `go-live-wsl.sh` etc. wrap or scroll on narrow viewports.  
**Current:** `<code>` in `<li>`; no overflow handling.  
**Severity:** Low — polish.

#### Gap 6: .ai-detail styling
**Ideal:** details/summary for 12 ways have distinct styling (accent border, spacing) for visual hierarchy.  
**Current:** class="ai-detail" exists; may inherit generic details styling only.  
**Severity:** Low — consistency with site.

#### Gap 7: No "Start here" for developers
**Ideal:** Developer sees "Start here: read Cursor Rules, then try a prompt from the reference."  
**Current:** No onboarding CTA.  
**Severity:** Medium — reduces adoption.

---

### Prioritized Recommendations

| # | Recommendation | Source | Priority |
|---|----------------|--------|----------|
| 1 | Add link to prompt reference | User story, Feature gap | High |
| 2 | Add TL;DR / executive one-liner | User story, Feature gap | High |
| 3 | Add expand-all / collapse-all for 12 details | User story, Feature gap | Medium |
| 4 | Add "Start here" CTA for developers | User story, Feature gap | Medium |
| 5 | Add in-content link to /cursor-rules at "Rules as orchestration" | Feature gap | Low |
| 6 | Ensure code blocks wrap or scroll on mobile | Feature gap | Low |
| 7 | Add .ai-detail styling if missing | UI/UX | Low |

---

## Sim 3: UI/UX Analysis

**Based on:** ai_orchestration.html, developer-page styles, summary-card, cosmic theme  
**Context:** Developer-facing page; hiring managers and CTOs also visit  
**Date:** 2026-02-27

---

### Current Implementation

- **Layout:** Single section, developer-page class. summary-card divs for Big Idea, 12 Ways, How It Works.
- **12 Ways:** details/summary with class="ai-detail". First open; rest collapsed.
- **CTA:** Two buttons—View Cursor Rules (primary), Summary (secondary).
- **Nav:** AI Orchestration under Developer dropdown with Source, Summary, Rules.
- **Typography:** Orbitron (headings), Rajdhani (body). Cosmic theme (accent, purple, glow).

---

### Findings

#### Strengths
- Semantic structure: section, h1, h2, details/summary, ul/li.
- Consistent with developer-page pattern (executive-summary, cursor-rules).
- summary-card provides visual grouping.
- CTA buttons link to related content.
- Skip link, focus states inherited from site.

#### Gaps & Recommendations

| # | Issue | Recommendation |
|---|-------|----------------|
| 1 | No TL;DR above fold | Add one-line value prop: "I orchestrate; the AI implements. Rules, not prompts." |
| 2 | 12 details—only #1 open; no expand-all | Add "Expand all" / "Collapse all" button; or open first 3 by default |
| 3 | No link to prompt reference | Add "View prompt reference" link (to /source with anchor, or new route for docs) |
| 4 | Code in list items may overflow on mobile | Add `code { word-break: break-all; }` or `overflow-x: auto` for .developer-page code |
| 5 | .ai-detail may lack distinct styling | Add margin/padding, accent border-left, or summary::before for visual hierarchy |
| 6 | CTA row has two buttons; no "Start here" | Add tertiary link: "Start here: Rules → Prompt reference" |
| 7 | No visual cue for "12 ways" count | Add badge or subheading: "12 ways this differs (expand to read)" |
| 8 | Selected/expanded detail not emphasized | Add .ai-detail[open] summary styling (accent, border) |

---

### Recommendations to Implement

1. **TL;DR one-liner** — Add above "The Big Idea" for 5-second scan.
2. **Prompt reference link** — Add link (e.g. to /source#prompts or docs). If prompt reference is HTML, add route.
3. **Expand-all control** — Optional JS or pure HTML (details with multiple open—HTML5 allows multiple open).
4. **Code overflow** — Ensure code in list items wraps or scrolls on narrow viewports.
5. **.ai-detail[open] styling** — Accent border or background when expanded.
6. **"Start here" CTA** — Tertiary link for developers.

---

## Implementation Summary

**Executed:** 2026-02-27 (after all 3 analyses)

| # | Item | Done |
|---|------|------|
| 1 | TL;DR one-liner | ✓ Added above "The Big Idea" |
| 2 | Prompt reference link | ✓ New /prompts route + content; CTA on AI Orchestration page |
| 3 | Expand-all | ✓ First 3 details open by default; hint "Expand each to read" |
| 4 | Code overflow | ✓ .developer-page code { word-break: break-word } |
| 5 | .ai-detail[open] styling | ✓ Accent border, box-shadow when expanded |
| 6 | "Start here" CTA | ✓ outcome-note with links to Rules + Prompt Reference |
| 7 | Nav/footer | ✓ Prompts added to Developer dropdown |
| 8 | HTTP test | ✓ prompts_200 |
