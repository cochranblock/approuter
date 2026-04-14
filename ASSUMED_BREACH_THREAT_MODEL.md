# Assumed Breach Threat Model

> **Operating assumption: every component below is already compromised. Design for damage containment and loud detection, not for prevention.**

This document is the canonical threat model for every project in the `cochranblock/*` portfolio. Each project adapts the Threat Surface section for its own context but shares the same first principles, mitigations, and verification protocol.

---

## First Principles

1. **Every record that matters has an external witness.** Hashes published to public git (or equivalent neutral timestamp authority) so tampering requires simultaneously corrupting your system AND the public chain.
2. **No single point of compromise.** Signing keys in hardware (YubiKey / TPM / Secure Enclave). Never in software. Never in env vars. Never in config files.
3. **Default air-gap.** No network dependency for correctness. Network is for backup + publishing hashes, both signed, both verifiable post-hoc.
4. **Append-only everything.** No delete path in any storage layer. Corrections are reversing entries referencing the original. Standard accounting discipline, enforced in code.
5. **Cryptographic audit chain.** Every day's state derives from the previous day's hash. Tampering with any day invalidates every subsequent day.
6. **Disclosure of methodology is a security feature.** If an auditor can independently verify the algorithm, they can independently verify the outputs. No "trust us" layers.
7. **Separation of duties enforced in software.** Entry, approval, and audit live in different trust zones. Compromise of one does not compromise the others.
8. **Redundancy across trust zones.** Local + different-cloud + different-format + offline. Attacker must compromise all to hide damage.
9. **Test breach scenarios regularly.** Triple Sims applied to tamper detection. If the chain does not detect a simulated tamper, the chain is broken.

---

## Threat Surface (approuter)

approuter is the front-door reverse proxy + Cloudflare tunnel manager + app registry + server-side analytics for every `cochranblock/*` product. It sits on the only network path between the public internet and the product fleet, and it holds the credentials that control the public DNS zone. Its threat surface is therefore **operational-blast-radius**, not **legal-evidence-integrity**: compromise lets an attacker redirect, intercept, or impersonate the entire product portfolio, but approuter itself emits no artifacts that carry legal, financial, or audit weight on their own.

### Records this project emits

- **Analytics JSONL** (`data/analytics/YYYY-MM-DD.jsonl`) — per-request events: timestamp, host, path, method, status, duration, country/region/city from CF geo headers, hashed client IP, UA family, bot flag. Retained `ROUTER_ANALYTICS_RETENTION_DAYS` days (default 30) then pruned. Not PII in the GDPR sense (IP is one-way hashed) but re-identifiable given a known IP corpus.
- **App registry** (`data/registry/*.json`) — every registered product's public hostname, upstream origin URL, and routing rules. A map of the internal service fleet.
- **Tunnel metrics + self-check events** — latency/error rings for each ingress path; historical for WAN/CF path health.
- **Cloudflare tunnel credentials** (`~/.cloudflared/*.json`) — named-tunnel private keys written by cloudflared.
- **.env** — `CF_TOKEN`, `CF_ACCOUNT_ID`, `CF_DNS_TOKEN`, `ROUTER_API_KEY`, upstream URLs.
- **Process-level logs** — tracing output, often mirrored to journald/tmuxisfree pane scrollback; may echo headers during debug.

### Project-specific threats

| # | Assume | Blast radius |
|---|--------|-------------|
| 1 | **CF API token exfiltrated from `.env`** | Full control of the `cochranblock.org` zone: DNS rewrites, tunnel re-pointing, cache purge, WAF bypass. Attacker redirects every product to an origin they control, mid-flight, without touching the binary. Highest-value single secret in the project. |
| 2 | **Cloudflared named-tunnel credential stolen from disk** | Attacker registers the same tunnel name from their own host; CF edge may deliver traffic to them instead of the legitimate origin. Silent traffic hijack. |
| 3 | **approuter binary compromised** | Reverse proxy sees every request body/header/cookie in cleartext for every product. Full MITM of the portfolio. Modified binary can also exfiltrate secrets from `.env`. |
| 4 | **App registry (disk or REST API) poisoned** | Attacker points a registered hostname at a phishing origin. Because approuter is the canonical router, poisoning one registry entry phishes every visitor to that hostname until rollback. |
| 5 | **`ROUTER_API_KEY` leaked** | Attacker gains tunnel start/stop, registry writes, cache purge, DNS automation (if `CF_DNS_TOKEN` is loaded). Operational control of the proxy without needing the CF token. |
| 6 | **Analytics store re-identification** | Hashed IP + geo + UA + path tuple is re-identifiable against a known IP corpus (e.g., a subpoenaed ISP log). Not a legal record, but a privacy-adjacent one. |
| 7 | **Tor-blocklist inversion** | The `CF-IPCountry: T1` Tor-exit block is policy-enforcing. An attacker who flips its sense (block → admit, or admit → block) can selectively admit attack traffic or selectively deny legitimate users. Silent policy subversion. |
| 8 | **Self-check / DNS-watch loop subverted** | If the self-check reports healthy while the direct path is down, or auto-updates the A record to an attacker-controlled IP, approuter becomes the DNS hijack primitive. The loop must be observable and manually gated for DNS writes. |
| 9 | **Supply chain — `axum`, `reqwest`, `cloudflare`, `tokio`** | Any malicious dep version lands in the hot request path. `cargo audit` + pinned lockfile + reproducible builds are the only mitigations. |
| 10 | **Physical seizure of the Mac mini running the binary** | Full-disk encryption + hardware key physically separate. Without FileVault + a separate YubiKey/TPM, `.env` reads flat. |
| 11 | **Clock skew on the proxy host** | Shifts analytics timestamps and self-check rollups. Not evidence-integrity-critical here, but distorts "when did it break" forensics. Cross-check against NTP + a periodic Cloudflare-server `Date:` header. |

