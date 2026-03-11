//! Ghost billing detection (Ghost Employees, Employee Existence).
//!
//! Red flags: unexplained employee ID gaps, billed-but-not-performed.

use crate::data::Dataset;
use crate::types::{Alert, FraudType, PredicateAct, RuleId};
use chrono::Utc;
use std::collections::HashSet;

pub struct GhostDetector;

impl Default for GhostDetector {
    fn default() -> Self {
        Self
    }
}

impl GhostDetector {
    pub fn new() -> Self {
        Self
    }

    #[must_use]
    pub fn run(&self, ds: &Dataset) -> Vec<Alert> {
        let mut alerts = Vec::new();
        let employee_ids: HashSet<&str> = ds.employee_ids();

        let performed_hours: std::collections::HashMap<(String, String, String), f64> = ds
            .labor_charges
            .iter()
            .fold(
                std::collections::HashMap::new(),
                |mut acc, lc| {
                    let key = (
                        lc.contract_id.clone(),
                        lc.employee_id.clone(),
                        lc.labor_cat.clone(),
                    );
                    *acc.entry(key).or_insert(0.0) += lc.hours;
                    acc
                },
            );

        for br in &ds.billing_records {
            let contract = ds.contract_by_id(&br.contract_id);
            let (cage_code, agency) = contract
                .map(|c| (c.cage_code.as_deref(), c.agency.as_deref()))
                .unwrap_or((None, None));

            if !employee_ids.contains(br.employee_id.as_str()) {
                alerts.push(alert(
                    RuleId::GhostNoEmployee,
                    95,
                    8,
                    &format!(
                        "Billed employee '{}' not in employee roster",
                        br.employee_id
                    ),
                    Some(&br.contract_id),
                    Some(&br.employee_id),
                    cage_code,
                    agency,
                    vec![PredicateAct::FalseClaims, PredicateAct::IdentityFraud],
                ));
            }

            if let Some(emp) = ds.employee_by_id(&br.employee_id) {
                if !emp.verified {
                    alerts.push(alert(
                        RuleId::GhostNotVerified,
                        70,
                        5,
                        &format!(
                            "Billed employee '{}' has no floorcheck verification",
                            br.employee_id
                        ),
                        Some(&br.contract_id),
                        Some(&br.employee_id),
                        cage_code,
                        agency,
                        vec![PredicateAct::FalseClaims],
                    ));
                }
            }

            let key = (
                br.contract_id.clone(),
                br.employee_id.clone(),
                br.billed_cat.clone(),
            );
            let performed = performed_hours.get(&key).copied().unwrap_or(0.0);
            if performed < br.billed_hours - 0.01 {
                let (conf, sev) = if performed == 0.0 { (90, 8) } else { (80, 7) };
                alerts.push(alert(
                    RuleId::GhostBilledNotPerformed,
                    conf,
                    sev,
                    &format!(
                        "Billed {} hrs for {}/{}/{} but only {} hrs performed",
                        br.billed_hours, br.contract_id, br.employee_id, br.billed_cat, performed
                    ),
                    Some(&br.contract_id),
                    Some(&br.employee_id),
                    cage_code,
                    agency,
                    vec![PredicateAct::FalseClaims, PredicateAct::WireFraud],
                ));
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
        fraud_type: FraudType::GhostBilling,
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
