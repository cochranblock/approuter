//! Cloudflare DNS provider for ACME DNS-01 challenge.
//!
//! Minimal surface: add TXT record, remove TXT record. Everything else
//! is Cloudflare's REST API. Uses the user's existing CF_TOKEN if it has
//! Zone.DNS:Edit permission, or a dedicated token if scoped tighter.

use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct CfDns {
    token: String,
    client: reqwest::Client,
}

#[derive(Serialize)]
struct CreateRecord<'a> {
    #[serde(rename = "type")]
    record_type: &'a str,
    name: &'a str,
    content: &'a str,
    ttl: u32,
}

#[derive(Deserialize, Debug)]
struct CfResponse<T> {
    success: bool,
    #[serde(default)]
    errors: Vec<CfError>,
    result: Option<T>,
}

#[derive(Deserialize, Debug)]
struct CfError {
    code: i64,
    message: String,
}

#[derive(Deserialize, Debug)]
struct Record {
    id: String,
}

#[derive(Deserialize, Debug)]
struct Zone {
    id: String,
}

impl CfDns {
    pub fn new(token: String) -> Self {
        Self {
            token,
            client: reqwest::Client::new(),
        }
    }

    /// Resolve a zone name (e.g. "cochranblock.org") to its CF zone id.
    pub async fn zone_id(&self, zone_name: &str) -> Result<String> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones?name={}",
            zone_name
        );
        let resp: CfResponse<Vec<Zone>> = self
            .client
            .get(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        if !resp.success {
            return Err(anyhow!("cf zone lookup failed: {:?}", resp.errors));
        }
        resp.result
            .and_then(|v| v.into_iter().next())
            .map(|z| z.id)
            .ok_or_else(|| anyhow!("zone '{}' not found in CF account", zone_name))
    }

    /// Create a TXT record. Returns the record id so we can delete it later.
    pub async fn add_txt(
        &self,
        zone_id: &str,
        name: &str,
        content: &str,
    ) -> Result<String> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            zone_id
        );
        let body = CreateRecord {
            record_type: "TXT",
            name,
            content,
            ttl: 60,
        };
        let resp: CfResponse<Record> = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?
            .json()
            .await
            .context("decoding CF TXT create response")?;
        if !resp.success {
            return Err(anyhow!("cf TXT add failed: {:?}", resp.errors));
        }
        Ok(resp.result.ok_or_else(|| anyhow!("no record returned"))?.id)
    }

    /// Delete a DNS record by id. Used to clean up after DNS-01 challenge.
    pub async fn delete_record(&self, zone_id: &str, record_id: &str) -> Result<()> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            zone_id, record_id
        );
        let resp: serde_json::Value = self
            .client
            .delete(&url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        if resp.get("success").and_then(|v| v.as_bool()) != Some(true) {
            return Err(anyhow!("cf TXT delete failed: {}", resp));
        }
        Ok(())
    }

    /// Upsert an A record. Used when we want to publish the direct-ingress IP
    /// under a specific hostname (e.g. direct.cochranblock.org → 173.69.182.131).
    pub async fn upsert_a(
        &self,
        zone_id: &str,
        name: &str,
        ip: &str,
        proxied: bool,
    ) -> Result<String> {
        // Look up existing record first.
        let list_url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records?type=A&name={}",
            zone_id, name
        );
        let existing: CfResponse<Vec<serde_json::Value>> = self
            .client
            .get(&list_url)
            .bearer_auth(&self.token)
            .send()
            .await?
            .json()
            .await?;
        let body = serde_json::json!({
            "type": "A",
            "name": name,
            "content": ip,
            "ttl": 60,
            "proxied": proxied,
        });
        if let Some(records) = existing.result {
            if let Some(rec) = records.first() {
                let id = rec["id"].as_str().ok_or_else(|| anyhow!("no id"))?;
                let url = format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                    zone_id, id
                );
                let resp: serde_json::Value = self
                    .client
                    .put(&url)
                    .bearer_auth(&self.token)
                    .json(&body)
                    .send()
                    .await?
                    .json()
                    .await?;
                if resp.get("success").and_then(|v| v.as_bool()) != Some(true) {
                    return Err(anyhow!("cf A update failed: {}", resp));
                }
                return Ok(id.to_string());
            }
        }
        // Create new.
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
            zone_id
        );
        let resp: CfResponse<Record> = self
            .client
            .post(&url)
            .bearer_auth(&self.token)
            .json(&body)
            .send()
            .await?
            .json()
            .await?;
        if !resp.success {
            return Err(anyhow!("cf A create failed: {:?}", resp.errors));
        }
        Ok(resp.result.ok_or_else(|| anyhow!("no record returned"))?.id)
    }
}
