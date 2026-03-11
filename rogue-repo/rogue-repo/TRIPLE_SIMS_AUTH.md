<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# TRIPLE SIMS: Rogue Repo — Auth (d7)

**Target:** roguerepo.io — f97 (register), f98 (login), f100 (verify), f101 (logout)  
**Method:** d1 → d2 → d3  
**Scope:** f102 (/login), f103 (/register), f97–f101  
**Date:** 2026-02-28

---

## d1: User Story Analysis

**Personas:** New user (register), Returning user (login), Unverified user, Logged-in user (logout)

---

### Simulation 1: New User (Register)

**Scenario:** User wants to create an account to buy Rogue Bucks and provision games.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Find f103 | Nav or CTA | ✓ Nav Login → f102; tabs to f103 |
| 2 | Submit email + password | Account created | ✓ POST f97; redirect /login?registered=1 |
| 3 | See verification message | "Check your email" | ✓ Message on f102 |
| 4 | Receive verification link | Email or dev log | ⚠ Dev: link logged; no SMTP |
| 5 | Click verification link | Email verified | ✓ GET f100?token=... → redirect f102?verified=1 |

**Pain points:** No email sent in prod; verification link only in logs.

---

### Simulation 2: Returning User (Login)

**Scenario:** User has verified email; wants to log in.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Go to f102 | Login form | ✓ Email, password, submit |
| 2 | Submit valid credentials | Session set, redirect | ✓ POST f98 → redirect / with rr_session cookie |
| 3 | Invalid credentials | Error message | ✓ Redirect f102 with error |
| 4 | Unverified email | Blocked with message | ✓ "Please verify your email first" |
| 5 | Session persistence | Stay logged in | ✓ 7-day cookie; httpOnly, SameSite=Lax |

**Pain points:** Login returns redirect; API clients may expect JSON.

---

### Simulation 3: Unverified User

**Scenario:** User registered but hasn't clicked verification link.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Try to login | Blocked | ✓ 403 "Please verify your email first" |
| 2 | Resend verification | New link | ⚠ Not implemented |
| 3 | Token expired | Clear message | ✓ f100?token=invalid → f102?error=invalid_token |

**Pain points:** No resend-verification. Token expires in 24h.

---

### Simulation 4: Logged-in User (Logout)

**Scenario:** User wants to sign out.

| Step | Action | Expected | Observed |
|------|--------|----------|----------|
| 1 | Find f101 | Nav link | ✓ Nav Logout → GET/POST f101 |
| 2 | Click logout | Cookie cleared | ✓ rr_session cleared; redirect / |

**Pain points:** None.

---

## User Story Coverage Summary

| US | Story | Status |
|----|-------|--------|
| US1 | New user registers (f97) | ✓ |
| US2 | User verifies email (f100) | ✓ |
| US3 | Returning user logs in (f98) | ✓ |
| US4 | Unverified user blocked | ✓ |
| US5 | User logs out (f101) | ✓ |
| US6 | Resend verification | ⚠ Not implemented |
| US7 | Forgot password | ⚠ Not implemented |

---

## d2: Feature Gap Analysis

**Method:** Current auth vs acceptance criteria  
**Reference:** auth.rs, login.html, register.html, migrations

---

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| f97 register | Email, password, submit | ✓ | None |
| Password hash | Argon2 | ✓ | None |
| Verification token | 24h expiry | ✓ | None |
| f100 verify | Clickable link | ✓ | None |
| Email delivery | SMTP or provider | Log only | **High** |
| f98 login | Email, password | ✓ | None |
| Session | Signed cookie | ✓ HMAC-SHA256 | None |
| f101 logout | Clear session | ✓ | None |
| Resend verification | POST /resend-verify | None | Medium |
| Rate limiting | Brute-force protection | None | Medium |

---

### Prioritized Recommendations

| # | Recommendation | Priority |
|---|----------------|----------|
| 1 | Add email sender (lettre or Resend) | High |
| 2 | Add POST /resend-verification | Medium |
| 3 | Add rate limiting | Medium |

---

## d3: UI/UX Analysis

**Reference:** login.html, register.html, pwa.css

---

### Auth Portal UI Patterns

| Pattern | Auth0 | Firebase | Rogue Repo |
|---------|-------|----------|------------|
| Login/Register tabs | ✓ | ✓ | ✓ f102, f103 |
| Error messages | Inline | Inline | Query param → JS |
| Success messages | Toast/inline | Inline | Query param → JS |
| Password requirements | Shown | Shown | Hint "8+ chars" |
| Back to app | ✓ | ✓ | ✓ "Back to home" |
| Theme consistency | ✓ | ✓ | ✓ Dark, cyan accent |

---

## d4: Implementation Summary

**Executed:** 2026-02-28

| # | Item | Done |
|---|------|------|
| 1 | Migration (password_hash, verification_token) | ✓ |
| 2 | f97 POST /register | ✓ |
| 3 | f98 POST /login (session cookie) | ✓ |
| 4 | f100 GET /verify-email | ✓ |
| 5 | f101 POST/GET /logout | ✓ |
| 6 | f102 GET /login, f103 GET /register | ✓ |
| 7 | Nav Login, Logout | ✓ |
| 8 | Argon2 password hashing | ✓ |
| 9 | HMAC-signed session | ✓ |
| 10 | Email delivery (prod) | ⚠ Stub; log only |
| 11 | Resend verification | ❌ |
| 12 | Rate limiting | ❌ |
