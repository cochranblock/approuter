<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# UI/UX Simulation — Legendary Team Analysis

**Date:** 2026-02-25  
**Site:** Michael Cochran | Cyber & Systems Consulting  
**Theme:** Cosmic/cyber (Orbitron, Rajdhani, starfield, teal/purple accents)

---

## Simulated Panel

### Don Norman (Cognitive Design)
> "The hero has two CTAs of equal weight — Schedule Consultation and View Services. The brain doesn't know where to go first. Make one primary (filled), one secondary (outline). The contact page repeats the same CTA twice (Email Me + footer Get in Touch). Consolidate. One clear path."

### Jakob Nielsen (Usability)
> "Tab panels need aria-hidden when inactive — screen readers announce hidden content. The research details are long; first-time visitors may not expand. Add a one-line teaser visible before expand. Service cards: the Outcome label is good but could be visually separated (e.g. subtle divider) so it scans faster."

### Luke Wroblewski (Mobile)
> "Hero padding 5rem — on 375px that's a lot of vertical space before content. Reduce on small screens. Button tap targets: .btn padding 0.6rem 1.5rem — that's ~24px height. WCAG recommends 44px minimum. Increase padding on mobile. Footer nav wraps awkwardly — consider horizontal scroll or stacked on very narrow."

### Julie Zhuo (Product)
> "The testimonial is strong but isolated. Add a second micro-trust signal somewhere — e.g. 'Trusted by federal programs' or a stat. The Profile tab paragraphs are dense; add a subheading or two to break up the wall of text. 'Credentials & Research' — good. The tab label length is uneven (Profile/Background vs Credentials & Research); consider shortening for balance."

### Jony Ive (Visual Design)
> "The footer has a dragon emoji before the copyright. It's playful but may clash with the federal/cyber seriousness. Consider removing or replacing with a subtle icon. Section transitions: the scroll reveal runs on load only — consider reducing animation delay so content appears faster. The starfield twinkle is subtle; good. Don't add more motion."

---

## Recommendations (Prioritized)

| # | Recommendation | Source | Effort |
|---|----------------|--------|--------|
| 1 | Primary vs secondary CTA — hero: Schedule Consultation primary (filled), View Services secondary | Norman | Low |
| 2 | Tab panels: add aria-hidden when inactive | Nielsen | Low |
| 3 | Button tap targets: min 44px height on mobile | Wroblewski | Low |
| 4 | Hero padding: reduce on small screens (e.g. 3rem) | Wroblewski | Low |
| 5 | Profile tab: add subheadings to break up text | Zhuo | Low |
| 6 | Remove or replace footer dragon emoji | Ive | Low |
| 7 | Font loading: add font-display: swap to Google Fonts URL | Performance | Low |
| 8 | Service Outcome: subtle visual separator (border-top or label) | Nielsen | Low |
| 9 | Research details: ensure first line is compelling before fold | Nielsen | Low |
