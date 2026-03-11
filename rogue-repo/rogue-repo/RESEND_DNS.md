# Resend DNS Records

Add MX, TXT (SPF), and DKIM for sending verification emails.

## Automated

```bash
cd cochranblock
# CF_TOKEN, RESEND_API_KEY (from rogue-repo/.env)
cargo run -p runner -- setup-resend-dns                    # roguerepo.io
cargo run -p runner -- setup-resend-dns --domain oakilydokily.com
```

Adds MX and TXT (SPF). If API key is full (not send-only), also creates domain in Resend and adds DKIM. Otherwise: add domain at [resend.com/domains](https://resend.com/domains), copy DKIM TXT value, add manually in Cloudflare.

---

## Manual setup (roguerepo.io / oakilydokily.com)

**First:** Add the domain at [resend.com/domains](https://resend.com/domains) and copy the DKIM value.

---

## Records to Add (Cloudflare)

| Field | Value |
|-------|-------|
| Type | MX |
| Name | `send` |
| Mail Server | `feedback-smtp.us-east-1.amazonses.com` |
| Priority | 10 |
| Proxy | DNS only (grey cloud) |

### 2. TXT — send (SPF)

| Field | Value |
|-------|-------|
| Type | TXT |
| Name | `send` |
| Content | `v=spf1 include:amazonses.com ~all` |
| Proxy | DNS only |

### 3. TXT — resend._domainkey (DKIM)

| Field | Value |
|-------|-------|
| Type | TXT |
| Name | `resend._domainkey` |
| Content | *(copy from Resend dashboard when you add the domain)* |
| Proxy | DNS only |

The DKIM value looks like `p=MIGfMA0GCS...` — get it from Resend → Domains → Add Domain → Copy TXT value.

---

## After Adding Records

1. In Resend, click **Verify DNS Records** on your domain.
2. Update `.env`: `RESEND_FROM=Rogue Repo <noreply@roguerepo.io>`

---

## Alternative: Use onboarding@resend.dev

If you skip domain verification, you can send from `onboarding@resend.dev` (Resend's default). No DNS records needed. Set `RESEND_FROM=Rogue Repo <onboarding@resend.dev>` in `.env`.
