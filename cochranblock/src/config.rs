// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use super::error::t18;

#[derive(Clone, Debug)]
pub struct t1 {
    pub s16: u16,
    pub s17: String,
    pub s18: String,
    pub s20: String,
    pub s21: String,
    pub s22: String,
    pub s9: u64,
    pub s23: Option<String>,
    pub s24: Option<String>,
    pub s26: Option<u16>,
}

impl t1 {
    /// from_env. Why: PORTFOLIO_MASTER_KEY required for crypto; others have safe defaults.
    pub fn from_env() -> Result<Self, t18> {
        let s20 = std::env::var("PORTFOLIO_MASTER_KEY")
            .map_err(|_| t18::E7("PORTFOLIO_MASTER_KEY required".into()))?;
        if s20.len() < 16 {
            return Err(t18::E7("PORTFOLIO_MASTER_KEY must be at least 16 bytes".into()));
        }
        Ok(Self {
            s16: std::env::var("PORTFOLIO_PORT").unwrap_or_else(|_| "3000".into()).parse().unwrap_or(3000),
            s17: std::env::var("PORTFOLIO_BIND").unwrap_or_else(|_| "127.0.0.1".into()),
            s18: std::env::var("PORTFOLIO_DATA_DIR").unwrap_or_else(|_| {
                std::env::var("COCHRANBLOCK_DATA_ROOT")
                    .ok()
                    .map(|r| format!("{}/portfolio", r.trim_end_matches('/')))
                    .or_else(|| {
                        dirs::data_dir()
                            .map(|p| p.join("cochranblock").join("portfolio"))
                            .and_then(|p| p.to_str().map(String::from))
                    })
                    .unwrap_or_else(|| "data".into())
            }),
            s20,
            s21: std::env::var("PORTFOLIO_CF_BASE").unwrap_or_else(|_| "https://api.cloudflare.com/client/v4".into()),
            s22: std::env::var("PORTFOLIO_IPIFY_URL").unwrap_or_else(|_| "https://api.ipify.org".into()),
            s9: std::env::var("PORTFOLIO_DNS_INTERVAL").unwrap_or_else(|_| "300".into()).parse().unwrap_or(300),
            s23: std::env::var("PORTFOLIO_DOMAIN").ok().filter(|s| !s.trim().is_empty()),
            s24: std::env::var("PORTFOLIO_BIND_2").ok().filter(|s| !s.trim().is_empty()),
            s26: std::env::var("PORTFOLIO_PORT_2").ok().and_then(|s| s.trim().parse().ok()),
        })
    }

    pub fn db_path(&self) -> String {
        format!("{}/portfolio.db", self.s18)
    }

    /// with_values. Why: Tests need explicit config without env vars.
    pub fn with_values(
        s16: u16,
        s17: &str,
        s18: &str,
        s20: &str,
        s21: &str,
        s22: &str,
        s9: u64,
    ) -> Self {
        Self {
            s16,
            s17: s17.to_string(),
            s18: s18.to_string(),
            s20: s20.to_string(),
            s21: s21.to_string(),
            s22: s22.to_string(),
            s9,
            s23: None,
            s24: None,
            s26: None,
        }
    }
}

impl Default for t1 {
    fn default() -> Self {
        Self {
            s16: 3000,
            s17: "127.0.0.1".into(),
            s18: "data".into(),
            s20: "test-master-key-32-bytes-long!!!!".into(),
            s21: "https://api.cloudflare.com/client/v4".into(),
            s22: "https://api.ipify.org".into(),
            s9: 300,
            s23: None,
            s24: None,
            s26: None,
        }
    }
}
