<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Feature Gap Analysis — Legendary Team Simulation

**Context:** Cyber/systems consulting portfolio. Theme: Guardians of the Galaxy × Dragon × Space (cosmic, cyber-teal, starfield).

---

## Simulated Brainstorm

### Jony Ive (Design)
> "The hero has presence but lacks visual hierarchy on scroll. Add subtle scroll-triggered reveals. The services section could use outcome-focused metrics—not just what you do, but what clients gain."

### Dieter Rams (Simplicity)
> "Credentials and Research are separated now—good. But the contact page has no clear next step. One primary action. The footer is bare."

### Steve Jobs (Focus)
> "What's the ONE thing you want visitors to do? Make it unmissable. Trust signals—9+ years, 100+ enterprise-scale deployments—are buried. Pull them up."

### Julie Zhuo (Product)
> "Stats bar: 9+ years | 100+ enterprise-scale deployments | Enterprise-vetted. The testimonial has no attribution. Add it. Open one service by default so depth is visible immediately."

### Marie Kondo (UX)
> "Does the site spark joy? The skills are a comma list. Make them chips—tangible, scannable. The testimonial feels static without a name."

---

## Implemented Features

| Feature | Source | Implementation |
|--------|--------|----------------|
| Stats bar / trust signals | Jobs, Zhuo | Hero: "9+ years · 100+ enterprise-scale deployments · Enterprise-vetted" |
| Scroll reveal | Ive | CSS `@media (prefers-reduced-motion: no-preference)` + opacity/transform |
| Open first service | Zhuo | `details.service-card[open]` on Systems Engineering |
| Testimonial attribution | Zhuo, Kondo | "— Former client, federal cyber program" |
| Skills as chips | Kondo | Tag-style chips instead of comma list |
| Outcome-focused services | Ive | Each service: outcome phrase (e.g. "Ship faster, with less risk") |
| Footer enhancement | Rams | Nav links, secondary CTA |
| Truthfinder / no-bullshit | User | Profile: "I tell you what's real, what's not, and what will actually ship" |
