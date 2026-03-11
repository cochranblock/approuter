// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, dead_code)]

use crate::t28;
use serde_json::json;
use std::error::Error;

const C0: &str = "b12525df-6971-4c47-9a0d-61ee57a5cbd5";
const C1: &str = "b12525df-6971-4c47-9a0d-61ee57a5cbd5.cfargotunnel.com";

/// CF API base URL. Tests set CF_API_BASE_URL to point at wiremock.
fn cf_api_base() -> String {
    std::env::var("CF_API_BASE_URL").unwrap_or_else(|_| "https://api.cloudflare.com".into())
}

/// c91 = tunnel target (cfargotunnel.com)
pub fn c91() -> &'static str {
    C1
}

/// f94 = zone_from_hostname. www.example.com → example.com.
fn f94(p0: &str) -> String {
    let v: Vec<&str> = p0.split('.').collect();
    if v.len() <= 2 {
        p0.to_string()
    } else {
        v[v.len() - 2..].join(".")
    }
}

/// f95 = ensure_cname. hostname→tunnel. Zone must exist in CF.
pub async fn f95(
    p0: &str,
    p1: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;
    let client = reqwest::Client::new();
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

/// f96 = update_tunnel_from_registry. Registry→ingress. Apps register via API; tunnel syncs from registry.
pub async fn f96(
    p0: &crate::registry::t32,
    p1: u16,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required (set in approuter .env)")?;
    let s34 = std::env::var("CF_ACCOUNT_ID")
        .or_else(|_| std::env::var("CLOUDFLARE_ACCOUNT_ID"))
        .map_err(|_| "CF_ACCOUNT_ID or CLOUDFLARE_ACCOUNT_ID required")?;
    let tunnel_id = std::env::var("CF_TUNNEL_ID").unwrap_or_else(|_| C0.into());

    let service = format!("http://127.0.0.1:{}", p1);
    let hostnames: Vec<String> = p0.hostname_map().into_keys().collect();

    let mut ingress: Vec<serde_json::Value> = hostnames
        .into_iter()
        .map(|h| json!({ "hostname": h, "service": service }))
        .collect();
    ingress.push(json!({ "service": "http_status:404" }));

    let body = json!({ "config": { "ingress": ingress } });
    let url = format!(
        "{}/client/v4/accounts/{}/cfd_tunnel/{}/configurations",
        cf_api_base(),
        s34, tunnel_id
    );

    let client = reqwest::Client::new();
    let res = client
        .put(&url)
        .header("Authorization", format!("Bearer {}", s8))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;
    if !status.is_success() {
        return Err(format!("Cloudflare API error {}: {}", status, text).into());
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
    tracing::info!("Tunnel ingress updated from registry -> {}", service);
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
    let client = reqwest::Client::new();
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

pub async fn f53(a: &t28) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required (set in .env)")?;

    let x0 = reqwest::Client::new();
    let x1 = format!("Bearer {}", s8);

    tracing::info!("Step 1/4: Looking up zone oakilydokily.com");
    let x2 = x0
        .get("https://api.cloudflare.com/client/v4/zones?name=oakilydokily.com")
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
        .ok_or("Zone oakilydokily.com not found in Cloudflare account")?;

    let s34 = x4
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|z| z.get("account").and_then(|a| a.get("id")))
        .and_then(|id| id.as_str())
        .ok_or("Could not get account_id from zone")?;

    tracing::info!("  -> Zone found (zone_id={}, account_id={})", s6, s34);

    tracing::info!("Step 2/4: Ensuring CNAME oakilydokily.com + www -> {}", C1);
    let x5 = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name=oakilydokily.com",
        s6
    );
    let x6 = x0
        .get(&x5)
        .header("Authorization", &x1)
        .header("Content-Type", "application/json")
        .send()
        .await?;
    let x7 = x6.text().await?;
    let x8: serde_json::Value = serde_json::from_str(&x7)?;
    let x9 = x8
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first());

    let x10 = json!({
        "type": "CNAME",
        "name": "oakilydokily.com",
        "content": C1,
        "ttl": 1,
        "proxied": true
    });

    if let Some(r) = x9 {
        let s7 = r.get("id").and_then(|id| id.as_str()).ok_or("No record id")?;
        let x11 = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}",
            s6, s7
        );
        let x12 = x0
            .put(&x11)
            .header("Authorization", &x1)
            .header("Content-Type", "application/json")
            .json(&x10)
            .send()
            .await?;
        let x13 = x12.text().await?;
        let x14: serde_json::Value = serde_json::from_str(&x13)?;
        if x14.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let e = x14
                .get("errors")
                .and_then(|e| e.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                        .collect::<Vec<_>>()
                        .join("; ")
                })
                .unwrap_or_else(|| x13.clone());
            return Err(format!("DNS update failed: {}", e).into());
        }
        tracing::info!("  -> CNAME updated (record already existed)");
    } else {
        let x15 = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", s6);
        let x16 = x0
            .post(&x15)
            .header("Authorization", &x1)
            .header("Content-Type", "application/json")
            .json(&x10)
            .send()
            .await?;
        let x17 = x16.text().await?;
        let x18: serde_json::Value = serde_json::from_str(&x17)?;
        if x18.get("success").and_then(|v| v.as_bool()) != Some(true) {
            let e = x18
                .get("errors")
                .and_then(|e| e.as_array())
                .map(|a| {
                    a.iter()
                        .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                        .collect::<Vec<_>>()
                        .join("; ")
                })
                .unwrap_or_else(|| x17.clone());
            return Err(format!("DNS create failed: {}", e).into());
        }
        tracing::info!("  -> CNAME created");
    }

    let x5w = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records?name=www.oakilydokily.com",
        s6
    );
    let x6w = x0.get(&x5w).header("Authorization", &x1).header("Content-Type", "application/json").send().await?;
    let x7w = x6w.text().await?;
    let x8w: serde_json::Value = serde_json::from_str(&x7w)?;
    let x9w = x8w.get("result").and_then(|r| r.as_array()).and_then(|a| a.first());
    let x10w = json!({"type":"CNAME","name":"www","content":C1,"ttl":1,"proxied":true});
    if let Some(r) = x9w {
        let s7w = r.get("id").and_then(|id| id.as_str()).ok_or("No record id")?;
        let x11w = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}", s6, s7w);
        let x12w = x0.put(&x11w).header("Authorization", &x1).header("Content-Type", "application/json").json(&x10w).send().await?;
        let _ = x12w.text().await?;
        tracing::info!("  -> www CNAME updated");
    } else {
        let x15w = format!("https://api.cloudflare.com/client/v4/zones/{}/dns_records", s6);
        let x16w = x0.post(&x15w).header("Authorization", &x1).header("Content-Type", "application/json").json(&x10w).send().await?;
        let _ = x16w.text().await?;
        tracing::info!("  -> www CNAME created");
    }

    tracing::info!("Step 3/4: Updating tunnel ingress");
    let x19 = format!("http://127.0.0.1:{}", a.s16);
    let x20 = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/cfd_tunnel/{}/configurations",
        s34, C0
    );

    let x21 = json!({
        "config": {
            "ingress": [
                { "hostname": "cochranblock.org", "service": x19 },
                { "hostname": "www.cochranblock.org", "service": x19 },
                { "hostname": "roguerepo.io", "service": x19 },
                { "hostname": "www.roguerepo.io", "service": x19 },
                { "hostname": "oakilydokily.com", "service": x19 },
                { "hostname": "www.oakilydokily.com", "service": x19 },
                { "hostname": "ronin-sites.pro", "service": x19 },
                { "hostname": "www.ronin-sites.pro", "service": x19 },
                { "hostname": "*.ronin-sites.pro", "service": x19 },
                { "service": "http_status:404" }
            ]
        }
    });

    let x22 = x0
        .put(&x20)
        .header("Authorization", &x1)
        .header("Content-Type", "application/json")
        .json(&x21)
        .send()
        .await?;
    let x23 = x22.text().await?;
    let x24: serde_json::Value = serde_json::from_str(&x23)?;

    if x24.get("success").and_then(|v| v.as_bool()) != Some(true) {
        let e = x24
            .get("errors")
            .and_then(|e| e.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_else(|| x23.clone());
        return Err(format!("Tunnel config update failed: {}", e).into());
    }

    tracing::info!("Tunnel ingress updated: oakilydokily.com -> {}", x19);

    tracing::info!("Step 4/5: Removing oakilydokily redirect rules (fixes loop)");
    let x30 = format!(
        "https://api.cloudflare.com/client/v4/zones/{}/rulesets/phases/http_request_dynamic_redirect/entrypoint",
        s6
    );
    let x31 = x0.get(&x30).header("Authorization", &x1).header("Content-Type", "application/json").send().await?;
    let x32 = x31.text().await?;
    let x33: serde_json::Value = serde_json::from_str(&x32)?;
    let x34 = x33.get("result");
    let x35 = x34.and_then(|r| r.get("id")).and_then(|id| id.as_str());
    let x36 = x34.and_then(|r| r.get("rules")).and_then(|r| r.as_array()).map(|a| a.to_vec()).unwrap_or_default();
    let x37: Vec<serde_json::Value> = x36.into_iter().filter(|r| {
        let e = r.get("expression").and_then(|e| e.as_str()).unwrap_or("");
        !e.contains("oakilydokily")
    }).collect();
    let x38_body = json!({ "rules": x37 });
    if let Some(rid) = x35 {
        let x39 = format!("https://api.cloudflare.com/client/v4/zones/{}/rulesets/{}", s6, rid);
        let x40 = x0.put(&x39).header("Authorization", &x1).header("Content-Type", "application/json").json(&x38_body).send().await?;
        let x41 = x40.text().await?;
        let x42: serde_json::Value = serde_json::from_str(&x41)?;
        if x42.get("success").and_then(|v| v.as_bool()) == Some(true) {
            tracing::info!("  -> oakilydokily redirect rules removed");
        } else {
            tracing::warn!("  -> Redirect rules update failed (non-fatal): {}", x41);
        }
    } else {
        tracing::info!("  -> No redirect ruleset found (nothing to remove)");
    }

    tracing::info!("Step 4b/5: Purging zone cache");
    let x43 = format!("https://api.cloudflare.com/client/v4/zones/{}/purge_cache", s6);
    let x44 = x0.post(&x43).header("Authorization", &x1).header("Content-Type", "application/json").json(&json!({"purge_everything":true})).send().await?;
    let x45 = x44.text().await?;
    let x46: serde_json::Value = serde_json::from_str(&x45)?;
    if x46.get("success").and_then(|v| v.as_bool()) == Some(true) {
        tracing::info!("  -> Cache purged");
    } else {
        tracing::warn!("  -> Cache purge failed (non-fatal): {}", x45);
    }

    tracing::info!("Step 5/5: Enabling SSL (flexible) + Always Use HTTPS");
    for (name, val) in [("ssl", "flexible"), ("always_use_https", "on")] {
        let x25 = format!(
            "https://api.cloudflare.com/client/v4/zones/{}/settings/{}",
            s6, name
        );
        let x26 = x0
            .patch(&x25)
            .header("Authorization", &x1)
            .header("Content-Type", "application/json")
            .json(&json!({ "value": val }))
            .send()
            .await?;
        let x27 = x26.text().await?;
        let x28: serde_json::Value = serde_json::from_str(&x27)?;
        if x28.get("success").and_then(|v| v.as_bool()) != Some(true) {
            tracing::warn!("Zone setting {} failed: {} (non-fatal)", name, x27);
        } else {
            tracing::info!("  -> {} = {}", name, val);
        }
    }

    tracing::info!("Done. Ensure approuter is running and ROUTER_OAKILYDOKILY_HOST=oakilydokily.com");
    Ok(())
}

