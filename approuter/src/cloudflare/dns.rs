#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use super::{cf_api_base, f94, C1, CF_CLIENT};
use serde_json::json;
use std::error::Error;

/// DNS write token. Prefers `CF_DNS_TOKEN` (Zone.DNS:Edit scope). Falls back to
/// the general `CF_TOKEN` / `CLOUDFLARE_API_TOKEN` if no dedicated DNS token is
/// configured. Returning a dedicated error message helps the operator fix the
/// env rather than hunt through the CF audit log.
fn dns_write_token() -> Result<String, Box<dyn Error + Send + Sync>> {
    std::env::var("CF_DNS_TOKEN")
        .or_else(|_| std::env::var("CF_TOKEN"))
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| {
            "CF_DNS_TOKEN (Zone.DNS:Edit) or CF_TOKEN required for DNS writes".into()
        })
}

/// Resolve a zone name (e.g. `cochranblock.org`) to its Cloudflare zone id.
pub async fn lookup_zone_id(zone_name: &str) -> Result<String, Box<dyn Error + Send + Sync>> {
    let token = dns_write_token()?;
    let url = format!("{}/client/v4/zones?name={}", cf_api_base(), zone_name);
    let res = CF_CLIENT
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let text = res.text().await?;
    let j: serde_json::Value = serde_json::from_str(&text)?;
    if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("Zones lookup failed: {}", text).into());
    }
    j.get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|z| z.get("id"))
        .and_then(|i| i.as_str())
        .map(String::from)
        .ok_or_else(|| format!("Zone {} not found", zone_name).into())
}

/// Update (or create) an A record. Resolves the existing record id via the
/// CF API; if absent, creates a new proxied A record. Returns the new/existing
/// record id on success.
///
/// `record_name` is fully-qualified (e.g. `gd.cochranblock.org`). The zone is
/// derived automatically via `f94` (last two dotted parts).
pub async fn update_dns_a_record(
    zone_id: &str,
    record_name: &str,
    new_ip: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let token = dns_write_token()?;
    let auth = format!("Bearer {}", token);

    // Find the existing A record, if any.
    let list_url = format!(
        "{}/client/v4/zones/{}/dns_records?type=A&name={}",
        cf_api_base(),
        zone_id,
        record_name
    );
    let res = CF_CLIENT
        .get(&list_url)
        .header("Authorization", &auth)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let text = res.text().await?;
    let j: serde_json::Value = serde_json::from_str(&text)?;
    if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("DNS list for {} failed: {}", record_name, text).into());
    }
    let existing = j
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .cloned();

    let body = json!({
        "type": "A",
        "name": record_name,
        "content": new_ip,
        "ttl": 300,
        "proxied": false
    });

    if let Some(rec) = existing {
        let rec_id = rec
            .get("id")
            .and_then(|i| i.as_str())
            .ok_or("existing record has no id")?
            .to_string();
        let put_url = format!(
            "{}/client/v4/zones/{}/dns_records/{}",
            cf_api_base(),
            zone_id,
            rec_id
        );
        let res = CF_CLIENT
            .put(&put_url)
            .header("Authorization", &auth)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let txt = res.text().await?;
        let j: serde_json::Value = serde_json::from_str(&txt)?;
        if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
            return Err(format!("DNS A update {} -> {} failed: {}", record_name, new_ip, txt).into());
        }
        tracing::info!("[dns] A {} updated -> {}", record_name, new_ip);
        Ok(rec_id)
    } else {
        let post_url = format!(
            "{}/client/v4/zones/{}/dns_records",
            cf_api_base(),
            zone_id
        );
        let res = CF_CLIENT
            .post(&post_url)
            .header("Authorization", &auth)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let txt = res.text().await?;
        let j: serde_json::Value = serde_json::from_str(&txt)?;
        if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
            return Err(format!("DNS A create {} -> {} failed: {}", record_name, new_ip, txt).into());
        }
        let new_id = j
            .get("result")
            .and_then(|r| r.get("id"))
            .and_then(|i| i.as_str())
            .unwrap_or("")
            .to_string();
        tracing::info!("[dns] A {} created -> {}", record_name, new_ip);
        Ok(new_id)
    }
}

