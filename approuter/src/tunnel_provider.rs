//! tunnel_provider — Multi-tunnel abstraction. Cloudflare, ngrok, Tailscale Funnel, Bore, localtunnel.
//! Each provider spawns its own child process. Health checks + latency probes per provider.

// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

use std::collections::HashMap;
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::registry;
use crate::tunnel_metrics::t50;

/// t44 = TunnelKind. Which provider.
#[derive(Clone, Debug, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum t44 {
    Cloudflare,
    Ngrok,
    Tailscale,
    Bore,
    Localtunnel,
}

impl t44 {
    pub fn name(&self) -> &str {
        match self {
            t44::Cloudflare => "cloudflare",
            t44::Ngrok => "ngrok",
            t44::Tailscale => "tailscale",
            t44::Bore => "bore",
            t44::Localtunnel => "localtunnel",
        }
    }

    pub fn all() -> Vec<t44> {
        vec![t44::Cloudflare, t44::Ngrok, t44::Tailscale, t44::Bore, t44::Localtunnel]
    }

    pub fn from_str(s: &str) -> Option<t44> {
        match s.to_lowercase().as_str() {
            "cloudflare" | "cf" => Some(t44::Cloudflare),
            "ngrok" => Some(t44::Ngrok),
            "tailscale" | "ts" => Some(t44::Tailscale),
            "bore" => Some(t44::Bore),
            "localtunnel" | "lt" => Some(t44::Localtunnel),
            _ => None,
        }
    }
}

impl std::fmt::Display for t44 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

/// t45 = ProviderConfig. Per-provider settings from env.
#[derive(Clone, Debug)]
pub struct t45 {
    pub kind: t44,
    pub enabled: bool,
    /// Auth token (NGROK_AUTHTOKEN, etc.)
    pub token: Option<String>,
    /// Custom domain if provider supports it
    pub domain: Option<String>,
    /// Custom server for bore (default: bore.pub)
    pub server: Option<String>,
    /// Custom subdomain for localtunnel
    pub subdomain: Option<String>,
}

/// t46 = RunningTunnel. A spawned provider instance.
pub struct t46 {
    pub kind: t44,
    pub child: Child,
    pub pid: u32,
    pub started_at: u64,
    pub public_url: Option<String>,
}

/// t47 = TunnelManager. Manages all tunnel providers.
pub struct t47 {
    providers: RwLock<HashMap<t44, t46>>,
    configs: Vec<t45>,
    metrics: Arc<t50>,
    port: u16,
    base_dir: std::path::PathBuf,
}

impl t47 {
    pub fn new(port: u16, base_dir: &Path, metrics: Arc<t50>) -> Self {
        let configs = load_provider_configs();
        Self {
            providers: RwLock::new(HashMap::new()),
            configs,
            metrics,
            port,
            base_dir: base_dir.to_path_buf(),
        }
    }

    /// Spawn all enabled providers.
    pub fn spawn_all(&self, reg: &registry::t32) -> Vec<(t44, Result<(), String>)> {
        let mut results = Vec::new();
        for cfg in &self.configs {
            if !cfg.enabled { continue; }
            let r = self.spawn_provider(cfg, reg);
            results.push((cfg.kind.clone(), r));
        }
        results
    }

    /// Spawn a single provider.
    pub fn spawn_provider(&self, cfg: &t45, reg: &registry::t32) -> Result<(), String> {
        // Kill existing if running
        self.stop_provider(&cfg.kind);

        let (child, public_url) = match cfg.kind {
            t44::Cloudflare => spawn_cloudflare(&self.base_dir, reg, self.port)?,
            t44::Ngrok => spawn_ngrok(self.port, cfg)?,
            t44::Tailscale => spawn_tailscale(self.port, cfg)?,
            t44::Bore => spawn_bore(self.port, cfg)?,
            t44::Localtunnel => spawn_localtunnel(self.port, cfg)?,
        };

        let pid = child.id();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        self.metrics.record_start(&cfg.kind);

        let running = t46 {
            kind: cfg.kind.clone(),
            child,
            pid,
            started_at: now,
            public_url,
        };

        let mut providers = self.providers.write().unwrap();
        providers.insert(cfg.kind.clone(), running);
        tracing::info!("[tunnel] {} started (pid={})", cfg.kind.name(), pid);
        Ok(())
    }

    /// Stop a provider.
    pub fn stop_provider(&self, kind: &t44) {
        let mut providers = self.providers.write().unwrap();
        if let Some(mut t) = providers.remove(kind) {
            let _ = t.child.kill();
            self.metrics.record_stop(kind);
            tracing::info!("[tunnel] {} stopped (pid={})", kind.name(), t.pid);
        }
    }

    /// Stop all providers.
    pub fn stop_all(&self) {
        let mut providers = self.providers.write().unwrap();
        for (kind, mut t) in providers.drain() {
            let _ = t.child.kill();
            self.metrics.record_stop(&kind);
        }
    }