pub async fn f54(a: &t28) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s34 = std::env::var("CF_ACCOUNT_ID")
        .or_else(|_| std::env::var("CLOUDFLARE_ACCOUNT_ID"))
        .map_err(|_| "CF_ACCOUNT_ID or CLOUDFLARE_ACCOUNT_ID required")?;
    let x0 = std::env::var("CF_TUNNEL_ID").unwrap_or_else(|_| C0.into());
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;

    let base = a.s45
        .clone()
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| std::path::Path::new(".").to_path_buf());
    let registry = crate::registry::t32::new(&base);
    let hostnames: Vec<String> = registry.hostname_map().into_keys().collect();

    let x1 = format!("http://127.0.0.1:{}", a.s16);
    let mut ingress: Vec<serde_json::Value> = hostnames
        .into_iter()
        .map(|h| json!({ "hostname": h, "service": x1 }))
        .collect();
    ingress.push(json!({ "service": "http_status:404" }));

    let x2 = json!({ "config": { "ingress": ingress } });

    let x3 = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/cfd_tunnel/{}/configurations",
        s34, x0
    );

    let x4 = reqwest::Client::new();
    let x5 = x4
        .put(&x3)
        .header("Authorization", format!("Bearer {}", s8))
        .header("Content-Type", "application/json")
        .json(&x2)
        .send()
        .await?;

    let x6 = x5.status();
    let x7 = x5.text().await?;

    if !x6.is_success() {
        return Err(format!("Cloudflare API error {}: {}", x6, x7).into());
    }

    let x8: serde_json::Value = serde_json::from_str(&x7)?;
    if x8.get("success").and_then(|v| v.as_bool()) != Some(true) {
        let e = x8
            .get("errors")
            .and_then(|e| e.as_array())
            .map(|a| {
                a.iter()
                    .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                    .collect::<Vec<_>>()
                    .join("; ")
            })
            .unwrap_or_else(|| x7.to_string());
        return Err(format!("Cloudflare API failed: {}", e).into());
    }

    tracing::info!("Tunnel config updated: all hostnames now route to {}", x1);
    Ok(())
}

