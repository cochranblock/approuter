//! Labor category fraud detection (Labor Mischarging, Labor Substitution).
//!
//! Red flags: budget vs actual variance, employees below min quals.

use crate::data::Dataset;
use crate::types::{Alert, FraudType, PredicateAct, RuleId};
use chrono::Utc;

/// Category ordering for qual-below check (higher index = higher category).
const CAT_ORDER: &[&str] = &["Junior", "Mid", "Senior", "Lead", "Principal"];

fn category_level(cat: &str) -> usize {
    CAT_ORDER
        .iter()
        .position(|&c| c.eq_ignore_ascii_case(cat))
        .unwrap_or(0)
}

pub struct LaborDetector {
    pub threshold_pct: f64,
}

impl LaborDetector {
    pub fn new(threshold_pct: f64) -> Self {
        Self { threshold_pct }
    }

    #[must_use]
    pub fn run(&self, ds: &Dataset) -> Vec<Alert> {
        let mut alerts = Vec::new();

        for lc in &ds.labor_charges {
            let contract = ds.contract_by_id(&lc.contract_id);
            if let Some(c) = contract {
                if !c.labor_cats.contains_key(&lc.labor_cat) {
                    alerts.push(alert(
                        RuleId::LaborVariance,
                        85,
                        6,
                        &format!(
                            "Labor category '{}' not in contract {}",
                            lc.labor_cat, lc.contract_id
                        ),
                        Some(&lc.contract_id),
                        Some(&lc.employee_id),
                        c.cage_code.as_deref(),
                        c.agency.as_deref(),
                        vec![PredicateAct::FalseClaims],
                    ));
                }
            }

            if let Some(emp) = ds.employee_by_id(&lc.employee_id) {
                if let Some(ref min_cat) = emp.labor_cat_min {
                    if category_level(&lc.labor_cat) > category_level(min_cat) {
                        let c = contract;
                        alerts.push(alert(
                            RuleId::LaborQualBelow,
                            90,
                            7,
                            &format!(
                                "Employee {} charged as '{}' but qualifies only for '{}'",
                                lc.employee_id, lc.labor_cat, min_cat
                            ),
                            Some(&lc.contract_id),
                            Some(&lc.employee_id),
                            c.and_then(|x| x.cage_code.as_ref()).map(|s| s.as_str()),
                            c.and_then(|x| x.agency.as_ref()).map(|s| s.as_str()),
                            vec![PredicateAct::FalseClaims, PredicateAct::WireFraud],
                        ));
                    }
                }
            }
        }

        alerts
    }
}

#[allow(clippy::too_many_arguments)]
fn alert(
    rule_id: RuleId,
    confidence: u8,
    severity: u8,
    summary: &str,
    contract_id: Option<&str>,
    employee_id: Option<&str>,
    cage_code: Option<&str>,
    agency: Option<&str>,
    predicate_acts: Vec<PredicateAct>,
) -> Alert {
    Alert {
        fraud_type: FraudType::LaborCategory,
        rule_id,
        severity,
        confidence,
        summary: summary.to_string(),
        contract_id: contract_id.map(String::from),
        employee_id: employee_id.map(String::from),
        cage_code: cage_code.map(String::from),
        agency: agency.map(String::from),
        predicate_acts: Some(predicate_acts),
        timestamp: Some(Utc::now().to_rfc3339()),
    }
}
