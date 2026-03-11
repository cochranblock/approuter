//! Referral export (GAGAS structure) and FBI case-opening.

use crate::types::Alert;
use serde::Serialize;

/// GAGAS-compliant referral package for DoD IG fraud referral.
#[derive(Debug, Serialize)]
pub struct ReferralPackage {
    pub document_type: String,
    pub generated_at: String,
    pub chain_of_custody: ChainOfCustody,
    pub alert_count: usize,
    pub alerts: Vec<Alert>,
    pub audit_entries: Vec<AuditEntry>,
}

#[derive(Debug, Serialize)]
pub struct ChainOfCustody {
    pub tool: String,
    pub version: String,
    pub each_alert_traced_to_rule: bool,
}

#[derive(Debug, Serialize)]
pub struct AuditEntry {
    pub rule_id: String,
    pub alert_index: usize,
    pub input_hash: String,
}

/// FBI case-opening documentation per AG Guidelines (F5).
#[derive(Debug, Serialize)]
pub struct FbiCaseOpening {
    pub document_type: String,
    pub generated_at: String,
    pub factual_basis: Vec<FactualBasis>,
    pub predicate_acts_summary: std::collections::HashMap<String, usize>,
}

#[derive(Debug, Serialize)]
pub struct FactualBasis {
    pub alert_index: usize,
    pub fraud_type: String,
    pub summary: String,
    pub confidence: u8,
    pub contract_id: Option<String>,
    pub employee_id: Option<String>,
    pub predicate_acts: Vec<String>,
}

pub fn fbi_case_opening(alerts: &[Alert]) -> FbiCaseOpening {
    let mut predicate_summary: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let factual_basis: Vec<FactualBasis> = alerts
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let acts: Vec<String> = a
                .predicate_acts
                .as_ref()
                .map(|v| v.iter().map(|p| format!("{:?}", p)).collect())
                .unwrap_or_default();
            for act in &acts {
                *predicate_summary.entry(act.clone()).or_insert(0) += 1;
            }
            FactualBasis {
                alert_index: i,
                fraud_type: format!("{:?}", a.fraud_type),
                summary: a.summary.clone(),
                confidence: a.confidence,
                contract_id: a.contract_id.clone(),
                employee_id: a.employee_id.clone(),
                predicate_acts: acts,
            }
        })
        .collect();

    FbiCaseOpening {
        document_type: "FBI Case Opening - Factual Basis".to_string(),
        generated_at: chrono::Utc::now().to_rfc3339(),
        factual_basis,
        predicate_acts_summary: predicate_summary,
    }
}

pub fn referral_package(alerts: &[Alert]) -> ReferralPackage {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let generated_at = chrono::Utc::now().to_rfc3339();
    let audit_entries: Vec<AuditEntry> = alerts
        .iter()
        .enumerate()
        .map(|(i, a)| {
            let mut hasher = DefaultHasher::new();
            a.contract_id.hash(&mut hasher);
            a.employee_id.hash(&mut hasher);
            a.summary.hash(&mut hasher);
            format!("{:?}", a.rule_id).hash(&mut hasher);
            AuditEntry {
                rule_id: format!("{:?}", a.rule_id),
                alert_index: i,
                input_hash: format!("{:x}", hasher.finish()),
            }
        })
        .collect();

    ReferralPackage {
        document_type: "DoD IG Fraud Referral Package".to_string(),
        generated_at,
        chain_of_custody: ChainOfCustody {
            tool: "whyyoulying".to_string(),
            version: env!("CARGO_PKG_VERSION", "?").to_string(),
            each_alert_traced_to_rule: true,
        },
        alert_count: alerts.len(),
        alerts: alerts.to_vec(),
        audit_entries,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Alert, FraudType, PredicateAct, RuleId};

    fn sample_alert() -> Alert {
        Alert {
            fraud_type: FraudType::LaborCategory,
            rule_id: RuleId::LaborQualBelow,
            severity: 7,
            confidence: 90,
            summary: "test".into(),
            contract_id: Some("C1".into()),
            employee_id: Some("E1".into()),
            cage_code: None,
            agency: None,
            predicate_acts: Some(vec![PredicateAct::FalseClaims]),
            timestamp: None,
        }
    }

    #[test]
    fn referral_package_structure() {
        let alerts = vec![sample_alert()];
        let pkg = referral_package(&alerts);
        assert_eq!(pkg.alert_count, 1);
        assert_eq!(pkg.alerts.len(), 1);
        assert_eq!(pkg.audit_entries.len(), 1);
        assert!(pkg.document_type.contains("DoD"));
        assert!(pkg.chain_of_custody.each_alert_traced_to_rule);
        assert_eq!(pkg.chain_of_custody.tool, "whyyoulying");
    }

    #[test]
    fn referral_package_audit_entry_has_hash() {
        let alerts = vec![sample_alert()];
        let pkg = referral_package(&alerts);
        assert!(!pkg.audit_entries[0].input_hash.is_empty());
        assert!(pkg.audit_entries[0].input_hash.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn fbi_case_opening_structure() {
        let alerts = vec![sample_alert()];
        let fbi = fbi_case_opening(&alerts);
        assert!(fbi.document_type.contains("FBI"));
        assert_eq!(fbi.factual_basis.len(), 1);
        assert!(!fbi.predicate_acts_summary.is_empty());
        assert_eq!(fbi.factual_basis[0].predicate_acts.len(), 1);
    }

    #[test]
    fn fbi_case_opening_empty() {
        let fbi = fbi_case_opening(&[]);
        assert!(fbi.factual_basis.is_empty());
        assert!(fbi.predicate_acts_summary.is_empty());
    }

    #[test]
    fn referral_package_multiple_alerts() {
        let alerts = vec![sample_alert(), sample_alert()];
        let pkg = referral_package(&alerts);
        assert_eq!(pkg.alert_count, 2);
        assert_eq!(pkg.audit_entries.len(), 2);
    }

    #[test]
    fn referral_package_audit_index_matches() {
        let alerts = vec![sample_alert()];
        let pkg = referral_package(&alerts);
        assert_eq!(pkg.audit_entries[0].alert_index, 0);
        assert_eq!(pkg.audit_entries[0].rule_id, "LaborQualBelow");
    }

    #[test]
    fn referral_package_json_serializable() {
        let alerts = vec![sample_alert()];
        let pkg = referral_package(&alerts);
        let j = serde_json::to_string(&pkg).unwrap();
        assert!(j.contains("DoD"));
        assert!(j.contains("chain_of_custody"));
    }
}
