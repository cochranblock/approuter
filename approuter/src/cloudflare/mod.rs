#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

mod dns;
mod tunnel;

pub use dns::*;
pub use tunnel::*;

use std::error::Error;
use std::sync::LazyLock;

/// Shared reqwest client for all Cloudflare API calls. Connection pooling, TLS session reuse.
pub(crate) static CF_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(|| {
    reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("reqwest client")
});

pub(crate) const C0: &str = "b12525df-6971-4c47-9a0d-61ee57a5cbd5";
pub(crate) const C1: &str = "b12525df-6971-4c47-9a0d-61ee57a5cbd5.cfargotunnel.com";

/// CF API base URL. Tests set CF_API_BASE_URL to point at wiremock.
pub(crate) fn cf_api_base() -> String {
    std::env::var("CF_API_BASE_URL").unwrap_or_else(|_| "https://api.cloudflare.com".into())
}

/// c91 = tunnel target (cfargotunnel.com)
pub fn c91() -> &'static str {
    C1
}

/// f94 = zone_from_hostname. www.example.com -> example.com.
pub(crate) fn f94(p0: &str) -> String {
    let v: Vec<&str> = p0.split('.').collect();
    if v.len() <= 2 {
        p0.to_string()
    } else {
        v[v.len() - 2..].join(".")
    }
}

/// Verify token via GET /user/tokens/verify. Returns status, id.
pub async fn verify_token(token: &str) -> Result<VerifyResult, Box<dyn Error + Send + Sync>> {
    let url = format!("{}/client/v4/user/tokens/verify", cf_api_base());
    let res = CF_CLIENT
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
    let res = CF_CLIENT
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

/// get_tunnel_token -- GET /accounts/{account_id}/cfd_tunnel/{tunnel_id}/token.
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

    let client = &*CF_CLIENT;
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
        .ok_or("Cloudflare token response missing result or result.token")?;
    Ok(token.to_string())
}