    /// Status of all providers.
    pub fn status_all(&self) -> Vec<serde_json::Value> {
        let mut providers = self.providers.write().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let mut out = Vec::new();
        for (kind, t) in providers.iter_mut() {
            let running = t.child.try_wait().ok().flatten().is_none();
            if !running {
                self.metrics.record_stop(kind);
            }
            out.push(serde_json::json!({
                "provider": kind.name(),
                "pid": t.pid,
                "running": running,
                "uptime_secs": now.saturating_sub(t.started_at),
                "public_url": t.public_url,
            }));
        }
        // Include configured-but-not-running
        for cfg in &self.configs {
            if cfg.enabled && !providers.contains_key(&cfg.kind) {
                out.push(serde_json::json!({
                    "provider": cfg.kind.name(),
                    "pid": null,
                    "running": false,
                    "uptime_secs": 0,
                    "public_url": null,
                }));
            }
        }
        out
    }

    /// Health check all running providers. Returns map of provider -> (reachable, latency_ms).
    pub async fn health_check_all(&self) -> HashMap<String, (bool, u64)> {
        let urls: Vec<(t44, String)> = {
            let providers = self.providers.read().unwrap();
            providers.iter()
                .filter_map(|(k, t)| t.public_url.as_ref().map(|u| (k.clone(), u.clone())))
                .collect()
        };

        let mut results = HashMap::new();
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap_or_else(|_| reqwest::Client::new());

        for (kind, url) in &urls {
            let start = Instant::now();
            let ok = match client.get(url).send().await {
                Ok(r) => r.status().as_u16() < 500,
                Err(_) => false,
            };
            let ms = start.elapsed().as_millis() as u64;
            self.metrics.record_probe(kind, ok, ms);
            results.insert(kind.name().to_string(), (ok, ms));
        }
        results
    }

    /// Get provider configs.
    pub fn configs(&self) -> &[t45] {
        &self.configs
    }

    /// Get a provider's public URL.
    pub fn public_url(&self, kind: &t44) -> Option<String> {
        let providers = self.providers.read().unwrap();
        providers.get(kind).and_then(|t| t.public_url.clone())
    }

    /// Check if a provider is running.
    pub fn is_running(&self, kind: &t44) -> bool {
        let mut providers = self.providers.write().unwrap();
        if let Some(t) = providers.get_mut(kind) {
            t.child.try_wait().ok().flatten().is_none()
        } else {
            false
        }
    }
}

/// Load provider configs from env vars.
fn load_provider_configs() -> Vec<t45> {
    let mut configs = Vec::new();

    // Cloudflare — enabled if CF_TOKEN or CF_TUNNEL_ID exists (existing behavior)
    let cf_enabled = std::env::var("CF_TOKEN").is_ok()
        || std::env::var("CLOUDFLARE_API_TOKEN").is_ok()
        || std::env::var("CF_TUNNEL_ID").is_ok();
    configs.push(t45 {
        kind: t44::Cloudflare,
        enabled: cf_enabled || std::env::var("TUNNEL_CLOUDFLARE").map(|v| v == "1").unwrap_or(false),
        token: std::env::var("CF_TOKEN").or_else(|_| std::env::var("CLOUDFLARE_API_TOKEN")).ok(),
        domain: None,
        server: None,
        subdomain: None,
    });

    // ngrok — enabled if TUNNEL_NGROK=1 or NGROK_AUTHTOKEN set
    let ngrok_token = std::env::var("NGROK_AUTHTOKEN").ok();
    configs.push(t45 {
        kind: t44::Ngrok,
        enabled: std::env::var("TUNNEL_NGROK").map(|v| v == "1").unwrap_or(false) || ngrok_token.is_some(),
        token: ngrok_token,
        domain: std::env::var("NGROK_DOMAIN").ok(),
        server: None,
        subdomain: None,
    });

    // Tailscale Funnel — enabled if TUNNEL_TAILSCALE=1
    configs.push(t45 {
        kind: t44::Tailscale,
        enabled: std::env::var("TUNNEL_TAILSCALE").map(|v| v == "1").unwrap_or(false),
        token: None,
        domain: None,
        server: None,
        subdomain: None,
    });

    // Bore — enabled if TUNNEL_BORE=1
    configs.push(t45 {
        kind: t44::Bore,
        enabled: std::env::var("TUNNEL_BORE").map(|v| v == "1").unwrap_or(false),
        token: std::env::var("BORE_SECRET").ok(),
        domain: None,
        server: std::env::var("BORE_SERVER").ok(),
        subdomain: None,
    });

    // localtunnel — enabled if TUNNEL_LOCALTUNNEL=1
    configs.push(t45 {
        kind: t44::Localtunnel,
        enabled: std::env::var("TUNNEL_LOCALTUNNEL").map(|v| v == "1").unwrap_or(false),
        token: None,
        domain: None,
        server: std::env::var("LT_HOST").ok(),
        subdomain: std::env::var("LT_SUBDOMAIN").ok(),
    });

    configs
}

