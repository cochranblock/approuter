<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# UI/UX Analysis — Developer Pages (Source, Executive Summary, Cursor Rules)

**Audit date:** 2026-02-26  
**Scope:** /source, /executive-summary, /cursor-rules  
**Panel:** Don Norman, Jakob Nielsen, Luke Wroblewski

---

## Executive Summary

**Strengths:** Clear structure, collapsible sections, cosmic theme consistency.  
**Gaps:** Nav crowding, code readability, mobile layout, hierarchy, copy-to-clipboard.

---

## 1. Source Code Page

| Issue | Recommendation |
|-------|----------------|
| **Nav has 7 items** | Group developer links: "Source", "Summary", "Rules" under a "Developer" dropdown or secondary nav row |
| **Code blocks too tall** | Max-height 24rem is good; add "Expand" control for full view |
| **No syntax highlighting** | Monospace + color is sufficient; consider line numbers for long files |
| **Copy button missing** | Add "Copy" button per file for developer convenience |
| **Page intro could be shorter** | One line: "Key files from the portfolio binary." |

---

## 2. Executive Summary Page

| Issue | Recommendation |
|-------|----------------|
| **Dense text blocks** | Add spacing between sections; use cards for Key Themes and Decisions |
| **No visual hierarchy** | H2s need more prominence; consider timeline or flow for themes |
| **Outcome link** | cochranblock.org link should be styled as CTA (btn or prominent) |
| **Readability** | Line-height 1.6; max-width for body text |

---

## 3. Cursor Rules Page

| Issue | Recommendation |
|-------|----------------|
| **Raw .mdc frontmatter** | Strip YAML frontmatter (---) for cleaner display, or style it muted |
| **Monospace for long rules** | Rule content is readable; ensure sufficient contrast |
| **Collapsible default** | First rule open; others closed — reduces scroll |
| **Rule descriptions** | Show description from frontmatter as subtitle under each rule name |

---

## 4. Cross-Cutting

| Issue | Recommendation |
|-------|----------------|
| **Nav overflow on mobile** | Hamburger already exists; ensure 7 links wrap or collapse gracefully |
| **Breadcrumb** | Add "Developer" or "Build" as context for these pages |
| **Consistent page structure** | All three: h1, intro, content. Apply same spacing and typography |
| **Focus order** | Ensure tab order: nav → main → details/summary |

---

## 5. Priority Implementation

1. **Nav grouping** — Developer dropdown or condensed labels (Source | Summary | Rules)
2. **Executive Summary** — Card layout for Themes and Decisions; CTA for outcome link
3. **Rules frontmatter** — Strip or style YAML block
4. **Copy button** — Add to source code blocks
5. **Mobile nav** — Verify 7 links in hamburger menu
