// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6
//! analytics — Server-side visitor analytics from Cloudflare geo headers.
//! Zero JS, zero cookies, zero GDPR consent needed. City-level geo for free.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

/// Single request event captured from CF headers.
#[derive(Clone, Serialize, Deserialize)]
pub struct t40 {
    pub ts: u64,
    pub host: String,
    pub path: String,
    pub method: String,
    pub status: u16,
    pub duration_ms: u64,
    pub country: String,
    pub region: String,
    pub region_code: String,
    pub city: String,
    pub timezone: String,
    pub ip_hash: String,
    pub ua_family: String,
    pub is_bot: bool,
}

/// Aggregated stats per site.
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct t41 {
    pub total_requests: u64,
    pub total_page_views: u64,
    pub unique_ips: u64,
    pub countries: HashMap<String, u64>,
    pub regions: HashMap<String, u64>,
    pub cities: HashMap<String, u64>,
    pub paths: HashMap<String, u64>,
    pub status_codes: HashMap<u16, u64>,
    pub ua_families: HashMap<String, u64>,
    pub requests_by_hour: HashMap<u8, u64>,
    pub bot_requests: u64,
    pub human_requests: u64,
}

/// Thread-safe analytics store.
pub struct t42 {
    events: Mutex<Vec<t40>>,
    data_dir: PathBuf,
    max_memory: usize,
}

impl t42 {
    pub fn new(base_dir: &Path) -> Self {
        let data_dir = base_dir.join("analytics");
        let _ = std::fs::create_dir_all(&data_dir);

        // Load today's events from disk
        let events = Self::load_today(&data_dir).unwrap_or_default();
        let count = events.len();
        if count > 0 {
            tracing::info!("[analytics] loaded {} events from disk", count);
        }

        Self {
            events: Mutex::new(events),
            data_dir,
            max_memory: 100_000,
        }
    }

    /// Record a request event.
    pub fn record(&self, event: t40) {
        let mut events = self.events.lock().unwrap();
        events.push(event);

        // Flush to disk every 100 events
        if events.len().is_multiple_of(100) {
            let _ = Self::save_events(&self.data_dir, &events);
        }

        // Trim old events from memory (keep last max_memory)
        if events.len() > self.max_memory {
            let drain = events.len() - self.max_memory;
            events.drain(..drain);
        }
    }

    /// Flush current events to disk.
    pub fn flush(&self) {
        let events = self.events.lock().unwrap();
        let _ = Self::save_events(&self.data_dir, &events);
    }

    /// Get aggregate stats, optionally filtered by host.
    pub fn stats(&self, host_filter: Option<&str>, hours: Option<u64>) -> t41 {
        let events = self.events.lock().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let cutoff = hours.map(|h| now.saturating_sub(h * 3600)).unwrap_or(0);

        let mut s = t41::default();
        let mut seen_ips = std::collections::HashSet::new();

        for e in events.iter() {
            if e.ts < cutoff { continue; }
            if let Some(h) = host_filter {
                if !e.host.contains(h) { continue; }
            }

            s.total_requests += 1;
            if !e.is_bot {
                s.human_requests += 1;
                // Count page views (HTML-likely requests)
                if e.status < 400 && (e.path == "/" || !e.path.contains('.') || e.path.ends_with(".html")) {
                    s.total_page_views += 1;
                }
            } else {
                s.bot_requests += 1;
            }

            if seen_ips.insert(e.ip_hash.clone()) {
                s.unique_ips += 1;
            }

            if !e.country.is_empty() {
                *s.countries.entry(e.country.clone()).or_default() += 1;
            }
            if !e.region.is_empty() {
                let key = format!("{}, {}", e.region, e.country);
                *s.regions.entry(key).or_default() += 1;
            }
            if !e.city.is_empty() {
                let key = if e.region_code.is_empty() {
                    format!("{}, {}", e.city, e.country)
                } else {
                    format!("{}, {} {}", e.city, e.region_code, e.country)
                };
                *s.cities.entry(key).or_default() += 1;
            }

            *s.paths.entry(e.path.clone()).or_default() += 1;
            *s.status_codes.entry(e.status).or_default() += 1;
            if !e.ua_family.is_empty() {
                *s.ua_families.entry(e.ua_family.clone()).or_default() += 1;
            }

            let hour = ((e.ts % 86400) / 3600) as u8;
            *s.requests_by_hour.entry(hour).or_default() += 1;
        }

        s
    }

