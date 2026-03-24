<!-- Unlicense — cochranblock.org -->

# Timeline of Invention

*Dated, commit-level record of what was built, when, and why.*

> Every entry maps to real commits. Run `git log --oneline` to verify.

---

## Entries

### 2026-03-22 — CODEOWNERS + Governance

**What:** Added CODEOWNERS and OWNERS.yaml for repository governance.
**Commit:** `ca99daf`
**AI Role:** AI generated governance files. Human decided ownership structure.

### 2026-03-20 — Tunnel Startup Sync

**What:** Sync tunnel ingress on startup to prevent stale port (55842) from previous runs.
**Why:** Production reliability — stale tunnel configs caused routing failures after restarts.
**Commit:** `6b07717`
**AI Role:** AI fixed the race condition. Human identified the production failure pattern.

### 2026-03-18 — Foundational Founders v0.2.0

**What:** Version bump, contributor attribution locked, Unlicense headers across all files.
**Commit:** `6366db2`
**AI Role:** AI applied headers systematically. Human decided licensing and attribution model.

### 2026-03-17 — GitHub Actions CI

**What:** CI workflow running exopack test binary (approuter-test) with xvfb + chromium.
**Commit:** `31f607e`
**AI Role:** AI wrote CI config. Human specified test requirements and environment.

### 2026-03-16 — Railway Removal + Cloudflare-Only

**What:** Removed all Railway deployment artifacts. approuter is Cloudflare tunnel + VPS only.
**Why:** Railway added unnecessary complexity. Single tunnel + single VPS is the architecture.
**Commit:** `3723a11`
**AI Role:** AI cleaned up artifacts. Human made the architectural decision to simplify.

### 2026-03-14 — Initial Architecture

**What:** Full reverse proxy with dynamic registration, Cloudflare tunnel management, DNS automation, OpenAPI spec, client library.
**Why:** Multi-product routing without operational overhead. One command, all services.
**AI Role:** AI generated initial implementation. Human designed routing strategy, credential model, and tunnel automation.

---

*Part of the [CochranBlock](https://cochranblock.org) zero-cloud architecture. All source under the Unlicense.*
