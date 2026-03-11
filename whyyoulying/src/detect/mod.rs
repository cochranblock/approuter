//! Fraud detection modules.

pub mod ghost;
pub mod labor;

#[cfg(test)]
mod tests {
    use super::labor::LaborDetector;
    use super::ghost::GhostDetector;
    use crate::data::Dataset;
    use crate::types::{Contract, Employee, LaborCharge, BillingRecord};

    fn contract(id: &str, agency: Option<&str>, cage: Option<&str>) -> Contract {
        Contract {
            id: id.into(),
            agency: agency.map(String::from),
            cage_code: cage.map(String::from),
            ..Default::default()
        }
    }

    #[test]
    fn labor_detector_empty_ds_no_alerts() {
        let ds = Dataset::default();
        let det = LaborDetector::new(15.0);
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn labor_detector_qual_below() {
        let mut ds = Dataset::default();
        let c = contract("C1", Some("DoD"), None);
        ds.contracts.insert(c.id.clone(), c);
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                quals: vec!["BA".into()],
                labor_cat_min: Some("Junior".into()),
                verified: false,
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Principal".into(),
            hours: 40.0,
            rate: Some(150.0),
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborQualBelow")));
    }

    #[test]
    fn labor_detector_variance_unapproved_cat() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Senior".to_string(), "BA".to_string())].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "UnapprovedCat".into(),
            hours: 10.0,
            rate: None,
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborVariance")));
    }

    #[test]
    fn ghost_detector_empty_ds_no_alerts() {
        let ds = Dataset::default();
        let det = GhostDetector::new();
        assert!(det.run(&ds).is_empty());
    }

    #[test]
    fn ghost_detector_no_employee() {
        let mut ds = Dataset::default();
        let c = contract("C1", None, None);
        ds.contracts.insert(c.id.clone(), c);
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E99".into(),
            billed_hours: 10.0,
            billed_cat: "Junior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostNoEmployee")));
    }

    #[test]
    fn ghost_detector_billed_not_performed() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: true,
                ..Default::default()
            },
        );
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(!alerts.is_empty());
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostBilledNotPerformed")));
    }

    #[test]
    fn labor_detector_qual_ok_no_alert() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("Junior".to_string(), "Assoc".to_string())].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                labor_cat_min: Some("Senior".into()),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Junior".into(),
            hours: 40.0,
            rate: None,
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(alerts.is_empty());
    }

    #[test]
    fn labor_detector_both_unknown_category_no_qual_alert() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                labor_cats: [("CustomCat".to_string(), "X".to_string())].into_iter().collect(),
                ..Default::default()
            },
        );
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                labor_cat_min: Some("OtherCustom".into()),
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "CustomCat".into(),
            hours: 40.0,
            rate: None,
        });
        let det = LaborDetector::new(15.0);
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("LaborQualBelow")));
    }

    #[test]
    fn ghost_detector_not_verified() {
        let mut ds = Dataset::default();
        let c = contract("C1", None, None);
        ds.contracts.insert(c.id.clone(), c);
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: false,
                ..Default::default()
            },
        );
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 10.0,
            billed_cat: "Junior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostNotVerified")));
    }

    #[test]
    fn ghost_detector_partial_performed() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: true,
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 20.0,
            rate: None,
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostBilledNotPerformed")));
    }

    #[test]
    fn ghost_detector_verified_no_alert() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                verified: true,
                ..Default::default()
            },
        );
        ds.labor_charges.push(LaborCharge {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            labor_cat: "Senior".into(),
            hours: 40.0,
            rate: None,
        });
        ds.billing_records.push(BillingRecord {
            contract_id: "C1".into(),
            employee_id: "E1".into(),
            billed_hours: 40.0,
            billed_cat: "Senior".into(),
            period: None,
        });
        let det = GhostDetector::new();
        let alerts = det.run(&ds);
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostNotVerified")));
        assert!(!alerts.iter().any(|a| format!("{:?}", a.rule_id).contains("GhostBilledNotPerformed")));
    }
}
