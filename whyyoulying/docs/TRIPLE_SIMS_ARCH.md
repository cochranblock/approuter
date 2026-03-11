# TRIPLE SIMS Architecture — whyyoulying

**Method:** Sim1→2→3→4. Implement=default.  
**Date:** 2026-02-27

---

## 1. Domain Model

### 1.1 Core Entities

```
┌─────────────────┐     ┌─────────────────┐     ┌─────────────────┐
│    Contract     │     │    Employee     │     │   LaborCharge    │
├─────────────────┤     ├─────────────────┤     ├─────────────────┤
│ id              │     │ id              │     │ contract_id     │
│ cage_code       │     │ quals[]         │     │ employee_id     │
│ agency          │     │ labor_cat_min   │     │ labor_cat       │
│ labor_cats[]    │     │ hire_date       │     │ hours           │
│ (proposal/req)  │     │ verified        │     │ rate            │
└────────┬────────┘     └────────┬────────┘     └────────┬────────┘
         │                       │                       │
         └───────────────────────┴───────────────────────┘
                                 │
                    ┌────────────▼────────────┐
                    │      BillingRecord      │
                    ├────────────────────────┤
                    │ contract_id            │
                    │ employee_id             │
                    │ billed_hours            │
                    │ billed_cat              │
                    │ period                  │
                    └────────────────────────┘
```

### 1.2 Entity Definitions

| Entity | Purpose | Key Fields |
|--------|---------|------------|
| **Contract** | Proposal/contract labor categories and requirements | id, cage_code, agency, labor_cats (map cat→min_qual) |
| **Employee** | Employee qualifications vs charged category | id, quals[], labor_cat_min, verified (floorcheck) |
| **LaborCharge** | Actual labor charged (timesheet/DCAA) | contract_id, employee_id, labor_cat, hours, rate |
| **BillingRecord** | What was billed to gov | contract_id, employee_id, billed_hours, billed_cat, period |

### 1.3 Detection Inputs

| Detector | Primary Inputs | Secondary |
|----------|----------------|-----------|
| **LaborDetector** | Contract.labor_cats, Employee.quals, LaborCharge.labor_cat | Config.labor_variance_threshold_pct |
| **GhostDetector** | Employee (existence), BillingRecord (billed vs performed) | Employee.verified |

---

## 2. Pipeline Flow

```
  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
  │   Ingest     │────▶│  Normalize   │────▶│   Detect     │
  │ (raw feeds)  │     │ (entities)   │     │ (labor+ghost)│
  └──────────────┘     └──────────────┘     └──────┬───────┘
         │                      │                   │
         ▼                      ▼                   ▼
  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
  │ Contract     │     │ Contract     │     │ Alert[]      │
  │ Labor        │     │ Employee     │     │ (rule_id,    │
  │ Billing      │     │ LaborCharge  │     │  timestamp)  │
  │ (file/API)   │     │ BillingRecord│     └──────┬───────┘
  └──────────────┘     └──────────────┘            │
                                                   ▼
  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐
  │ Export       │◀───│ AuditLog     │◀───│ Output       │
  │ (referral,   │     │ (chain of    │     │ (stdout/     │
  │  case docs)  │     │  custody)    │     │  file)       │
  └──────────────┘     └──────────────┘     └──────────────┘
```

### 2.1 Stages

| Stage | Module | Output |
|-------|--------|--------|
| Ingest | `data::Ingest` | Raw records (contract, labor, billing) |
| Normalize | `data` (or `data::normalize`) | Contract, Employee, LaborCharge, BillingRecord |
| Detect | `detect::labor`, `detect::ghost` | Vec&lt;Alert&gt; |
| Output | `main` / CLI | stdout (JSON/CSV) or file |
| Export | `export::referral_package`, `export::fbi_case_opening` | Referral package, FBI case docs |
| Audit | `ReferralPackage.audit_entries`, `chain_of_custody` | rule_id + input hash |

---

## 3. Rule IDs (for Alert.rule_id)

| rule_id | Detector | Description |
|---------|----------|-------------|
| `LABOR_VARIANCE` | LaborDetector | Budget vs actual variance exceeds threshold |
| `LABOR_QUAL_BELOW` | LaborDetector | Employee quals below charged category min |
| `GHOST_NO_EMPLOYEE` | GhostDetector | Billed employee_id not in Employee set |
| `GHOST_NOT_VERIFIED` | GhostDetector | Billed but no floorcheck verification |
| `GHOST_BILLED_NOT_PERFORMED` | GhostDetector | Billing record without matching LaborCharge |

---

## 4. Implementation Phases

### Phase A: Foundation (blocks all)

| # | Item | Module | Deps |
|---|------|--------|------|
| A1 | Domain types (Contract, Employee, LaborCharge, BillingRecord) | `types.rs` | — |
| A2 | Alert + timestamp, rule_id | `types.rs` | — |
| A3 | Config from file + --config, --data-path, --threshold | `config.rs`, `main.rs` | — |
| A4 | --test binary (f49 f50 f51 scaffold) | `main.rs` | P14 |

### Phase B: Data

| # | Item | Module | Deps |
|---|------|--------|------|
| B1 | Ingest from data_path (JSON) | `data.rs` | A3 |
| B2 | Normalize → entities | `data.rs` | A1, B1 |

### Phase C: Detection

| # | Item | Module | Deps |
|---|------|--------|------|
| C1 | LaborDetector: variance logic | `detect/labor.rs` | A1, A2, B2 |
| C2 | LaborDetector: qual vs charged | `detect/labor.rs` | A1, A2, B2 |
| C3 | GhostDetector: employee existence | `detect/ghost.rs` | A1, A2, B2 |
| C4 | GhostDetector: billed-not-performed | `detect/ghost.rs` | A1, A2, B2 |

### Phase D: CLI & Output

| # | Item | Module | Deps |
|---|------|--------|------|
| D1 | CLI: --config, --data-path, --threshold, --output | `main.rs` | A3 |
| D2 | Subcommands: run, ingest, export-referral | `main.rs` | — |
| D3 | Exit codes: 0=ok, 1=alerts, 2=error | `main.rs` | — |
| D4 | stdout=structured only; stderr=progress | `main.rs` | — |

### Phase E: Referral & Audit

| # | Item | Module | Deps |
|---|------|--------|------|
| E1 | Referral export (GAGAS structure) | `export::referral_package` | C1–C4 |
| E2 | Audit log (rule_id + input hash) | `ReferralPackage.audit_entries` | A2 |

---

## 5. File Structure (Proposed)

```
src/
├── main.rs          # CLI, --test, subcommands
├── lib.rs
├── config.rs
├── types.rs         # Alert, FraudType, Contract, Employee, LaborCharge, BillingRecord
├── data.rs          # Ingest, normalize
├── detect/
│   ├── mod.rs
│   ├── labor.rs
│   └── ghost.rs
└── export/
    └── mod.rs       # referral_package, fbi_case_opening
```

---

## 6. Test Strategy (f49 f50 f51)

| Tier | Scope | I/O | Example |
|------|-------|-----|---------|
| f49 | Unit | None | Config::load, Alert::serialize, LaborDetector::run with mock data |
| f50 | Integration | TempDir | Ingest from temp JSON files; detect pipeline |
| f51 | E2E | Real | Optional: real data path; colored PASS/FAIL |

---

## 7. Next Steps

1. Run `@t` `@b` `@go` after changes.
2. Update `TRIPLE_SIMS_WHYYOULYING.md` Implementation Summary when adding features.
