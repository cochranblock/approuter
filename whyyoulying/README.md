# whyyoulying

Proactive detection of **Labor Category Fraud** and **Ghost Billing** for DoD IG and FBI fraud investigators.

Per DoDI 5505.02/03, DoD OIG Fraud Scenarios, and Attorney General Guidelines.

---

## Quick Start

```bash
# Build
cargo build --release

# Run detection on sample data
cargo run --release -- --data-path fixtures run

# Run test suite (f49 f50 f51...)
cargo run --release -- --test
```

---

## Usage

| Command | Description |
|---------|-------------|
| `run` | Load data, run labor + ghost detectors, output alerts (default) |
| `ingest` | Load and validate data only |
| `export-referral` | Export GAGAS referral package or FBI case-opening docs |

### Options

| Flag | Description |
|------|-------------|
| `--data-path PATH` | Directory with contracts.json, employees.json, labor_charges.json, billing_records.json |
| `--config PATH` | Config file (labor_variance_threshold_pct, min_confidence) |
| `--threshold PCT` | Labor variance threshold (0–100) |
| `--min-confidence 0-100` | Filter alerts below confidence (S4 false-positive control) |
| `--agency AGENCY` | DoD nexus: filter by agency (e.g. DoD, Army) |
| `--cage-code CODE` | DoD nexus: filter by CAGE code |
| `--output json\|csv` | Output format |
| `--test` | Run f49–f60 test suite |

### Exit Codes

- `0` — No alerts
- `1` — Alerts found
- `2` — Error

---

## Data Format

Place JSON files in `--data-path`:

- `contracts.json` — id, cage_code, agency, labor_cats
- `employees.json` — id, quals, labor_cat_min, verified
- `labor_charges.json` — contract_id, employee_id, labor_cat, hours, rate
- `billing_records.json` — contract_id, employee_id, billed_hours, billed_cat, period

See `fixtures/` for examples.

---

## Detection Rules

| Rule | Type | Description |
|------|------|-------------|
| LABOR_VARIANCE | Labor | Labor category not in contract |
| LABOR_QUAL_BELOW | Labor | Employee charged above qualification |
| GHOST_NO_EMPLOYEE | Ghost | Billed employee not in roster |
| GHOST_NOT_VERIFIED | Ghost | No floorcheck verification |
| GHOST_BILLED_NOT_PERFORMED | Ghost | Billed hours exceed performed |

---

## Docs

- [USER_STORY_ANALYSIS](docs/USER_STORY_ANALYSIS.md) — DoD IG / FBI personas
- [TRIPLE_SIMS_WHYYOULYING](docs/TRIPLE_SIMS_WHYYOULYING.md) — Sim 1–4
- [TRIPLE_SIMS_ARCH](docs/TRIPLE_SIMS_ARCH.md) — Domain model, pipeline 