/// Fetch the proxy's external IPv4 via the configured lookup URL
/// (`SELFCHECK_EXT_IP_URL`, default `https://api.ipify.org`).
pub async fn fetch_external_ip() -> Result<String, Box<dyn Error + Send + Sync>> {
    let url = std::env::var("SELFCHECK_EXT_IP_URL")
        .unwrap_or_else(|_| "https://api.ipify.org".into());
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(5))
        .build()?;
    let res = client.get(&url).send().await?;
    let ip = res.text().await?.trim().to_string();
    if ip.is_empty() {
        return Err("external IP lookup returned empty body".into());
    }
    Ok(ip)
}

/// Convenience: resolve the zone from a fully-qualified record name, then
/// update the A record. One call, no explicit zone wrangling at the call site.
pub async fn set_a_record_for(
    record_name: &str,
    new_ip: &str,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let zone_name = f94(record_name);
    let zone_id = lookup_zone_id(&zone_name).await?;
    update_dns_a_record(&zone_id, record_name, new_ip).await
}

/// Long-running loop: every `interval_secs`, fetch the external IP, and if it
/// differs from the last observed value, update the A record for `record_name`.
/// Runs until the process exits or an unrecoverable CF error occurs. Only
/// invoked via `approuter dns watch-ip` — never from `start-all`.
pub async fn watch_ip_and_update(
    record_name: &str,
    interval_secs: u64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let interval = std::time::Duration::from_secs(interval_secs.max(30));
    let mut last_ip: Option<String> = None;
    tracing::info!(
        "[dns-watch] watching external IP, updating A {} every {}s",
        record_name,
        interval.as_secs()
    );
    loop {
        match fetch_external_ip().await {
            Ok(ip) => {
                if last_ip.as_deref() != Some(&ip) {
                    match set_a_record_for(record_name, &ip).await {
                        Ok(id) => {
                            tracing::info!(
                                "[dns-watch] {} -> {} (record id: {})",
                                record_name,
                                ip,
                                id
                            );
                            last_ip = Some(ip);
                        }
                        Err(e) => tracing::warn!("[dns-watch] CF update failed: {}", e),
                    }
                } else {
                    tracing::debug!("[dns-watch] {} -> {} unchanged", record_name, ip);
                }
            }
            Err(e) => tracing::warn!("[dns-watch] external IP fetch failed: {}", e),
        }
        tokio::time::sleep(interval).await;
    }
}

