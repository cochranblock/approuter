# TRIPLE SIMS Test Coverage Stat — whyyoulying

**Method:** Sim1→2→3→4. f49 f50 f51. Same binary.  
**Command:** `cargo run -p whyyoulying -- --test`  
**Date:** 2026-02-27

---

## Test Counts

| Phase | Count | Description |
|-------|-------|-------------|
| cargo test (unit) | 50 | config(10), types(9), data(12), detect(11), export(8) |
| cargo test (integration) | 10 | run, agency, csv, export-referral, export-fbi, empty, --test |
| --test f49–f60 | 12 | Unit, TempDir, e2e, self-integration (run, agency, csv, export, empty) |
| — | **72** | |

---

## TRIPLE SIMS Mapping

### Sim 1: User Story → Tests

| User Story | Test(s) |
|------------|---------|
| D1: Proactive labor alerts | f50 LaborQualBelow |
| D3: Ghost detection | f50 GHOST_NO_EMPLOYEE; run fixtures |
| S1: Data ingestion | f50 Ingest::load_from_path |
| S3: Audit trail | ReferralPackage.audit_entries |

### Sim 2: Feature Gap → Tests

| Criterion | Test(s) |
|-----------|---------|
| Labor detector | f49, f50 |
| Ghost detector | run fixtures |
| Config thresholds | f49 Config::default |

### Sim 3: CLI/API → Tests

| Criterion | Test(s) |
|-----------|---------|
| --test flag | f49 f50 f51 |
| Exit codes | f51 e2e |

### Sim 4: Output Schema

| Criterion | Test(s) |
|-----------|---------|
| Alert serialization | f49 |
| Export format | export-referral |

---

## Run

```bash
cargo build --release && cargo run --release -- --test
# or: @b && @t
```

Exit 0 = all pass. Same binary (P14). E2E tests require release build.
