// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_imports)]

use crate::error::t18;

#[derive(Clone, Debug)]
pub struct t4 {
    pub s6: String,
    pub s7: String,
    pub s8: String,
    pub s21: String,
}

#[derive(serde::Deserialize)]
struct ZoneListResponse {
    result: Option<Vec<ZoneResult>>,
}

#[derive(serde::Deserialize)]
struct ZoneResult {
    id: String,
}

#[derive(serde::Deserialize)]
struct RecordListResponse {
    result: Option<Vec<RecordResult>>,
}

#[derive(serde::Deserialize)]
struct RecordResult {
    id: String,
    name: String,
}

#[derive(serde::Deserialize)]
struct CreateRecordResponse {
    result: Option<RecordResult>,
}

/// f47 = dns_cname_url — Cloudflare API CNAME record URL
pub fn f47(base: &str, zone: &str, record: &str) -> String {
    format!("{}/zones/{}/dns_records/{}", base.trim_end_matches('/'), zone, record)
}

/// f48 = ip_changed — compare two IP strings
pub fn f48(ip1: &str, ip2: &str) -> bool {
    ip1.trim() != ip2.trim()
}

/// f46 = validate_cf_token — check token format
pub fn f46(token: &str) -> Result<(), t18> {
    if token.trim().is_empty() {
        Err(t18::E7("API token cannot be empty".into()))
    } else {
        Ok(())
    }
}

pub async fn f11(config: &t4, ip: &str, record_name: &str) -> Result<(), t18> {
    f46(&config.s8)?;
    let url = f47(&config.s21, &config.s6, &config.s7);
    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "type": if ip.contains(':') { "AAAA" } else { "A" },
        "name": record_name,
        "content": ip,
        "ttl": 300
    });
    let res = client
        .patch(&url)
        .header("Authorization", format!("Bearer {}", config.s8))
        .json(&body)
        .send()
        .await
        .map_err(|e| t18::E3(e.to_string()))?;
    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Err(t18::E3(text));
    }
    Ok(())
}

/// Get zone ID by domain name (e.g. cochranblock.org)
pub async fn f49(base_url: &str, token: &str, domain: &str) -> Result<String, t18> {
    f46(token)?;
    let url = format!("{}/zones?name={}", base_url.trim_end_matches('/'), domain);
    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| t18::E3(e.to_string()))?;
    let body: ZoneListResponse = res.json().await.map_err(|e| t18::E3(e.to_string()))?;
    let zone = body
        .result
        .and_then(|r| r.into_iter().next())
        .ok_or_else(|| t18::E7(format!("Zone not found for {}", domain)))?;
    Ok(zone.id)
}

/// Create or get A record for zone. Returns (record_id, record_name).
pub async fn f50(base_url: &str, token: &str, zone_id: &str, domain: &str, ip: &str) -> Result<(String, String), t18> {
    f46(token)?;
    let base = base_url.trim_end_matches('/');
    let list_url = format!("{}/zones/{}/dns_records?name={}", base, zone_id, domain);
    let client = reqwest::Client::new();
    let res = client
        .get(&list_url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await
        .map_err(|e| t18::E3(e.to_string()))?;
    let body: RecordListResponse = res.json().await.map_err(|e| t18::E3(e.to_string()))?;
    if let Some(records) = body.result {
        if let Some(rec) = records.into_iter().next() {
            return Ok((rec.id, rec.name));
        }
    }
    let create_url = format!("{}/zones/{}/dns_records", base, zone_id);
    let create_body = serde_json::json!({
        "type": "A",
        "name": domain,
        "content": ip,
        "ttl": 300,
        "proxied": false
    });
    let res = client
        .post(&create_url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&create_body)
        .send()
        .await
        .map_err(|e| t18::E3(e.to_string()))?;
    if !res.status().is_success() {
        let text = res.text().await.unwrap_or_default();
        return Err(t18::E3(text));
    }
    let body: CreateRecordResponse = res.json().await.map_err(|e| t18::E3(e.to_string()))?;
    let rec = body
        .result
        .ok_or_else(|| t18::E7("Create record response missing result".into()))?;
    Ok((rec.id, rec.name))
}
