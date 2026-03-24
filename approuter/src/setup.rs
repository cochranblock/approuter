//! f114–f137 setup commands. Stub until full implementation restored.

// Unlicense — cochranblock.org
// Contributors: Mattbusel (XFactor), GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3

use std::path::Path;

/// cb_root = cochranblock root. Default: current dir.
pub fn cb_root() -> std::path::PathBuf {
    std::env::current_dir().unwrap_or_else(|_| std::path::PathBuf::from("."))
}

pub fn f114(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f114 not implemented".into())
}
/// f117 = purge_cache. God mode: purge all Cloudflare zones under the account.
pub fn f117(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let token = std::env::var("CF_TOKEN").map_err(|_| "CF_TOKEN not set")?;
    let auth = format!("Bearer {}", token);
    let rt = tokio::runtime::Runtime::new()?;
    rt.block_on(async {
        let client = reqwest::Client::new();
        let zones: Vec<(String, String)> = match std::env::var("CF_ACCOUNT_ID") {
            Ok(acct) if !acct.is_empty() => {
                let r = client.get(format!("https://api.cloudflare.com/client/v4/zones?account.id={}", acct))
                    .header("Authorization", &auth).send().await?;
                let j: serde_json::Value = r.json().await?;
                j["result"].as_array()
                    .map(|a| a.iter().filter_map(|z| {
                        Some((z["id"].as_str()?.to_string(), z["name"].as_str()?.to_string()))
                    }).collect())
                    .unwrap_or_default()
            }
            _ => {
                let zid = std::env::var("CF_ZONE_ID").map_err(|_| "CF_ZONE_ID or CF_ACCOUNT_ID required")?;
                vec![(zid, "cochranblock.org".to_string())]
            }
        };
        for (id, name) in &zones {
            let r = client.post(format!("https://api.cloudflare.com/client/v4/zones/{}/purge_cache", id))
                .header("Authorization", &auth)
                .header("Content-Type", "application/json")
                .body(r#"{"purge_everything":true}"#)
                .send().await?;
            let j: serde_json::Value = r.json().await?;
            if j["success"].as_bool() == Some(true) {
                println!("purged {}", name);
            } else {
                println!("purge failed for {}: {}", name, j);
            }
        }
        Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
    })?;
    Ok(())
}
pub fn f118(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f118 not implemented".into())
}
pub fn f119(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f119 not implemented".into())
}
pub fn f120(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f120 not implemented".into())
}
pub fn f121(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f121 not implemented".into())
}
pub fn f122(_root: &Path, _domain: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f122 not implemented".into())
}
pub fn f123(_root: &Path, _domain: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f123 not implemented".into())
}
pub fn f124(_root: &Path, _domain: &str, _value: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f124 not implemented".into())
}
pub fn f125(_root: &Path, _domain: &str, _name: &str, _target: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f125 not implemented".into())
}
pub fn f132(_root: &Path, _package: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f132 not implemented".into())
}
pub fn f133(_root: &Path) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f133 not implemented".into())
}
pub fn f134(_root: &Path, _target: &std::path::Path, _force: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f134 not implemented".into())
}
pub fn f135(_root: &Path, _project: Option<&str>, _sa_name: Option<&str>, _key_file: Option<&str>) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f135 not implemented".into())
}
pub fn f136(_free_only: bool, _preferred: bool) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f136 not implemented".into())
}
pub fn f137(_site: &str, _sitemap: &str) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    Err("setup::f137 not implemented".into())
}