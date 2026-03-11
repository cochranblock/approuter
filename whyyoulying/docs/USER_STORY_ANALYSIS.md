# User Story Analysis: whyyoulying

**Intent (from README):** The solution to the government's ability to detect **Labor Category Fraud** and **Ghost Billing** proactively.

---

## Personas & Governing Rules

### Persona 1: DoD IG / DCIS Fraud Investigator

**Governing rules:**
- DoDI 5505.02 — Criminal Investigations of Fraud Offenses
- DoDI 5505.03 — Initiation of Investigations by DCIOs (DoD nexus required)
- DoD OIG Administrative Investigations Manual
- DoD Hotline procedures (complaint intake, triage)
- DoD IG Fraud Detection Resources (scenarios: Labor Mischarging, Ghost Employees, Time Overcharging, Labor Substitution)

**Key constraints:**
- Must identify DoD nexus before initiating criminal/civil investigation
- Jurisdiction split: DCIS vs MCIOs vs other DCIOs
- Complaint-driven intake (hotline, mail, online) — often reactive
- Labor floorchecks/interviews (DCAA 13500) are annual, not continuous
- Fraud referral required when indicators found during audit

---

### Persona 2: FBI Fraud Investigator

**Governing rules:**
- Attorney General's Guidelines (28 U.S.C. 509, 510, 533)
- FBI Domestic Investigations and Operations Guide (DIOG)
- General Crimes / Racketeering Guidelines

**Key constraints:**
- **Predicate required:** Preliminary inquiry when info doesn't warrant full investigation; full investigation when facts reasonably indicate criminal activity
- Tiered approach: Assessment → Preliminary Inquiry → Predicated Investigation → Enterprise Investigation
- Must protect individual rights; confine to legitimate law enforcement interest
- Digital financial data analysis used to identify suspicious transactions
- Often no complainant — allegations from sources of unknown reliability require preliminary inquiry first

---

## Fraud Types (whyyoulying scope)

| Fraud Type | Definition | DoD IG Scenario | Red Flags |
|------------|------------|-----------------|-----------|
| **Labor Category Fraud** | Charging employees to higher labor categories than qualifications support; inflating bill rates | Labor Mischarging, Labor Substitution | Budget vs actual variance by category; employees below min quals; missing personnel files; no segregation of duties |
| **Ghost Billing** | Billing for labor/employees that don't exist or didn't perform work | Ghost Employees, Employee Existence | Unexplained gaps in employee IDs; no floorcheck verification; small business "face" billing for prime work |

---

## User Stories (mapped to procedures)

### DoD IG / DCIS Investigator

| ID | As a... | I want to... | So that... | Procedure / Rule |
|----|---------|--------------|------------|-------------------|
| D1 | DoD IG fraud investigator | Receive proactive alerts when labor category variance exceeds threshold | I can initiate inquiry before annual floorcheck | DoDI 5505.03 (initiation); Fraud scenario: Labor Mischarging |
| D2 | DoD IG fraud investigator | See contractor labor charges vs. employee qualifications in one view | I can verify DoD nexus and assess referral merit | DoDI 5505.02 jurisdiction; Labor Mischarging red flags |
| D3 | DoD IG fraud investigator | Identify ghost employees before hotline complaint | I shift from reactive to proactive detection | Ghost Employees scenario; Hotline intake is reactive |
| D4 | DoD IG fraud investigator | Correlate billed labor categories with proposal/contract requirements | I can document factual predicate for referral | DoD Admin Investigations Manual; documentary evidence standards |
| D5 | DoD IG fraud investigator | Filter by DoD contract / CAGE / agency | I satisfy DoD nexus requirement before opening case | DoDI 5505.03 |
| D6 | DoD IG fraud investigator | Export anomaly report for fraud referral package | I meet GAGAS and referral documentation requirements | Fraud Detection Resources; audit documentation |

---

### FBI Fraud Investigator

| ID | As a... | I want to... | So that... | Procedure / Rule |
|----|---------|--------------|------------|-------------------|
| F1 | FBI fraud investigator | Receive indicators that meet preliminary inquiry threshold | I can open limited inquiry without full predicated investigation | AG Guidelines; preliminary inquiry when info doesn't warrant full investigation |
| F2 | FBI fraud investigator | See labor/billing anomalies with source reliability metadata | I can assess predicate strength for full investigation | DIOG; predicate requirements |
| F3 | FBI fraud investigator | Analyze digital financial data (labor hours, rates, categories) at scale | I can cull large datasets for relevant transactions | FBI LEB: Analysis of Digital Financial Data |
| F4 | FBI fraud investigator | Distinguish labor category fraud from ghost billing patterns | I can route to appropriate predicate (False Claims, wire fraud, etc.) | RICO predicate acts; fraud types |
| F5 | FBI fraud investigator | Get structured output for case opening documentation | I document factual basis per AG Guidelines | Case opening procedures |

---

### Shared / Cross-Agency

| ID | As a... | I want to... | So that... | Procedure / Rule |
|----|---------|--------------|------------|-------------------|
| S1 | Fraud officer (DoD or FBI) | Ingest contract, labor, and billing data from standard sources | I can run detection without manual data wrangling | Both rely on contractor/gov data systems |
| S2 | Fraud officer | Configure thresholds and red-flag rules | I align with agency-specific referral criteria | DoD vs FBI predicate standards differ |
| S3 | Fraud officer | Audit trail of how each alert was generated | I preserve chain of custody and meet evidentiary standards | Both require documentation for referrals |
| S4 | Fraud officer | Avoid false positives that waste limited investigative resources | I stay within legitimate law enforcement interest | AG Guidelines; DoD resource constraints |

---

## Procedure-Driven Requirements Summary

| Requirement | DoD IG | FBI |
|-------------|--------|-----|
| **Proactive vs reactive** | Today: hotline + annual floorchecks. Need: continuous monitoring. | Today: predicate-driven. Need: assessment-level signals to justify preliminary inquiry. |
| **Data inputs** | Contract labor categories, employee quals, timesheets, DCAA data | Financial records, labor hours, billing data |
| **Output format** | Fraud referral package; GAGAS-compliant documentation | Case opening docs; predicate documentation |
| **Thresholds** | Agency-specific; must support DoD nexus | Must support "reasonable indication" or "possible indications" |
| **Privacy / rights** | Whistleblower confidentiality; limited disclosure | Individual rights; legitimate law enforcement interest |

---

## Suggested Epic Structure

1. **Data ingestion** — Contract, labor, billing feeds; normalization
2. **Labor category detection** — Variance analysis; qualification vs. charged category
3. **Ghost billing detection** — Employee existence; billed-but-not-performed patterns
4. **Alerting & thresholds** — Configurable rules; DoD/FBI predicate alignment
5. **Referral packaging** — Export for DoD IG fraud referral; FBI case opening
6. **Audit trail** — Chain of custody; evidentiary documentation