/// f95 = ensure_cname. hostname->tunnel. Zone must exist in CF.
pub async fn f95(
    p0: &str,
    p1: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;
    let client = &*CF_CLIENT;
    let auth = format!("Bearer {}", s8);

    let zone_name = f94(p0);
    let url = format!(
        "{}/client/v4/zones?name={}",
        cf_api_base(),
        zone_name
    );
    let res = client
        .get(&url)
        .header("Authorization", &auth)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let text = res.text().await?;
    let j: serde_json::Value = serde_json::from_str(&text)?;
    if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("Zones API failed: {}", text).into());
    }
    let zone_id = j
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|z| z.get("id"))
        .and_then(|id| id.as_str())
        .ok_or_else(|| format!("Zone {} not found in Cloudflare", zone_name))?;

    let record_name = if p0 == zone_name || p0 == format!("{}.", zone_name) {
        "@"
    } else {
        p0.strip_suffix(&format!(".{}", zone_name)).unwrap_or("@")
    };

    let list_url = format!(
        "{}/client/v4/zones/{}/dns_records?name={}",
        cf_api_base(),
        zone_id, p0
    );
    let r = client
        .get(&list_url)
        .header("Authorization", &auth)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let j: serde_json::Value = serde_json::from_str(&r.text().await?)?;
    let rec = j.get("result").and_then(|r| r.as_array()).and_then(|a| a.first());

    let body = json!({
        "type": "CNAME",
        "name": record_name,
        "content": p1,
        "ttl": 1,
        "proxied": true
    });

    if let Some(r) = rec {
        let rec_id = r.get("id").and_then(|i| i.as_str()).ok_or("No record id")?;
        let put_url = format!(
            "{}/client/v4/zones/{}/dns_records/{}",
            cf_api_base(),
            zone_id, rec_id
        );
        let res = client
            .put(&put_url)
            .header("Authorization", &auth)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let txt = res.text().await?;
        let j: serde_json::Value = serde_json::from_str(&txt)?;
        if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
            return Err(format!("DNS update {} failed: {}", p0, txt).into());
        }
        tracing::info!("  -> {} CNAME updated -> {}", p0, p1);
    } else {
        let post_url = format!(
            "{}/client/v4/zones/{}/dns_records",
            cf_api_base(),
            zone_id
        );
        let res = client
            .post(&post_url)
            .header("Authorization", &auth)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let txt = res.text().await?;
        let j: serde_json::Value = serde_json::from_str(&txt)?;
        if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
            return Err(format!("DNS create {} failed: {}", p0, txt).into());
        }
        tracing::info!("  -> {} CNAME created -> {}", p0, p1);
    }
    Ok(())
}

/// f97 = update_dns_record. A/AAAA for dynamic IP. CF_TOKEN in approuter.
pub async fn f97(
    zone_id: &str,
    record_id: &str,
    content: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required (set in approuter .env)")?;

    let rec_type = if content.contains(':') { "AAAA" } else { "A" };
    let body = json!({
        "type": rec_type,
        "content": content,
        "ttl": 300
    });

    let url = format!(
        "{}/client/v4/zones/{}/dns_records/{}",
        cf_api_base(),
        zone_id,
        record_id
    );
    let client = &*CF_CLIENT;
    let res = client
        .patch(&url)
        .header("Authorization", format!("Bearer {}", s8))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;
    if !status.is_success() {
        return Err(format!("Cloudflare DNS update failed {}: {}", status, text).into());
    }
    let j: serde_json::Value = serde_json::from_str(&text)?;
    if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
        let e = j
            .get("errors")
            .and_then(|e| e.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_else(|| text);
        return Err(format!("Cloudflare API failed: {}", e).into());
    }
    Ok(())
}

/// Add DNS CNAME for roguerepo.io and www.roguerepo.io -> tunnel. Requires CF_TOKEN.
pub async fn f93() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;
    let x0 = &*CF_CLIENT;
    let x1 = format!("Bearer {}", s8);

    tracing::info!("Looking up zone roguerepo.io");
    let x2 = x0
        .get("https://api.cloudflare.com/client/v4/zones?name=roguerepo.io")
        .header("Authorization", &x1)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let x3 = x2.text().await?;
    let x4: serde_json::Value = serde_json::from_str(&x3)?;
    if x4.get("success").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("Zones API failed: {}", x3).into());
    }
    let s6 = x4
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|z| z.get("id"))
        .and_then(|id| id.as_str())
        .ok_or("Zone roguerepo.io not found in Cloudflare account")?;

    async fn ensure_cname(
        client: &reqwest::Client,
        auth: &str,
        zone_id: &str,
        name: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}",
            zone_id, name
        );
        let r = client
            .get(&url)
            .header("Authorization", auth)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let j: serde_json::Value = serde_json::from_str(&r.text().await?)?;
        let rec = j.get("result").and_then(|r| r.as_array()).and_then(|a| a.first());
        let rec_name = if name == "roguerepo.io" { "@" } else { "www" };
        let body = json!({
            "type": "CNAME",
            "name": rec_name,
            "content": content,
            "ttl": 1,
            "proxied": true
        });
        let (method, path) = if let Some(r) = rec {
            let id = r.get("id").and_then(|i| i.as_str()).ok_or("No record id")?;
            (
                "PUT",
                format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                    zone_id, id
                ),
            )
        } else {
            (
                "POST",
                format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
                    zone_id
                ),
            )
        };
        let req = if method == "PUT" {
            client.put(&path)
        } else {
            client.post(&path)
        };
        let res = req
            .header("Authorization", auth)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let txt = res.text().await?;
        let j: serde_json::Value = serde_json::from_str(&txt)?;
        if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
            return Err(format!("DNS {} failed: {}", name, txt).into());
        }
        tracing::info!("  -> {} CNAME ok", name);
        Ok(())
    }

    ensure_cname(x0, &x1, s6, "roguerepo.io", C1).await?;
    ensure_cname(x0, &x1, s6, "www.roguerepo.io", C1).await?;

    tracing::info!("roguerepo.io DNS done. Run --update-tunnel to add hostnames to tunnel.");
    Ok(())
}

