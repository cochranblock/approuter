//! Core types for fraud detection.
//!
//! Domain model per TRIPLE_SIMS_ARCH.md: Contract, Employee, LaborCharge, BillingRecord.

use std::fmt;
use serde::{Deserialize, Serialize};

/// Fraud classification per DoD IG scenarios.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FraudType {
    LaborCategory,
    GhostBilling,
}

impl fmt::Display for FraudType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            FraudType::LaborCategory => "labor_category",
            FraudType::GhostBilling => "ghost_billing",
        })
    }
}

/// Rule ID for audit trail and chain of custody (Sim 4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RuleId {
    LaborVariance,
    LaborQualBelow,
    GhostNoEmployee,
    GhostNotVerified,
    GhostBilledNotPerformed,
}

impl fmt::Display for RuleId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            RuleId::LaborVariance => "LABOR_VARIANCE",
            RuleId::LaborQualBelow => "LABOR_QUAL_BELOW",
            RuleId::GhostNoEmployee => "GHOST_NO_EMPLOYEE",
            RuleId::GhostNotVerified => "GHOST_NOT_VERIFIED",
            RuleId::GhostBilledNotPerformed => "GHOST_BILLED_NOT_PERFORMED",
        })
    }
}

/// Predicate act for FBI case routing (F4).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PredicateAct {
    FalseClaims,
    WireFraud,
    IdentityFraud,
}

/// Alert produced by a detector for fraud referral.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Alert {
    pub fraud_type: FraudType,
    pub rule_id: RuleId,
    pub severity: u8,
    /// 0-100; higher = stronger indicator (S4 false-positive control).
    pub confidence: u8,
    pub summary: String,
    pub contract_id: Option<String>,
    pub employee_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cage_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agency: Option<String>,
    /// FBI predicate routing (F4).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub predicate_acts: Option<Vec<PredicateAct>>,
    pub timestamp: Option<String>,
}

// --- Domain entities (TRIPLE_SIMS_ARCH §1) ---

/// Contract proposal/requirements: labor categories and min quals.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    pub cage_code: Option<String>,
    pub agency: Option<String>,
    /// Map labor_cat → min qualification level.
    pub labor_cats: std::collections::HashMap<String, String>,
}

/// Employee qualifications vs charged category.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Employee {
    pub id: String,
    /// Qualification levels (e.g. ["Senior", "BA"]).
    pub quals: Vec<String>,
    /// Minimum labor category this employee qualifies for.
    pub labor_cat_min: Option<String>,
    /// Floorcheck verified (DCAA 13500).
    pub verified: bool,
}

/// Actual labor charged (timesheet/DCAA).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct LaborCharge {
    pub contract_id: String,
    pub employee_id: String,
    pub labor_cat: String,
    pub hours: f64,
    pub rate: Option<f64>,
}

/// What was billed to gov.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BillingRecord {
    pub contract_id: String,
    pub employee_id: String,
    pub billed_hours: f64,
    pub billed_cat: String,
    pub period: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alert_serialize_fraud_type_snake_case() {
        let a = Alert {
            fraud_type: FraudType::LaborCategory,
            rule_id: RuleId::LaborVariance,
            severity: 5,
            confidence: 85,
            summary: "x".into(),
            contract_id: None,
            employee_id: None,
            cage_code: None,
            agency: None,
            predicate_acts: None,
            timestamp: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("labor_category"));
    }

    #[test]
    fn alert_serialize_rule_id_screaming_snake() {
        let a = Alert {
            fraud_type: FraudType::GhostBilling,
            rule_id: RuleId::GhostNoEmployee,
            severity: 8,
            confidence: 95,
            summary: "x".into(),
            contract_id: None,
            employee_id: None,
            cage_code: None,
            agency: None,
            predicate_acts: None,
            timestamp: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("GHOST_NO_EMPLOYEE"));
    }

    #[test]
    fn alert_roundtrip() {
        let a = Alert {
            fraud_type: FraudType::LaborCategory,
            rule_id: RuleId::LaborQualBelow,
            severity: 7,
            confidence: 90,
            summary: "test".into(),
            contract_id: Some("C1".into()),
            employee_id: Some("E1".into()),
            cage_code: Some("1ABC".into()),
            agency: Some("DoD".into()),
            predicate_acts: Some(vec![PredicateAct::FalseClaims]),
            timestamp: Some("2026-01-01T00:00:00Z".into()),
        };
        let j = serde_json::to_string(&a).unwrap();
        let b: Alert = serde_json::from_str(&j).unwrap();
        assert_eq!(a.fraud_type, b.fraud_type);
        assert_eq!(a.rule_id, b.rule_id);
        assert_eq!(a.contract_id, b.contract_id);
    }

    #[test]
    fn contract_default() {
        let c = Contract::default();
        assert!(c.id.is_empty());
        assert!(c.labor_cats.is_empty());
    }

    #[test]
    fn employee_default() {
        let e = Employee::default();
        assert!(e.id.is_empty());
        assert!(e.quals.is_empty());
        assert!(!e.verified);
    }

    #[test]
    fn labor_charge_default() {
        let lc = LaborCharge::default();
        assert!(lc.contract_id.is_empty());
        assert_eq!(lc.hours, 0.0);
    }

    #[test]
    fn billing_record_default() {
        let br = BillingRecord::default();
        assert!(br.employee_id.is_empty());
        assert_eq!(br.billed_hours, 0.0);
    }

    #[test]
    fn fraud_type_ghost_serialize() {
        let a = Alert {
            fraud_type: FraudType::GhostBilling,
            rule_id: RuleId::GhostNoEmployee,
            severity: 8,
            confidence: 95,
            summary: "x".into(),
            contract_id: None,
            employee_id: None,
            cage_code: None,
            agency: None,
            predicate_acts: None,
            timestamp: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("ghost_billing"));
    }

    #[test]
    fn predicate_act_serialize() {
        let a = Alert {
            fraud_type: FraudType::LaborCategory,
            rule_id: RuleId::LaborVariance,
            severity: 5,
            confidence: 85,
            summary: "x".into(),
            contract_id: None,
            employee_id: None,
            cage_code: None,
            agency: None,
            predicate_acts: Some(vec![PredicateAct::WireFraud]),
            timestamp: None,
        };
        let j = serde_json::to_string(&a).unwrap();
        assert!(j.contains("wire_fraud"));
    }
}
