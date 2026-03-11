<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# UI/UX Audit — Michael Cochran Consultation Business

**Audit date:** 2026-02-25  
**Scope:** Public portfolio site (Hero, Services, About Me, Contact)  
**Objective:** Ensure the site positions you as a top-tier consultation business for federal cyber, systems engineering, and AI-assisted development.

---

## Executive Summary

**Strengths:** Strong visual identity (cosmic/cyber aesthetic), clear technical credibility, good information architecture.  
**Gaps:** Contact has no conversion path, hero lacks a clear CTA, services need outcome-focused framing, mobile/accessibility not verified.

---

## 1. Hero Section

### Current
- Name + tagline "Senior Systems Engineer & Offensive Cyber Operator"
- Links: Services · About Me · Contact

### Recommendations
| Issue | Fix |
|-------|-----|
| **No primary CTA** | Add a prominent "Schedule Consultation" or "Get in Touch" button below the tagline. Consultation businesses need one clear next step. |
| **Tagline is role, not value** | Reframe: "Mission-critical cyber systems. Federal-grade expertise. AI-accelerated delivery." — speaks to outcomes, not just title. |
| **"MC 🐉" nav brand** | Consider full name or "Cochran Consulting" for nav — "MC" is ambiguous for new visitors. |
| **Hero links look like nav** | Differentiate: keep nav in header; hero should have one primary CTA + optional secondary. |

---

## 2. Services Section

### Current
- Single paragraph + bullet list (Systems Engineering, Offensive Cyber, API & Integration, CI/CD & Automation)

### Recommendations
| Issue | Fix |
|-------|-----|
| **Features, not outcomes** | Each service should answer "What do I get?" e.g. "Systems Engineering — Design, build, and harden mission-critical infrastructure for Title 10 and IC environments." |
| **No differentiation** | Add 1–2 sentence differentiator: "USCYBERCOM-vetted. 9+ years offensive and defensive. Delivered 100+ missions." |
| **List feels flat** | Use cards or expandable sections with short descriptions. Consultation buyers want to skim and drill down. |
| **Missing engagement CTA** | End with "Discuss your mission" or "Book a discovery call" linking to Contact. |

---

## 3. About Me (Tabbed)

### Current
- Profile tab: 2 sentences (USCYBERCOM, skills)
- Legacy tab: Resume, whitepaper, innovation blurb

### Recommendations
| Issue | Fix |
|-------|-----|
| **Profile too brief** | Expand to 3–4 short paragraphs: who you are, what you've done, why you consult, what makes you different. |
| **"Legacy System" label** | Rename to "Credentials" or "Work & Research" — "Legacy" sounds deprecated. |
| **Innovation blurb in wrong place** | Move "3.4 hours, AI, single binary" to a subtle footer or About sidebar — it's impressive but not primary for consultation buyers. |
| **No social proof** | Add "Selected clients" or "Past engagements" (anonymized if needed) — trust signals matter. |
| **Tab UX** | Add `aria-selected` and `role="tab"` for accessibility. Ensure tab focus order is logical. |

---

## 4. Contact Section

### Current
- "Reach out for opportunities in systems engineering, offensive cyber, or AI-assisted development."
- No form, no email, no calendar link.

### Recommendations
| Issue | Fix |
|-------|-----|
| **No conversion mechanism** | Add at least one of: (a) mailto link with clear subject, (b) Calendly/Cal.com embed, (c) simple contact form (name, email, message). |
| **Vague CTA** | Replace with: "Ready to discuss your mission? Book a 30-minute discovery call or email [address]." |
| **No response expectation** | Set expectations: "I typically respond within 24–48 hours." |
| **Form styling** | `.form-container` exists in CSS but isn't used — wire it up for a contact form. |

---

## 5. Navigation & Layout

### Current
- Nav: MC 🐉 | Hero, Services, About Me, Contact
- Fixed structure, no mobile menu

### Recommendations
| Issue | Fix |
|-------|-----|
| **"Hero" in nav** | Use "Home" — "Hero" is internal jargon. |
| **Mobile nav** | Add hamburger + collapsible menu for small screens. Test at 375px, 768px. |
| **Active state** | Ensure current page nav link has `.active` class for clarity. |
| **Sticky nav** | Consider `position: sticky` so CTA stays visible on scroll. |

---

## 6. Trust & Credibility

### Recommendations
| Addition | Purpose |
|----------|---------|
| **Clearance / certs** | If applicable: "TS/SCI" or relevant certs (OSCP, etc.) — even as text, not badges. |
| **Testimonial or quote** | One short quote from a past engagement (anonymized) builds trust. |
| **LinkedIn link** | Professional profiles reinforce legitimacy. |
| **Consistent tone** | Use "you" and "your mission" — consultation is client-focused. |

---

## 7. Accessibility

### Recommendations
| Issue | Fix |
|-------|-----|
| **Focus states** | Ensure all interactive elements have visible `:focus` outline (keyboard users). |
| **Color contrast** | Verify `--muted` (#94a3b8) on `--bg` (#050508) meets WCAG AA (4.5:1). |
| **Skip link** | Add "Skip to main content" for screen readers. |
| **Heading hierarchy** | Ensure single `<h1>` per page; logical h2 → h3 structure. |
| **Alt text** | No images currently; if added, use descriptive alt. |

---

## 8. Performance & Polish

### Recommendations
| Issue | Fix |
|-------|-----|
| **Font loading** | Google Fonts can block render — consider `font-display: swap` or self-host. |
| **Meta description** | Add `<meta name="description" content="...">` for SEO and link previews. |
| **OG tags** | Add Open Graph tags for LinkedIn/Twitter sharing. |
| **Favicon** | Add favicon for browser tab identity. |

---

## 9. Priority Implementation Order

1. **Contact conversion** — Add mailto or Calendly link (highest impact).
2. **Hero CTA** — One clear "Get in Touch" or "Schedule Call" button.
3. **Services outcome framing** — Rewrite bullets as outcomes, add CTA.
4. **Nav fixes** — "Home" not "Hero", active state.
5. **About Me expansion** — Richer profile, rename "Legacy" tab.
6. **Mobile nav** — Hamburger for small screens.
7. **Accessibility** — Focus states, contrast check.
8. **Meta/OG** — SEO and sharing.

---

## 10. Quick Wins (Low Effort)

- Change nav "Hero" → "Home"
- Add `mailto:michael@example.com?subject=Consultation%20Inquiry` to Contact
- Add `<meta name="description" content="Michael Cochran — Senior Systems Engineer & Offensive Cyber Operator. Federal cyber, systems engineering, AI-assisted development.">`
- Rename "Legacy System" tab → "Credentials" or "Work & Research"