/// Add DNS CNAME for roguerepo.io and www.roguerepo.io -> tunnel. Requires CF_TOKEN.
pub async fn f93() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;
    let x0 = reqwest::Client::new();
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

    ensure_cname(&x0, &x1, s6, "roguerepo.io", C1).await?;
    ensure_cname(&x0, &x1, s6, "www.roguerepo.io", C1).await?;

    tracing::info!("roguerepo.io DNS done. Run --update-tunnel to add hostnames to tunnel.");
    Ok(())
}

/// Add DNS CNAME for ronin-sites.pro zone -> tunnel. Requires CF_TOKEN.
pub async fn f94_ronin() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;
    let x0 = reqwest::Client::new();
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

    ensure_cname(&x0, &x1, s6, "ronin-sites.pro", "@", C1).await?;
    ensure_cname(&x0, &x1, s6, "www.ronin-sites.pro", "www", C1).await?;
    ensure_cname(&x0, &x1, s6, "*.ronin-sites.pro", "*", C1).await?;

    tracing::info!("ronin-sites.pro DNS done. Run --update-tunnel to add hostnames to tunnel.");
    Ok(())
}

/// Verify token via GET /user/tokens/verify. Returns status, id.
pub async fn verify_token(token: &str) -> Result<VerifyResult, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/client/v4/user/tokens/verify", cf_api_base());
    let res = reqwest::Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let status_code = res.status();
    let text = res.text().await?;
    let j: serde_json::Value = serde_json::from_str(&text).unwrap_or_default();
    let ok = status_code.is_success() && j.get("success").and_then(|v| v.as_bool()).unwrap_or(false);
    let result = j.get("result");
    let status = result.and_then(|r| r.get("status")).and_then(|s| s.as_str()).unwrap_or(if status_code.is_success() { "active" } else { "error" });
    let id = result.and_then(|r| r.get("id"))
        .and_then(|i| i.as_str().map(String::from).or_else(|| i.as_u64().map(|n| n.to_string())))
        .unwrap_or_default();
    let expires = result.and_then(|r| r.get("expires_on")).and_then(|e| e.as_str()).unwrap_or("");
    Ok(VerifyResult { ok, status: status.into(), id, expires: expires.into() })
}