    /// Get stats broken down by site.
    pub fn stats_all_sites(&self, hours: Option<u64>) -> HashMap<String, t41> {
        let events = self.events.lock().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let cutoff = hours.map(|h| now.saturating_sub(h * 3600)).unwrap_or(0);

        // Collect unique hosts
        let mut hosts: std::collections::HashSet<String> = std::collections::HashSet::new();
        for e in events.iter() {
            if e.ts >= cutoff {
                let h = e.host.split(':').next().unwrap_or(&e.host).to_string();
                hosts.insert(h);
            }
        }
        drop(events);

        let mut result = HashMap::new();
        for host in hosts {
            result.insert(host.clone(), self.stats(Some(&host), hours));
        }
        result
    }

    /// Get raw recent events (for live view).
    pub fn recent(&self, limit: usize, host_filter: Option<&str>) -> Vec<t40> {
        let events = self.events.lock().unwrap();
        events.iter()
            .rev()
            .filter(|e| host_filter.is_none_or(|h| e.host.contains(h)))
            .take(limit)
            .cloned()
            .collect()
    }

    fn today_file(dir: &Path) -> PathBuf {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let day = now / 86400;
        dir.join(format!("events_{}.jsonl", day))
    }

    fn load_today(dir: &Path) -> Option<Vec<t40>> {
        let path = Self::today_file(dir);
        let content = std::fs::read_to_string(&path).ok()?;
        let events: Vec<t40> = content.lines()
            .filter_map(|l| serde_json::from_str(l).ok())
            .collect();
        Some(events)
    }

    fn save_events(dir: &Path, events: &[t40]) -> Result<(), String> {
        let path = Self::today_file(dir);
        let content: String = events.iter()
            .map(|e| serde_json::to_string(e).unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n");
        std::fs::write(&path, content + "\n").map_err(|e| format!("save analytics: {}", e))
    }
}

/// Extract analytics event from request headers (Cloudflare geo headers).
pub fn extract_event(
    headers: &axum::http::HeaderMap,
    method: &str,
    path: &str,
    status: u16,
    duration_ms: u64,
) -> t40 {
    let get = |name: &str| -> String {
        headers.get(name)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string()
    };

    let host = get("host");
    let country = get("cf-ipcountry");
    let city = get("cf-ipcity");
    let region = get("cf-region");
    let region_code = get("cf-region-code");
    let timezone = get("cf-timezone");
    let real_ip = get("cf-connecting-ip");
    let ua = get("user-agent");

    // Hash IP for privacy (sha256 truncated)
    let ip_hash = if real_ip.is_empty() {
        String::new()
    } else {
        use sha2::{Sha256, Digest};
        let mut hasher = Sha256::new();
        hasher.update(real_ip.as_bytes());
        let result = hasher.finalize();
        format!("{:x}", result).chars().take(12).collect()
    };

    let ua_family = classify_ua(&ua);
    let is_bot = detect_bot(&ua);

    t40 {
        ts: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
        host: host.split(':').next().unwrap_or(&host).to_string(),
        path: path.to_string(),
        method: method.to_string(),
        status,
        duration_ms,
        country,
        region,
        region_code,
        city,
        timezone,
        ip_hash,
        ua_family,
        is_bot,
    }
}

fn classify_ua(ua: &str) -> String {
    if ua.contains("Edg/") { "Edge".into() }
    else if ua.contains("CriOS") { "Chrome iOS".into() }
    else if ua.contains("Chrome/") && ua.contains("Mobile") { "Chrome Mobile".into() }
    else if ua.contains("Chrome/") { "Chrome".into() }
    else if ua.contains("Safari/") && ua.contains("iPhone") { "Safari iPhone".into() }
    else if ua.contains("Safari/") && ua.contains("iPad") { "Safari iPad".into() }
    else if ua.contains("Safari/") && ua.contains("Macintosh") { "Safari Mac".into() }
    else if ua.contains("Firefox/") { "Firefox".into() }
    else if ua.contains("curl/") { "curl".into() }
    else if ua.is_empty() { "unknown".into() }
    else { "other".into() }
}

fn detect_bot(ua: &str) -> bool {
    let l = ua.to_lowercase();
    l.contains("bot") || l.contains("spider") || l.contains("crawl")
        || l.contains("curl/") || l.contains("python") || l.contains("go-http")
        || l.contains("wget") || l.contains("scanner") || l.contains("semrush")
        || l.contains("ahrefs") || l.contains("yandex") || l.contains("headlesschrome")
        || l.contains("phantomjs") || l.is_empty()
}