// --- Provider spawn functions ---

fn spawn_cloudflare(base: &Path, reg: &registry::t32, port: u16) -> Result<(Child, Option<String>), String> {
    let child = crate::tunnel::f92(base, reg, port).map_err(|e| e.to_string())?;
    let tid = crate::tunnel::tunnel_id();
    let url = format!("https://{}.cfargotunnel.com", tid);
    Ok((child, Some(url)))
}

fn spawn_ngrok(port: u16, cfg: &t45) -> Result<(Child, Option<String>), String> {
    let bin = which_bin("ngrok").ok_or("ngrok not found in PATH")?;

    // Set authtoken if provided
    if let Some(ref token) = cfg.token {
        let _ = Command::new(&bin)
            .args(["config", "add-authtoken", token])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }

    let mut cmd = Command::new(&bin);
    cmd.arg("http").arg(port.to_string())
        .arg("--log").arg("stdout")
        .arg("--log-format").arg("json");

    if let Some(ref domain) = cfg.domain {
        cmd.arg("--domain").arg(domain);
    }

    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = cmd.spawn().map_err(|e| format!("ngrok spawn: {}", e))?;

    // ngrok exposes API at localhost:4040 — we'll grab the URL from there after startup.
    // For now return None; health check will fill it via the API.
    Ok((child, None))
}

fn spawn_tailscale(port: u16, _cfg: &t45) -> Result<(Child, Option<String>), String> {
    let bin = which_bin("tailscale").ok_or("tailscale not found in PATH")?;

    let mut cmd = Command::new(&bin);
    cmd.arg("funnel").arg(port.to_string())
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = cmd.spawn().map_err(|e| format!("tailscale funnel spawn: {}", e))?;

    // URL comes from `tailscale funnel status` — grab later
    Ok((child, None))
}

fn spawn_bore(port: u16, cfg: &t45) -> Result<(Child, Option<String>), String> {
    let bin = which_bin("bore").ok_or("bore not found in PATH")?;
    let server = cfg.server.as_deref().unwrap_or("bore.pub");

    let mut cmd = Command::new(&bin);
    cmd.arg("local").arg(port.to_string())
        .arg("--to").arg(server);

    if let Some(ref secret) = cfg.token {
        cmd.arg("--secret").arg(secret);
    }

    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = cmd.spawn().map_err(|e| format!("bore spawn: {}", e))?;

    // bore prints the URL to stdout: "listening at bore.pub:XXXXX"
    // We'll parse it from health check
    let url = format!("http://{}:<pending>", server);
    Ok((child, Some(url)))
}

fn spawn_localtunnel(port: u16, cfg: &t45) -> Result<(Child, Option<String>), String> {
    let bin = which_bin("lt").ok_or("lt (localtunnel) not found in PATH")?;

    let mut cmd = Command::new(&bin);
    cmd.arg("--port").arg(port.to_string());

    if let Some(ref subdomain) = cfg.subdomain {
        cmd.arg("--subdomain").arg(subdomain);
    }
    if let Some(ref host) = cfg.server {
        cmd.arg("--host").arg(host);
    }

    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let child = cmd.spawn().map_err(|e| format!("lt spawn: {}", e))?;

    // lt prints: "your url is: https://xxxxx.loca.lt"
    Ok((child, None))
}

fn which_bin(name: &str) -> Option<String> {
    // Check common locations then PATH
    for dir in ["/usr/local/bin", "/usr/bin", "/opt/homebrew/bin"] {
        let p = format!("{}/{}", dir, name);
        if std::path::Path::new(&p).exists() {
            return Some(p);
        }
    }
    // Try which
    Command::new("which")
        .arg(name)
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout).ok().map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

/// Resolve ngrok public URL from its local API (localhost:4040).
pub async fn resolve_ngrok_url() -> Option<String> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build().ok()?;
    let resp = client.get("http://127.0.0.1:4040/api/tunnels")
        .send().await.ok()?;
    let j: serde_json::Value = resp.json().await.ok()?;
    j["tunnels"].as_array()
        .and_then(|t| t.first())
        .and_then(|t| t["public_url"].as_str())
        .map(|s| s.to_string())
}

/// Resolve tailscale funnel URL from `tailscale status --json`.
pub async fn resolve_tailscale_url() -> Option<String> {
    let output = Command::new("tailscale")
        .args(["status", "--json"])
        .output().ok()?;
    if !output.status.success() { return None; }
    let j: serde_json::Value = serde_json::from_slice(&output.stdout).ok()?;
    let dns = j["Self"]["DNSName"].as_str()?;
    Some(format!("https://{}", dns.trim_end_matches('.')))
}
