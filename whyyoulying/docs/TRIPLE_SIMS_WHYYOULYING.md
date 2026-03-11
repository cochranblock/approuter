# TRIPLE SIMS: whyyoulying

**Target:** Proactive Labor Category Fraud and Ghost Billing detection  
**Method:** Sim1→2→3→4. Implement=default. @t @b @go §1.  
**Date:** 2026-02-27

**Architecture:** [TRIPLE_SIMS_ARCH.md](TRIPLE_SIMS_ARCH.md) — domain model, pipeline, phases.

---

## Sim 1: User Story Analysis

**Done.** See [USER_STORY_ANALYSIS.md](USER_STORY_ANALYSIS.md).

| Persona | User Stories |
|---------|--------------|
| DoD IG / DCIS | D1–D6: proactive alerts, labor vs quals, ghost detection, DoD nexus, referral export |
| FBI | F1–F5: preliminary inquiry signals, predicate strength, data at scale, fraud-type routing |
| Shared | S1–S4: data ingestion, configurable thresholds, audit trail, false-positive control |

---

## Sim 2: Feature Gap Analysis

**Method:** Acceptance criteria vs current scaffold (lib.rs, config, data, detect, types)

### Acceptance Criteria vs Current State

| Criterion | Expected | Current | Gap |
|-----------|----------|---------|-----|
| Data ingestion | Contract, labor, billing feeds | JSON from data_path | ✓ |
| Labor category detection | Variance; quals vs charged | LaborDetector LABOR_VARIANCE, LABOR_QUAL_BELOW | ✓ |
| Ghost billing detection | Employee existence; billed-not-performed | GhostDetector GHOST_* | ✓ |
| Configurable thresholds | labor_variance_threshold_pct, min_confidence | Config + --threshold, --min-confidence | ✓ |
| Alert output | Alert struct | types::Alert (rule_id, confidence, predicate_acts) | ✓ |
| Fraud referral export | GAGAS / predicate docs | export-referral, export-referral --fbi | ✓ |
| Audit trail | Chain of custody | ReferralPackage.chain_of_custody, audit_entries | ✓ |
| --test flag | f49 f50 f51 same binary | ✓ | ✓ |

### Prioritized Gaps

| # | Gap | Status |
|---|-----|--------|
| 1 | Data ingestion (S1) | ✓ Done |
| 2 | Labor detector logic (D1, D2) | ✓ Done |
| 3 | Ghost detector logic (D3) | ✓ Done |
| 4 | --test binary (P14) | ✓ Done |
| 5 | Referral export (D6, F5) | ✓ Done |
| 6 | Audit trail (S3) | ✓ Done |
| 7 | Config thresholds (S2) | ✓ Done |

---

## Sim 3: CLI / API UX

**Context:** Library + CLI. No web UI. Fraud officers run locally or integrate into agency pipelines.

### Current

- Subcommands: `run`, `ingest`, `export-referral`
- CLI: `--config`, `--data-path`, `--threshold`, `--min-confidence`, `--agency`, `--cage-code`, `--output` (json/csv)
- Exit codes: 0=ok, 1=alerts, 2=error; stderr=progress, stdout=structured only

### Recommendations

| # | Item | Recommendation |
|---|------|----------------|
| 1 | CLI args | `--config`, `--data-path`, `--threshold`, `--output` (json/csv) |
| 2 | Subcommands | `run`, `ingest`, `export-referral` |
| 3 | Exit codes | 0=ok, 1=alerts found, 2=error |
| 4 | Logging | stderr for progress; stdout for structured output only |
| 5 | --test | f49 f50 f51; colored PASS/FAIL |

---

## Sim 4: Output Schema / Artifacts

**Method:** Audit output formats for DoD IG and FBI referral compatibility.

### Artifacts

| Artifact | Purpose | Format |
|----------|---------|--------|
| Alert | Single anomaly | JSON: fraud_type, severity, summary, contract_id, employee_id |
| Referral package | DoD IG fraud referral | Structured export (GAGAS) |
| Case opening docs | FBI predicate | Structured export (AG Guidelines) |
| Audit log | Chain of custody | Timestamped, immutable |

### Schema Requirements

- Alert: fraud_type, severity, summary, contract_id, employee_id, timestamp, rule_id
- Export: configurable; support JSON, CSV for integration
- Audit: every alert links to rule_id + input hash

---

## Implementation Summary

**Status:** Full implementation complete.

| # | Item | Done |
|---|------|------|
| 1 | Sim 1 User Story | ✓ USER_STORY_ANALYSIS.md |
| 2 | Sim 2 Feature Gap | ✓ This doc |
| 3 | Sim 3 CLI/API UX | ✓ run, ingest, export-referral; --config, --data-path, --threshold, --output |
| 4 | Sim 4 Output Schema | ✓ Alert (rule_id, timestamp); ReferralPackage + AuditEntry |
| 5 | Architecture (TRIPLE_SIMS_ARCH.md) | ✓ Domain model, pipeline, phases |
| 6 | Domain types | ✓ Contract, Employee, LaborCharge, BillingRecord |
| 7 | --test binary | ✓ f49 f50 f51; colored PASS/FAIL |
| 8 | Data ingestion | ✓ JSON from data_path (contracts, employees, labor_charges, billing_records) |
| 9 | Labor/Ghost detectors | ✓ LABOR_VARIANCE, LABOR_QUAL_BELOW, GHOST_* |
| 10 | Referral export | ✓ GAGAS structure with audit entries |

**Commands:** `@t` `@b` `@go` §1. **Fixtures:** `fixtures/` for sample data.

### Investigator Features (D5, F4, F5, S4)

| Feature | CLI | Description |
|---------|-----|-------------|
| DoD nexus filter | `--agency`, `--cage-code` | Filter by agency/CAGE before case opening |
| Confidence threshold | `--min-confidence` | S4 false-positive control (0-100) |
| FBI predicate routing | `predicate_acts` in Alert | False Claims, wire fraud, identity fraud |
| FBI case-opening export | `export-referral --fbi` | AG Guidelines factual basis |
| GAGAS referral | `export-referral` | Chain of custody, audit entries |
