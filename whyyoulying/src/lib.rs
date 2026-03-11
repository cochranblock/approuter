//! Proactive detection of Labor Category Fraud and Ghost Billing.
//!
//! Supports DoD IG and FBI fraud investigator workflows per DoDI 5505.02/03
//! and Attorney General Guidelines.

#[cfg(feature = "tests")]
pub mod tests;
pub mod config;
pub mod data;
pub mod detect;
pub mod export;
pub mod types;

pub use config::Config;
pub use data::{Dataset, Ingest};
pub use detect::{labor::LaborDetector, ghost::GhostDetector};
pub use types::{
    Alert, BillingRecord, Contract, Employee, FraudType, LaborCharge, PredicateAct, RuleId,
};
