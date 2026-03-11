//! f116=register_app. Self-register with approuter when ROUTER or APPROUTER_URL is set.
//! Used by cochranblock, oakilydokily.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

/// Config for approuter registration. Each binary passes its app_id, hostnames, backend_url.
pub struct RegisterConfig {
    pub app_id: &'static str,
    pub hostnames: Vec<String>,
    pub backend_url: String,
}

/// f116=register_app. No-op if ROUTER/APPROUTER_URL unset.
pub async fn f116(config: RegisterConfig) {
    let v0 = std::env::var("APPROUTER_URL")
        .or_else(|_| std::env::var("ROUTER"))
        .ok()
        .filter(|u| !u.is_empty())
        .map(|u| u.trim_end_matches('/').to_string());
    let v0 = match v0 {
        Some(b) => b,
        None => return,
    };

    if config.hostnames.is_empty() {
        tracing::warn!("{} hostnames empty, skipping approuter registration", config.app_id);
        return;
    }

    let v1 = match reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("reqwest client build failed: {}", e);
            return;
        }
    };

    let openapi_url = format!("{}/approuter/openapi.json", v0);
    let register_url = format!("{}/approuter/register", v0);

    for attempt in 1..=10 {
        if v1.get(&openapi_url).send().await.is_ok() {
            break;
        }
        if attempt < 10 {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        } else {
            tracing::warn!("approuter not reachable after 10 attempts, skipping registration");
            return;
        }
    }

    let v4 = serde_json::json!({
        "app_id": config.app_id,
        "hostnames": config.hostnames,
        "backend_url": config.backend_url.trim_end_matches('/')
    });

    match v1.post(&register_url).json(&v4).send().await {
        Ok(v5) if v5.status().is_success() => {
            tracing::info!("Registered with approuter: {} -> {}", config.app_id, config.backend_url);
        }
        Ok(v5) => {
            let status = v5.status();
            let body = v5.text().await.unwrap_or_default();
            tracing::warn!("approuter register failed {}: {}", status, body);
        }
        Err(e) => tracing::warn!("approuter register request failed: {}", e),
    }
}
