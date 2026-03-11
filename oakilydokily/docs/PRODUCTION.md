# OakilyDokily — Production Checklist

How to make the site production-ready. Follow in order.

---

## 1. Infrastructure

| Step | Action |
|------|--------|
| 1.1 | **HTTPS** — Site must be served over HTTPS. Approuter + Cloudflare Tunnel handles TLS at edge. |
| 1.2 | **Domain** — oakilydokily.com, www.oakilydokily.com point to approuter (port 8080). |
| 1.3 | **Backend** — oakilydokily runs on port 3000, approuter proxies to it. |

---

## 2. Environment Variables

Create `.env` from `.env.example`. For production:

```bash
# Required for auth
OD_BASE_URL=https://oakilydokily.com
SESSION_SECRET=<32+ random chars, e.g. openssl rand -hex 32>

# Google OAuth
GOOGLE_CLIENT_ID=<from Google Cloud Console>
GOOGLE_CLIENT_SECRET=<from Google Cloud Console>

# Apple Sign In (optional)
APPLE_CLIENT_ID=<Services ID>
APPLE_CLIENT_SECRET=<JWT, see §4>

# Optional but recommended
RESEND_API_KEY=<for waiver emails>
RESEND_FROM=OakilyDokily <noreply@oakilydokily.com>
GA4_MEASUREMENT_ID=G-XXXXXXXXXX
TURNSTILE_SITE_KEY=
TURNSTILE_SECRET_KEY=
```

---

## 3. Google OAuth (Production)

1. Go to [Google Cloud Console](https://console.cloud.google.com/) → APIs & Services → Credentials.
2. Create or edit OAuth 2.0 Client ID (Web application).
3. **Authorized redirect URIs** — add exactly:
   - `https://oakilydokily.com/auth/google/callback`
   - `https://www.oakilydokily.com/auth/google/callback`
4. Copy Client ID and Client Secret to `.env`.
5. If using restricted scopes (e.g. Gmail), complete OAuth verification. For `email` + `profile` + `openid`, verification is usually not required.

---

## 4. Apple Sign In (Production)

Apple requires HTTPS and a Services ID. No localhost.

1. **Apple Developer** → Certificates, IDs & Profiles → Identifiers.
2. Create **Services ID** (e.g. `com.oakilydokily.service`). Enable "Sign in with Apple".
3. **Return URLs** — add:
   - `https://oakilydokily.com/auth/apple/callback`
   - `https://www.oakilydokily.com/auth/apple/callback`
4. Create **Sign in with Apple Key** (Keys → Create → Sign in with Apple). Download `.p8` file once.
5. **Generate client_secret** — JWT signed with your `.p8` key. Expires in 6 months (max).

   - [Apple docs: Creating a client secret](https://developer.apple.com/documentation/accountorganizationaldatasharing/creating-a-client-secret)
   - Node: [apple-client-secret](https://www.npmjs.com/package/apple-client-secret) or manual JWT with `jsonwebtoken` + `node-forge` (ES256)
   - Python: `PyJWT` with ES256
   - Store the JWT in `APPLE_CLIENT_SECRET`. **Set a calendar reminder to rotate before 6 months.**

---

## 5. Session & Cookie Security

| Check | Status |
|-------|--------|
| SESSION_SECRET | 32+ chars: `openssl rand -hex 32` |
| Cookie HttpOnly | ✓ |
| Cookie SameSite=Lax | ✓ |
| Cookie Secure | ✓ When `OD_BASE_URL` starts with `https://` |
| Session expiry | 7 days |

**Generate SESSION_SECRET:**
```bash
openssl rand -hex 32
```

---

## 6. Deploy & Verify

```bash
# Build release binary
cd "${WORKSPACE_ROOT:-.}" && cargo build --release -p oakilydokily

# Restart via script (kills old, starts new)
cd "${WORKSPACE_ROOT:-.}" && approuter restart-oakilydokily

# Verify
curl -sI -H "Host: oakilydokily.com" https://oakilydokily.com/  # or via tunnel
```

---

## 7. Post-Launch

| Task | When |
|------|------|
| Test Sign in (Google) | After deploy |
| Test Sign in (Apple) | If configured |
| Test waiver flow | After deploy |
| Apple client_secret rotation | Every 5–6 months |
| Monitor logs | Ongoing |

---

## Quick Reference

| Item | Value |
|------|-------|
| Google redirect | `https://oakilydokily.com/auth/google/callback` |
| Apple return URL | `https://oakilydokily.com/auth/apple/callback` |
| Session cookie | `od_session` (HttpOnly, 7 days) |
