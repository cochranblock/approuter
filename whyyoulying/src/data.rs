//! Data ingestion and normalization.

use crate::config::Config;
use crate::types::{BillingRecord, Contract, Employee, LaborCharge};
use anyhow::{Context, Result};
use std::collections::{HashMap, HashSet};
use std::path::Path;

/// Normalized dataset for detection pipeline.
#[derive(Debug, Clone, Default)]
pub struct Dataset {
    pub contracts: HashMap<String, Contract>,
    pub employees: HashMap<String, Employee>,
    pub labor_charges: Vec<LaborCharge>,
    pub billing_records: Vec<BillingRecord>,
}

impl Dataset {
    pub fn contract_by_id(&self, id: &str) -> Option<&Contract> {
        self.contracts.get(id)
    }

    pub fn employee_by_id(&self, id: &str) -> Option<&Employee> {
        self.employees.get(id)
    }

    pub fn employee_ids(&self) -> HashSet<&str> {
        self.employees.keys().map(|s| s.as_str()).collect()
    }

    /// DoD nexus filter (D5): contract IDs matching agency and/or CAGE.
    pub fn nexus_contract_ids(
        &self,
        filter_agency: Option<&str>,
        filter_cage_code: Option<&str>,
    ) -> HashSet<&str> {
        if filter_agency.is_none() && filter_cage_code.is_none() {
            return self.contracts.keys().map(|s| s.as_str()).collect();
        }
        self.contracts
            .values()
            .filter(|c| {
                let agency_ok = filter_agency
                    .map(|a| c.agency.as_deref().is_some_and(|x| x.eq_ignore_ascii_case(a)))
                    .unwrap_or(true);
                let cage_ok = filter_cage_code
                    .map(|g| c.cage_code.as_deref().is_some_and(|x| x.eq_ignore_ascii_case(g)))
                    .unwrap_or(true);
                agency_ok && cage_ok
            })
            .map(|c| c.id.as_str())
            .collect()
    }
}

pub struct Ingest;

impl Ingest {
    /// Load and normalize data from config.data_path.
    pub fn load(config: &Config) -> Result<Dataset> {
        let path = config
            .data_path
            .as_deref()
            .context("data_path required for ingest")?;
        Self::load_from_path(Path::new(path))
    }

    pub fn load_from_path(path: &Path) -> Result<Dataset> {
        let mut ds = Dataset::default();

        let contracts_path = path.join("contracts.json");
        if contracts_path.exists() {
            let s = std::fs::read_to_string(&contracts_path)
                .with_context(|| format!("read {}", contracts_path.display()))?;
            let raw: Vec<Contract> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", contracts_path.display()))?;
            ds.contracts = raw.into_iter().map(|c| (c.id.clone(), c)).collect();
        }

        let employees_path = path.join("employees.json");
        if employees_path.exists() {
            let s = std::fs::read_to_string(&employees_path)
                .with_context(|| format!("read {}", employees_path.display()))?;
            let raw: Vec<Employee> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", employees_path.display()))?;
            ds.employees = raw.into_iter().map(|e| (e.id.clone(), e)).collect();
        }

        let labor_path = path.join("labor_charges.json");
        if labor_path.exists() {
            let s = std::fs::read_to_string(&labor_path)
                .with_context(|| format!("read {}", labor_path.display()))?;
            let raw: Vec<LaborCharge> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", labor_path.display()))?;
            ds.labor_charges = raw;
        }

        let billing_path = path.join("billing_records.json");
        if billing_path.exists() {
            let s = std::fs::read_to_string(&billing_path)
                .with_context(|| format!("read {}", billing_path.display()))?;
            let raw: Vec<BillingRecord> = serde_json::from_str(&s)
                .with_context(|| format!("parse {}", billing_path.display()))?;
            ds.billing_records = raw;
        }

        Ok(ds)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Contract, Employee};
    use std::collections::HashMap;

    #[test]
    fn load_from_path_empty_dir() {
        let tmp = tempfile::TempDir::new().unwrap();
        let ds = Ingest::load_from_path(tmp.path()).unwrap();
        assert!(ds.contracts.is_empty());
        assert!(ds.employees.is_empty());
        assert!(ds.labor_charges.is_empty());
        assert!(ds.billing_records.is_empty());
    }