/// Add DNS CNAME for ronin-sites.pro zone -> tunnel. Requires CF_TOKEN.
pub async fn f94_ronin() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;
    let x0 = &*CF_CLIENT;
    let x1 = format!("Bearer {}", s8);

    tracing::info!("Looking up zone ronin-sites.pro");
    let x2 = x0
        .get("https://api.cloudflare.com/client/v4/zones?name=ronin-sites.pro")
        .header("Authorization", &x1)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let x3 = x2.text().await?;
    let x4: serde_json::Value = serde_json::from_str(&x3)?;
    if x4.get("success").and_then(|v| v.as_bool()) != Some(true) {
        return Err(format!("Zones API failed: {}", x3).into());
    }
    let s6 = x4
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|z| z.get("id"))
        .and_then(|id| id.as_str())
        .ok_or("Zone ronin-sites.pro not found in Cloudflare account")?;

    async fn ensure_cname(
        client: &reqwest::Client,
        auth: &str,
        zone_id: &str,
        name: &str,
        rec_name: &str,
        content: &str,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name={}",
            zone_id, name
        );
        let r = client
            .get(&url)
            .header("Authorization", auth)
            .header("Content-Type", "application/json")
            .send()
            .await?;
        let j: serde_json::Value = serde_json::from_str(&r.text().await?)?;
        let rec = j.get("result").and_then(|r| r.as_array()).and_then(|a| a.first());
        let body = json!({
            "type": "CNAME",
            "name": rec_name,
            "content": content,
            "ttl": 1,
            "proxied": true
        });
        let (method, path) = if let Some(r) = rec {
            let id = r.get("id").and_then(|i| i.as_str()).ok_or("No record id")?;
            (
                "PUT",
                format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
                    zone_id, id
                ),
            )
        } else {
            (
                "POST",
                format!(
                    "https://api.cloudflare.com/client/v4/zones/{}/dns_records",
                    zone_id
                ),
            )
        };
        let req = if method == "PUT" {
            client.put(&path)
        } else {
            client.post(&path)
        };
        let res = req
            .header("Authorization", auth)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;
        let txt = res.text().await?;
        let j: serde_json::Value = serde_json::from_str(&txt)?;
        if j.get("success").and_then(|v| v.as_bool()) != Some(true) {
            return Err(format!("DNS {} failed: {}", name, txt).into());
        }
        tracing::info!("  -> {} CNAME ok", name);
        Ok(())
    }

    ensure_cname(x0, &x1, s6, "ronin-sites.pro", "@", C1).await?;
    ensure_cname(x0, &x1, s6, "www.ronin-sites.pro", "www", C1).await?;
    ensure_cname(x0, &x1, s6, "*.ronin-sites.pro", "*", C1).await?;

    tracing::info!("ronin-sites.pro DNS done. Run --update-tunnel to add hostnames to tunnel.");
    Ok(())
}
