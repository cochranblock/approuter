<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# User Story Analysis — Booking Calendar (Braintrust Context)

## Context

**Use case:** Calendar link shared on Braintrust talent profile. When hiring managers/recruiters view Michael's profile on Braintrust, they click through to schedule a discovery call directly.

**Audience:** Companies and hiring managers browsing Braintrust for cyber/systems engineering talent. They're evaluating fit and want a low-friction way to connect.

**Braintrust expectations:** Professional presentation, clear value, quick booking. Talent who make it easy to schedule get more calls.

---

## User Stories

| ID | As a... | I want to... | So that... |
|----|---------|---------------|-------------|
| US1 | Hiring manager on Braintrust | Click a calendar link and see available slots | I can book a call without leaving the flow |
| US2 | Hiring manager | Know this is a 30-min discovery call | I can block my calendar correctly |
| US3 | Recruiter | See availability for the next 1–2 weeks | I can propose times that work |
| US4 | Hiring manager | Book with one action (click slot → email opens) | I minimize friction |
| US5 | Braintrust visitor | Immediately understand this is Michael's personal booking page | I'm not confused by a generic calendar |
| US6 | Mobile user | Use the calendar on my phone | I can book from anywhere |

## Acceptance Criteria

- Page title/heading: "Schedule a Discovery Call" — clear purpose
- Subheading: "30 minutes · Discuss your mission and how I can help"
- Braintrust context: Optional line "Shared via Braintrust" or similar for credibility
- Calendar shows 2 weeks, typical availability (e.g., Tue/Thu 2–4pm ET)
- Slots open mailto with subject: "Discovery Call Request — [date] [time]"
- Matches site aesthetic (cosmic/cyber)
- Mobile-responsive, accessible
- Nav includes Book link for consistency

## Implementation Approach

- Route: `/book`
- No backend: mailto for slot selection
- Client-side calendar (vanilla JS or minimal)
- Add "Book" to nav (alongside Contact)