#[derive(Debug)]
pub struct VerifyResult {
    pub ok: bool,
    pub status: String,
    pub id: String,
    pub expires: String,
}

/// Check if token can fetch tunnel token. Returns Ok(true) if yes, Ok(false) if 403/401, Err for other.
pub async fn can_get_tunnel_token(
    token: &str,
    account_id: &str,
    tunnel_id: &str,
) -> Result<bool, Box<dyn Error + Send + Sync>> {
    let url = format!(
        "{}/client/v4/accounts/{}/cfd_tunnel/{}/token",
        cf_api_base(),
        account_id,
        tunnel_id
    );
    let res = reqwest::Client::new()
        .get(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;
    let status = res.status();
    let text = res.text().await?;
    if status.is_success() {
        let j: serde_json::Value = serde_json::from_str(&text)?;
        return Ok(j.get("success").and_then(|v| v.as_bool()).unwrap_or(false));
    }
    if status.as_u16() == 403 || status.as_u16() == 401 {
        return Ok(false);
    }
    Err(format!("{}: {}", status, text).into())
}

/// get_tunnel_token — GET /accounts/{account_id}/cfd_tunnel/{tunnel_id}/token.
/// Returns the token string for running cloudflared with --token.
pub async fn get_tunnel_token() -> Result<String, Box<dyn Error + Send + Sync>> {
    let s8 = std::env::var("CF_TOKEN")
        .or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN"))
        .map_err(|_| "CF_TOKEN or CLOUDFLARE_API_TOKEN required")?;
    let s34 = std::env::var("CF_ACCOUNT_ID")
        .or_else(|_| std::env::var("CLOUDFLARE_ACCOUNT_ID"))
        .map_err(|_| "CF_ACCOUNT_ID or CLOUDFLARE_ACCOUNT_ID required")?;
    let tunnel_id = std::env::var("CF_TUNNEL_ID").unwrap_or_else(|_| C0.into());

    let url = format!(
        "{}/client/v4/accounts/{}/cfd_tunnel/{}/token",
        cf_api_base(),
        s34,
        tunnel_id
    );

    let client = reqwest::Client::new();
    let res = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", s8))
        .header("Content-Type", "application/json")
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;
    if !status.is_success() {
        return Err(format!("Cloudflare API error {}: {}", status, text).into());
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
    // GET /token returns result as the token string directly (not result.token)
    let token = j
        .get("result")
        .and_then(|r| r.as_str())
        .or_else(|| {
            j.get("result")
                .and_then(|r| r.get("token"))
                .and_then(|t| t.as_str())
        })
        .ok_or_else(|| "Cloudflare token response missing result or result.token")?;
    Ok(token.to_string())
}