    #[test]
    fn load_from_path_partial() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(
            tmp.path().join("contracts.json"),
            r#"[{"id":"C1","cage_code":"1X","agency":"DoD","labor_cats":{}}]"#,
        )
        .unwrap();
        let ds = Ingest::load_from_path(tmp.path()).unwrap();
        assert_eq!(ds.contracts.len(), 1);
        assert_eq!(ds.contract_by_id("C1").unwrap().id, "C1");
        assert!(ds.employees.is_empty());
    }

    #[test]
    fn contract_by_id() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: Some("1X".into()),
                agency: Some("DoD".into()),
                labor_cats: HashMap::new(),
            },
        );
        assert!(ds.contract_by_id("C1").is_some());
        assert!(ds.contract_by_id("C2").is_none());
    }

    #[test]
    fn nexus_contract_ids_no_filter_returns_all() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: None,
                agency: None,
                labor_cats: HashMap::new(),
            },
        );
        let ids = ds.nexus_contract_ids(None, None);
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("C1"));
    }

    #[test]
    fn nexus_contract_ids_filter_agency() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: None,
                agency: Some("DoD".into()),
                labor_cats: HashMap::new(),
            },
        );
        ds.contracts.insert(
            "C2".into(),
            Contract {
                id: "C2".into(),
                cage_code: None,
                agency: Some("GSA".into()),
                labor_cats: HashMap::new(),
            },
        );
        let ids = ds.nexus_contract_ids(Some("DoD"), None);
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("C1"));
    }

    #[test]
    fn nexus_contract_ids_filter_cage() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: Some("1ABC".into()),
                agency: None,
                labor_cats: HashMap::new(),
            },
        );
        let ids = ds.nexus_contract_ids(None, Some("1ABC"));
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn nexus_contract_ids_case_insensitive() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: None,
                agency: Some("DoD".into()),
                labor_cats: HashMap::new(),
            },
        );
        let ids = ds.nexus_contract_ids(Some("dod"), None);
        assert_eq!(ids.len(), 1);
    }

    #[test]
    fn nexus_contract_ids_both_filters() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: Some("1X".into()),
                agency: Some("DoD".into()),
                labor_cats: HashMap::new(),
            },
        );
        ds.contracts.insert(
            "C2".into(),
            Contract {
                id: "C2".into(),
                cage_code: Some("2Y".into()),
                agency: Some("DoD".into()),
                labor_cats: HashMap::new(),
            },
        );
        let ids = ds.nexus_contract_ids(Some("DoD"), Some("1X"));
        assert_eq!(ids.len(), 1);
        assert!(ids.contains("C1"));
    }

    #[test]
    fn nexus_contract_ids_empty_ds() {
        let ds = Dataset::default();
        let ids = ds.nexus_contract_ids(None, None);
        assert!(ids.is_empty());
    }

    #[test]
    fn nexus_contract_ids_filter_excludes_missing_agency() {
        let mut ds = Dataset::default();
        ds.contracts.insert(
            "C1".into(),
            Contract {
                id: "C1".into(),
                cage_code: None,
                agency: None,
                labor_cats: HashMap::new(),
            },
        );
        let ids = ds.nexus_contract_ids(Some("DoD"), None);
        assert!(ids.is_empty(), "contract with no agency must not match agency filter");
    }

    #[test]
    fn employee_by_id() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                quals: vec!["BA".into()],
                ..Default::default()
            },
        );
        assert!(ds.employee_by_id("E1").is_some());
        assert!(ds.employee_by_id("E2").is_none());
    }

    #[test]
    fn employee_ids() {
        let mut ds = Dataset::default();
        ds.employees.insert(
            "E1".into(),
            Employee {
                id: "E1".into(),
                ..Default::default()
            },
        );
        ds.employees.insert(
            "E2".into(),
            Employee {
                id: "E2".into(),
                ..Default::default()
            },
        );
        let ids = ds.employee_ids();
        assert_eq!(ids.len(), 2);
        assert!(ids.contains("E1"));
        assert!(ids.contains("E2"));
    }

    #[test]
    fn load_from_path_all_files() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.json"), r#"[{"id":"C1","labor_cats":{}}]"#).unwrap();
        std::fs::write(tmp.path().join("employees.json"), r#"[{"id":"E1","quals":[],"verified":false}]"#).unwrap();
        std::fs::write(tmp.path().join("labor_charges.json"), r#"[{"contract_id":"C1","employee_id":"E1","labor_cat":"X","hours":1.0}]"#).unwrap();
        std::fs::write(tmp.path().join("billing_records.json"), r#"[{"contract_id":"C1","employee_id":"E1","billed_hours":1.0,"billed_cat":"X"}]"#).unwrap();
        let ds = Ingest::load_from_path(tmp.path()).unwrap();
        assert_eq!(ds.contracts.len(), 1);
        assert_eq!(ds.employees.len(), 1);
        assert_eq!(ds.labor_charges.len(), 1);
        assert_eq!(ds.billing_records.len(), 1);
    }

    #[test]
    fn load_from_path_invalid_json_fails() {
        let tmp = tempfile::TempDir::new().unwrap();
        std::fs::write(tmp.path().join("contracts.json"), "not json").unwrap();
        assert!(Ingest::load_from_path(tmp.path()).is_err());
    }
}