### Not applicable to approuter

- **Hardware-key signing of outputs.** approuter emits no artifact with legal or financial consequence — no releases, no court records, no DCAA-audited timesheet entries. Binary releases inherit signing from the workspace release pipeline, not from approuter itself.
- **Public-chain deployment.** No `cochranblock/approuter-chain` repo. Analytics JSONL and metrics are operational telemetry, not evidence of consequence; daily BLAKE3-chained commits would be theater.
- **Append-only everything / no delete path.** Analytics explicitly retains for `ROUTER_ANALYTICS_RETENTION_DAYS` then prunes — an intentional delete path, consistent with the operational (not evidentiary) nature of the data.
- **Daily hash-chained state derivation.** N/A for the same reason.
- **Triple Sims tamper-detection on the chain.** N/A — no chain. The Triple Sims gate still applies to build/test discipline; it simply has no tamper-scenario arm in this project.

If approuter ever begins emitting a record of legal consequence (e.g., an attested audit log for a regulated customer), the N/A rows promote to active and a `cochranblock/approuter-chain` repo ships alongside.

---

## Mitigations

| Assume | Mitigation | Verification |
|--------|-----------|--------------|
| Binary compromised | Hardware-key signatures for every output of consequence | Anyone can verify the public key matches expected fingerprint |
| Storage compromised | Append-only sled trees. Delete is not a function, not a policy. | Hash chain breaks on any rewrite. External witness detects. |
| Network MITM | Air-gap capable. Network used only for signed backups + hash publishing. | NTP + GitHub timestamp + hardware counter cross-checked. |
| Signing key stolen | Daily hash committed to public git. Stolen key cannot retroactively change committed days. | Any day older than the public commit is immutable in evidence. |
| Audit log tampered | Separate sled tree, write-only from main app. Auditor tool reads both + cross-checks. | Compromise of main app leaves audit log intact. |
| Backup tampered | 3 different targets with 3 different credentials (local USB + off-site cloud + paper). | Attacker needs all three to hide damage. |
| Insider / self-tampering | No admin role. No delete. Reversing entries only. | Legal record immune to author second-thoughts. |
| Clock manipulation | Multiple time sources: local clock, NTP, git commit timestamp, hardware-key counter. | Divergence flags exception requiring supervisor approval. |
| Supply chain (deps) | `cargo audit` in CI. Pinned SBOM. Reproducible builds where possible. | Anyone can reproduce the binary from source + lockfile. |
| Physical device seizure | Full-disk encryption. Hardware key physically separate from device. | Stolen laptop without key is useless for forgery. |

---

## Public-Chain Deployment

This project publishes tamper-evident hashes to a public companion repo: `cochranblock/<project>-chain` (where `<project>` is the project name).

- **Daily cycle:** at 23:59 local, compute BLAKE3 of all records-of-consequence from the day. Sign with hardware key. Commit to chain repo. Push.
- **GitHub timestamp** on the commit = neutral third-party witness. Anyone can cold-verify records were not rewritten after commit time.
- **Verification:** `<project> verify` reads the chain and re-derives hashes. Any divergence = tampering detected.

This pattern is a private Certificate Transparency log for project state. Same primitive Google uses for TLS certs, applied to whatever the project tracks.

---

## Triple Sims for Tamper Detection

Standard Triple Sims gate (run 3x identically) extended with a tamper-scenario sim:

1. Normal run → produce canonical output
2. Simulated tampering (flip one bit in storage) → `verify` must flag it
3. Simulated clock rewind → `verify` must flag it

If any sim fails to detect, the chain is broken. Fix before merge.

---

## Scope of this Document

- Covers: any artifact this project emits that has legal, financial, or audit consequence.
- Does NOT cover: source code itself (public under Unlicense, not sensitive), build outputs (reproducible), marketing content (public by design).
- If your project emits no records of consequence, the relevant sections are zero-length and the public-chain deployment is skipped. Document that explicitly.

---

## Relation to Other Docs

- **TIMELINE_OF_INVENTION.md** — establishes priority dates for contributions. Feeds into the chain's initial state.
- **PROOF_OF_ARTIFACTS.md** — cryptographic signatures on release artifacts. Adjacent pattern, same first principles.
- **DCAA_COMPLIANCE.md** (where applicable) — how this threat model satisfies FAR/DFARS audit requirements.

---

## Status

- [ ] Threat Surface section adapted for this project
- [ ] Hardware-key signing integrated or N/A documented
- [ ] Public-chain repo created and connected or N/A documented
- [ ] Triple Sims tamper-detection test present or N/A documented
- [ ] External verification procedure documented

---

*Unlicensed. Public domain. Fork, strip attribution, adapt, ship.*

*Canonical source: cochranblock.org/threat-model — last revision 2026-04-14*
