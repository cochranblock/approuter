#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

//! metrics_catalog — per-request metrics catalog. Replaces the Cloudflare
//! Analytics dependency for operational telemetry; approuter becomes the
//! canonical source of truth for "what is happening at the front door."
//!
//! Design (P26 Moonshot Frame):
//! - Typed enums, no stringly-typed events: `IngressPath`, `UaClass`, `ErrorType`.
//! - Bounded storage: 10k-event ring, 168 hourly buckets, 1024 probe hits.
//! - Atomic counters for hot-path writes, `RwLock` ring for time-windowed reads.
//! - Single `record()` write path. Snapshot read paths are shape-per-consumer.
//! - Side-effect-free: no disk, no network. Persistence lives in analytics.rs.
//!
//! Tagging:
//! - `ingress_path` — which listener the request arrived on (`cf-tunnel` |
//!   `direct` | `lan`). Source of truth for multi-ingress rollout.
//! - `backend_app` — the `app_id` from the registry, or empty if the legacy
//!   routing path served the request.

use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

/// Where the request physically arrived at the proxy.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum IngressPath {
    /// Received via the Cloudflare tunnel outbound connection (default).
    #[default]
    CfTunnel,
    /// Received on the direct public listener (INGRESS_DIRECT_PORT).
    Direct,
    /// Received from a LAN/private source — RFC1918 or loopback.
    Lan,
}

impl IngressPath {
    pub fn as_str(self) -> &'static str {
        match self {
            IngressPath::CfTunnel => "cf-tunnel",
            IngressPath::Direct => "direct",
            IngressPath::Lan => "lan",
        }
    }

    /// Classify a connecting IP as LAN / public. Caller tells us which listener
    /// the request arrived on; this helper covers the ambiguity when both
    /// public and private traffic share a single listener.
    pub fn classify_direct_peer(peer_ip: &str) -> IngressPath {
        if peer_ip.is_empty() {
            return IngressPath::Direct;
        }
        if peer_ip.starts_with("127.")
            || peer_ip == "::1"
            || peer_ip.starts_with("10.")
            || peer_ip.starts_with("192.168.")
            || is_rfc1918_172(peer_ip)
            || peer_ip.starts_with("fc")
            || peer_ip.starts_with("fd")
        {
            IngressPath::Lan
        } else {
            IngressPath::Direct
        }
    }
}

fn is_rfc1918_172(ip: &str) -> bool {
    let mut parts = ip.splitn(3, '.');
    let (Some(a), Some(b)) = (parts.next(), parts.next()) else { return false };
    if a != "172" {
        return false;
    }
    match b.parse::<u8>() {
        Ok(n) if (16..=31).contains(&n) => true,
        _ => false,
    }
}

/// Coarse user-agent classification. Kept deliberately small and typed.
#[derive(Clone, Copy, Debug, Default, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum UaClass {
    Browser,
    Bot,
    Api,
    #[default]
    Unknown,
}

impl UaClass {
    pub fn from_ua(ua: &str) -> Self {
        if ua.is_empty() {
            return UaClass::Unknown;
        }
        let lower = ua.to_ascii_lowercase();
        if lower.contains("bot")
            || lower.contains("spider")
            || lower.contains("crawl")
            || lower.contains("scanner")
            || lower.contains("semrush")
            || lower.contains("ahrefs")
            || lower.contains("yandex")
            || lower.contains("headlesschrome")
            || lower.contains("phantomjs")
        {
            return UaClass::Bot;
        }
        if lower.contains("curl/")
            || lower.contains("wget")
            || lower.contains("python")
            || lower.contains("go-http")
            || lower.contains("postman")
            || lower.contains("httpie")
            || lower.contains("insomnia")
        {
            return UaClass::Api;
        }
        if lower.contains("mozilla/")
            || lower.contains("chrome/")
            || lower.contains("safari/")
            || lower.contains("firefox/")
            || lower.contains("edg/")
            || lower.contains("opr/")
        {
            return UaClass::Browser;
        }
        UaClass::Unknown
    }

    pub fn as_str(self) -> &'static str {
        match self {
            UaClass::Browser => "browser",
            UaClass::Bot => "bot",
            UaClass::Api => "api",
            UaClass::Unknown => "unknown",
        }
    }
}

/// Typed error classification. `None` = successful request.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum ErrorType {
    BadGateway,
    Timeout,
    BadRequest,
    UpstreamError,
    Forbidden,
    NotFound,
    Other,
}

impl ErrorType {
    pub fn from_status(status: u16) -> Option<Self> {
        match status {
            0..=399 => None,
            400 | 411 | 413 | 414 | 415 | 431 => Some(ErrorType::BadRequest),
            403 => Some(ErrorType::Forbidden),
            404 | 410 => Some(ErrorType::NotFound),
            408 | 504 => Some(ErrorType::Timeout),
            502 => Some(ErrorType::BadGateway),
            500..=599 => Some(ErrorType::UpstreamError),
            _ => Some(ErrorType::Other),
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ErrorType::BadGateway => "bad_gateway",
            ErrorType::Timeout => "timeout",
            ErrorType::BadRequest => "bad_request",
            ErrorType::UpstreamError => "upstream_error",
            ErrorType::Forbidden => "forbidden",
            ErrorType::NotFound => "not_found",
            ErrorType::Other => "other",
        }
    }
}

/// One captured request event. All fields are fixed-shape; no free-form maps.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestEvent {
    pub ts: u64,
    pub method: String,
    pub path: String,
    pub host: String,
    pub status_code: u16,
    pub response_bytes: u64,
    pub response_time_ms: u64,
    /// Truncated to /24 (IPv4) or /48 (IPv6) for privacy-preserving aggregates.
    pub client_ip_trunc: String,
    pub country: String,
    pub ua_class: UaClass,
    pub ingress_path: IngressPath,
    pub tls_version: Option<String>,
    pub http_version: String,
    pub cache_hit: bool,
    pub backend_app: String,
    pub backend_url: String,
    pub backend_latency_ms: u64,
    pub error_type: Option<ErrorType>,
}

/// Per-route running counters. Latencies kept in a bounded ring for percentiles.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RouteCounters {
    pub request_count: u64,
    pub error_count: u64,
    pub bytes_out: u64,
    #[serde(skip)]
    pub latencies_ms: VecDeque<u64>,
}

const ROUTE_LATENCY_WINDOW: usize = 2048;

/// One hour of aggregate traffic. 168 buckets = one rolling week.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HourlyBucket {
    pub hour_start_ts: u64,
    pub requests: u64,
    pub errors: u64,
    pub bytes_out: u64,
}

/// Hit against a known probe path. Auditors want every one of these visible.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProbeHit {
    pub ts: u64,
    pub path: String,
    pub host: String,
    pub client_ip_trunc: String,
    pub country: String,
    pub ua_class: UaClass,
}

/// Probe-path patterns matched against `path.to_ascii_lowercase()`. Match is
/// substring, not exact, so `/foo/wp-admin/bar` still hits.
const PROBE_PATTERNS: &[&str] = &[
    ".env",
    "wp-admin",
    "wp-login",
    "wp-config",
    "/.git",
    "phpmyadmin",
    "xmlrpc",
    "shell.php",
    "eval-stdin",
    "/administrator",
    ".aws/credentials",
    ".ssh/id_rsa",
    "/phpinfo",
    "/config.json",
    "/debug/pprof",
];

pub fn looks_like_probe(path: &str) -> bool {
    let lower = path.to_ascii_lowercase();
    PROBE_PATTERNS.iter().any(|p| lower.contains(p))
}

/// Truncate an IP to /24 (IPv4) or /48 (IPv6). Preserves geo/network utility
/// while removing individual identifiability.
pub fn truncate_ip(raw: &str) -> String {
    let raw = raw.trim();
    if raw.is_empty() {
        return String::new();
    }
    if raw.contains(':') {
        let head: Vec<&str> = raw.split(':').take(3).collect();
        format!("{}::/48", head.join(":"))
    } else {
        let parts: Vec<&str> = raw.split('.').collect();
        if parts.len() == 4 {
            format!("{}.{}.{}.0/24", parts[0], parts[1], parts[2])
        } else {
            raw.to_string()
        }
    }
}

/// Per-ingress aggregate counters.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct IngressCounters {
    pub request_count: u64,
    pub bytes_out: u64,
}

/// Public-facing snapshot: aggregate counts only. No IPs, paths, or UAs.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PublicMetricsSnapshot {
    pub uptime_s: u64,
    pub total_requests: u64,
    pub total_errors: u64,
    pub total_bytes_out: u64,
    pub ingress: HashMap<String, u64>,
    pub per_country: HashMap<String, u64>,
    pub hourly: Vec<HourlyBucket>,
    pub probe_paths_detected_count: usize,
}

/// Authenticated snapshot: full detail.
#[derive(Clone, Debug, Default, Serialize)]
pub struct MetricsSnapshot {
    pub uptime_s: u64,
    pub started_at: u64,
    pub total_requests: u64,
    pub total_bytes_out: u64,
    pub total_errors: u64,
    pub ingress: HashMap<String, IngressCounters>,
    pub per_route: Vec<RouteSnapshot>,
    pub top_countries_24h: Vec<(String, u64)>,
    pub top_paths_24h: Vec<(String, u64)>,
    pub top_user_agents_24h: Vec<(String, u64)>,
    pub probe_paths_detected: Vec<ProbeHit>,
    pub hourly: Vec<HourlyBucket>,
    pub event_ring_len: usize,
    pub event_ring_capacity: usize,
}

/// Per-route snapshot with 1h/24h counts and percentiles.
#[derive(Clone, Debug, Serialize)]
pub struct RouteSnapshot {
    pub route: String,
    pub request_count_1h: u64,
    pub request_count_24h: u64,
    pub error_count_1h: u64,
    pub bytes_out_1h: u64,
    pub bytes_out_24h: u64,
    pub p50_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,
}

const MAX_EVENTS: usize = 10_000;
const MAX_HOURLY: usize = 168;
const MAX_PROBE_HITS: usize = 1024;
const TOP_N_COUNTRIES: usize = 10;
const TOP_N_PATHS: usize = 25;

/// MetricsCatalog — the per-request store. Construct once, share as `Arc`.
pub struct MetricsCatalog {
    events: RwLock<VecDeque<RequestEvent>>,
    per_route: RwLock<HashMap<String, RouteCounters>>,
    hourly: RwLock<VecDeque<HourlyBucket>>,
    probe_hits: RwLock<VecDeque<ProbeHit>>,
    ingress_counts: RwLock<HashMap<IngressPath, IngressCounters>>,

    total_requests: AtomicU64,
    total_bytes_out: AtomicU64,
    total_errors: AtomicU64,

    started_at: u64,
}

impl Default for MetricsCatalog {
    fn default() -> Self {
        Self::new()
    }
}

impl MetricsCatalog {
    pub fn new() -> Self {
        let started_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        Self {
            events: RwLock::new(VecDeque::with_capacity(MAX_EVENTS)),
            per_route: RwLock::new(HashMap::new()),
            hourly: RwLock::new(VecDeque::with_capacity(MAX_HOURLY)),
            probe_hits: RwLock::new(VecDeque::with_capacity(MAX_PROBE_HITS)),
            ingress_counts: RwLock::new(HashMap::new()),
            total_requests: AtomicU64::new(0),
            total_bytes_out: AtomicU64::new(0),
            total_errors: AtomicU64::new(0),
            started_at,
        }
    }

    pub fn total_requests(&self) -> u64 {
        self.total_requests.load(Ordering::Relaxed)
    }
    pub fn total_bytes_out(&self) -> u64 {
        self.total_bytes_out.load(Ordering::Relaxed)
    }
    pub fn total_errors(&self) -> u64 {
        self.total_errors.load(Ordering::Relaxed)
    }
    pub fn started_at(&self) -> u64 {
        self.started_at
    }
    pub fn uptime_secs(&self) -> u64 {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.started_at)
    }

    /// Record one request. All writes are lock-local; no persistence.
    pub fn record(&self, event: RequestEvent) {
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        self.total_bytes_out
            .fetch_add(event.response_bytes, Ordering::Relaxed);
        if event.error_type.is_some() {
            self.total_errors.fetch_add(1, Ordering::Relaxed);
        }

        if looks_like_probe(&event.path) {
            if let Ok(mut probes) = self.probe_hits.write() {
                probes.push_back(ProbeHit {
                    ts: event.ts,
                    path: event.path.clone(),
                    host: event.host.clone(),
                    client_ip_trunc: event.client_ip_trunc.clone(),
                    country: event.country.clone(),
                    ua_class: event.ua_class,
                });
                while probes.len() > MAX_PROBE_HITS {
                    probes.pop_front();
                }
            }
            tracing::warn!(
                "[metrics] probe-path hit: host={} path={} country={} ip={}",
                event.host,
                event.path,
                event.country,
                event.client_ip_trunc
            );
        }

        if let Ok(mut map) = self.per_route.write() {
            let key = if event.backend_app.is_empty() {
                event.host.clone()
            } else {
                event.backend_app.clone()
            };
            let counters = map.entry(key).or_default();
            counters.request_count += 1;
            counters.bytes_out += event.response_bytes;
            if event.error_type.is_some() {
                counters.error_count += 1;
            }
            counters.latencies_ms.push_back(event.response_time_ms);
            while counters.latencies_ms.len() > ROUTE_LATENCY_WINDOW {
                counters.latencies_ms.pop_front();
            }
        }

        if let Ok(mut map) = self.ingress_counts.write() {
            let counters = map.entry(event.ingress_path).or_default();
            counters.request_count += 1;
            counters.bytes_out += event.response_bytes;
        }

        let hour_start = (event.ts / 3600) * 3600;
        if let Ok(mut hourly) = self.hourly.write() {
            let appended = matches!(hourly.back(), Some(b) if b.hour_start_ts == hour_start);
            if appended {
                if let Some(b) = hourly.back_mut() {
                    b.requests += 1;
                    b.bytes_out += event.response_bytes;
                    if event.error_type.is_some() {
                        b.errors += 1;
                    }
                }
            } else {
                hourly.push_back(HourlyBucket {
                    hour_start_ts: hour_start,
                    requests: 1,
                    errors: if event.error_type.is_some() { 1 } else { 0 },
                    bytes_out: event.response_bytes,
                });
                while hourly.len() > MAX_HOURLY {
                    hourly.pop_front();
                }
            }
        }

        if let Ok(mut ring) = self.events.write() {
            ring.push_back(event);
            while ring.len() > MAX_EVENTS {
                ring.pop_front();
            }
        }
    }

    /// Authenticated snapshot — full detail.
    pub fn snapshot(&self) -> MetricsSnapshot {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let cutoff_1h = now.saturating_sub(3600);
        let cutoff_24h = now.saturating_sub(86_400);

        let ingress: HashMap<String, IngressCounters> = self
            .ingress_counts
            .read()
            .map(|m| {
                m.iter()
                    .map(|(k, v)| (k.as_str().to_string(), v.clone()))
                    .collect()
            })
            .unwrap_or_default();

        let (top_countries, top_paths, top_ua, _event_len) = self.top_from_ring(cutoff_24h);

        let per_route = self.per_route_snapshot(cutoff_1h, cutoff_24h);

        let probe_paths_detected: Vec<ProbeHit> = self
            .probe_hits
            .read()
            .map(|p| p.iter().rev().take(100).cloned().collect())
            .unwrap_or_default();

        let hourly: Vec<HourlyBucket> = self
            .hourly
            .read()
            .map(|h| h.iter().cloned().collect())
            .unwrap_or_default();

        let (event_ring_len, _cap) = self
            .events
            .read()
            .map(|e| (e.len(), e.capacity()))
            .unwrap_or((0, 0));

        MetricsSnapshot {
            uptime_s: self.uptime_secs(),
            started_at: self.started_at,
            total_requests: self.total_requests(),
            total_bytes_out: self.total_bytes_out(),
            total_errors: self.total_errors(),
            ingress,
            per_route,
            top_countries_24h: top_countries,
            top_paths_24h: top_paths,
            top_user_agents_24h: top_ua,
            probe_paths_detected,
            hourly,
            event_ring_len,
            event_ring_capacity: MAX_EVENTS,
        }
    }

    /// Public snapshot — aggregate counts only.
    pub fn snapshot_public(&self) -> PublicMetricsSnapshot {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let cutoff_24h = now.saturating_sub(86_400);

        let ingress: HashMap<String, u64> = self
            .ingress_counts
            .read()
            .map(|m| {
                m.iter()
                    .map(|(k, v)| (k.as_str().to_string(), v.request_count))
                    .collect()
            })
            .unwrap_or_default();

        let mut per_country: HashMap<String, u64> = HashMap::new();
        if let Ok(ring) = self.events.read() {
            for e in ring.iter() {
                if e.ts < cutoff_24h {
                    continue;
                }
                if !e.country.is_empty() {
                    *per_country.entry(e.country.clone()).or_insert(0) += 1;
                }
            }
        }

        let hourly: Vec<HourlyBucket> = self
            .hourly
            .read()
            .map(|h| h.iter().cloned().collect())
            .unwrap_or_default();

        let probe_paths_detected_count = self
            .probe_hits
            .read()
            .map(|p| p.len())
            .unwrap_or(0);

        PublicMetricsSnapshot {
            uptime_s: self.uptime_secs(),
            total_requests: self.total_requests(),
            total_errors: self.total_errors(),
            total_bytes_out: self.total_bytes_out(),
            ingress,
            per_country,
            hourly,
            probe_paths_detected_count,
        }
    }

    /// Prometheus text format. One metric family per concern.
    pub fn prometheus_text(&self) -> String {
        let mut out = String::with_capacity(2048);
        let s = self.snapshot();

        out.push_str("# HELP approuter_uptime_seconds Proxy uptime.\n");
        out.push_str("# TYPE approuter_uptime_seconds gauge\n");
        out.push_str(&format!("approuter_uptime_seconds {}\n", s.uptime_s));

        out.push_str("# HELP approuter_requests_total Total requests served since start.\n");
        out.push_str("# TYPE approuter_requests_total counter\n");
        out.push_str(&format!("approuter_requests_total {}\n", s.total_requests));

        out.push_str("# HELP approuter_bytes_out_total Total response bytes since start.\n");
        out.push_str("# TYPE approuter_bytes_out_total counter\n");
        out.push_str(&format!(
            "approuter_bytes_out_total {}\n",
            s.total_bytes_out
        ));

        out.push_str("# HELP approuter_errors_total Total error responses since start.\n");
        out.push_str("# TYPE approuter_errors_total counter\n");
        out.push_str(&format!("approuter_errors_total {}\n", s.total_errors));

        out.push_str("# HELP approuter_requests_by_ingress Total requests by ingress path.\n");
        out.push_str("# TYPE approuter_requests_by_ingress counter\n");
        for (ingress, c) in &s.ingress {
            out.push_str(&format!(
                "approuter_requests_by_ingress{{path=\"{}\"}} {}\n",
                ingress, c.request_count
            ));
        }

        out.push_str(
            "# HELP approuter_route_p95_latency_ms Per-route p95 latency over recent window.\n",
        );
        out.push_str("# TYPE approuter_route_p95_latency_ms gauge\n");
        for r in &s.per_route {
            out.push_str(&format!(
                "approuter_route_p95_latency_ms{{route=\"{}\"}} {}\n",
                prom_label_escape(&r.route),
                r.p95_latency_ms
            ));
        }

        out.push_str("# HELP approuter_route_requests_1h Requests per route last hour.\n");
        out.push_str("# TYPE approuter_route_requests_1h gauge\n");
        for r in &s.per_route {
            out.push_str(&format!(
                "approuter_route_requests_1h{{route=\"{}\"}} {}\n",
                prom_label_escape(&r.route),
                r.request_count_1h
            ));
        }

        out.push_str("# HELP approuter_probe_hits_total Probe-path hits observed.\n");
        out.push_str("# TYPE approuter_probe_hits_total counter\n");
        out.push_str(&format!(
            "approuter_probe_hits_total {}\n",
            s.probe_paths_detected.len()
        ));

        out
    }

    fn top_from_ring(
        &self,
        cutoff_24h: u64,
    ) -> (Vec<(String, u64)>, Vec<(String, u64)>, Vec<(String, u64)>, usize) {
        let ring = match self.events.read() {
            Ok(r) => r,
            Err(_) => return (vec![], vec![], vec![], 0),
        };
        let mut countries: HashMap<String, u64> = HashMap::new();
        let mut paths: HashMap<String, u64> = HashMap::new();
        let mut uas: HashMap<String, u64> = HashMap::new();

        for e in ring.iter() {
            if e.ts < cutoff_24h {
                continue;
            }
            if !e.country.is_empty() {
                *countries.entry(e.country.clone()).or_insert(0) += 1;
            }
            *paths.entry(e.path.clone()).or_insert(0) += 1;
            *uas.entry(e.ua_class.as_str().to_string()).or_insert(0) += 1;
        }

        let sort_top = |m: HashMap<String, u64>, n: usize| -> Vec<(String, u64)> {
            let mut v: Vec<_> = m.into_iter().collect();
            v.sort_by(|a, b| b.1.cmp(&a.1));
            v.truncate(n);
            v
        };

        (
            sort_top(countries, TOP_N_COUNTRIES),
            sort_top(paths, TOP_N_PATHS),
            sort_top(uas, 16),
            ring.len(),
        )
    }

    fn per_route_snapshot(&self, cutoff_1h: u64, cutoff_24h: u64) -> Vec<RouteSnapshot> {
        // Per-route time-windowed counts come from the event ring; lifetime
        // latency percentiles come from `per_route`'s bounded windows.
        let ring = match self.events.read() {
            Ok(r) => r,
            Err(_) => return vec![],
        };
        let routes = match self.per_route.read() {
            Ok(r) => r,
            Err(_) => return vec![],
        };

        let mut by_route: HashMap<String, (u64, u64, u64, u64, u64)> = HashMap::new();
        //                                 (req_1h, req_24h, err_1h, bytes_1h, bytes_24h)
        for e in ring.iter() {
            let key = if e.backend_app.is_empty() {
                e.host.clone()
            } else {
                e.backend_app.clone()
            };
            let entry = by_route.entry(key).or_insert((0, 0, 0, 0, 0));
            if e.ts >= cutoff_24h {
                entry.1 += 1;
                entry.4 += e.response_bytes;
            }
            if e.ts >= cutoff_1h {
                entry.0 += 1;
                entry.3 += e.response_bytes;
                if e.error_type.is_some() {
                    entry.2 += 1;
                }
            }
        }

        let mut out = Vec::new();
        // Ensure every registered route surfaces even if no 24h traffic.
        for (route, rc) in routes.iter() {
            let (req_1h, req_24h, err_1h, bytes_1h, bytes_24h) = by_route
                .remove(route)
                .unwrap_or((0, 0, 0, 0, 0));
            let (p50, p95, p99) = percentiles(&rc.latencies_ms);
            out.push(RouteSnapshot {
                route: route.clone(),
                request_count_1h: req_1h,
                request_count_24h: req_24h,
                error_count_1h: err_1h,
                bytes_out_1h: bytes_1h,
                bytes_out_24h: bytes_24h,
                p50_latency_ms: p50,
                p95_latency_ms: p95,
                p99_latency_ms: p99,
            });
        }
        // Routes present in the ring but not (yet) in `per_route`.
        for (route, (req_1h, req_24h, err_1h, bytes_1h, bytes_24h)) in by_route.into_iter() {
            out.push(RouteSnapshot {
                route,
                request_count_1h: req_1h,
                request_count_24h: req_24h,
                error_count_1h: err_1h,
                bytes_out_1h: bytes_1h,
                bytes_out_24h: bytes_24h,
                p50_latency_ms: 0,
                p95_latency_ms: 0,
                p99_latency_ms: 0,
            });
        }
        out.sort_by(|a, b| b.request_count_24h.cmp(&a.request_count_24h));
        out
    }
}

fn percentiles(latencies: &VecDeque<u64>) -> (u64, u64, u64) {
    if latencies.is_empty() {
        return (0, 0, 0);
    }
    let mut v: Vec<u64> = latencies.iter().copied().collect();
    v.sort_unstable();
    let pick = |pct: f64| -> u64 {
        let idx = ((v.len() as f64 * pct) as usize).min(v.len() - 1);
        v[idx]
    };
    (pick(0.50), pick(0.95), pick(0.99))
}

fn prom_label_escape(raw: &str) -> String {
    raw.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', " ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ev(ts: u64, path: &str, status: u16, ingress: IngressPath, app: &str) -> RequestEvent {
        RequestEvent {
            ts,
            method: "GET".into(),
            path: path.into(),
            host: "cochranblock.org".into(),
            status_code: status,
            response_bytes: 1024,
            response_time_ms: 42,
            client_ip_trunc: "1.2.3.0/24".into(),
            country: "US".into(),
            ua_class: UaClass::Browser,
            ingress_path: ingress,
            tls_version: Some("TLS1.3".into()),
            http_version: "HTTP/2.0".into(),
            cache_hit: false,
            backend_app: app.into(),
            backend_url: "http://127.0.0.1:8081".into(),
            backend_latency_ms: 30,
            error_type: ErrorType::from_status(status),
        }
    }

    #[test]
    fn records_and_tops_out() {
        let m = MetricsCatalog::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        for i in 0..5 {
            m.record(ev(now - i * 10, "/", 200, IngressPath::CfTunnel, "cochranblock"));
        }
        m.record(ev(now, "/wp-admin/setup.php", 404, IngressPath::CfTunnel, ""));
        assert_eq!(m.total_requests(), 6);
        assert_eq!(m.total_errors(), 1);
        let s = m.snapshot();
        assert!(s.per_route.iter().any(|r| r.route == "cochranblock"));
        assert_eq!(s.probe_paths_detected.len(), 1);
    }

    #[test]
    fn ring_is_bounded() {
        let m = MetricsCatalog::new();
        let now = 1_700_000_000;
        for _ in 0..(MAX_EVENTS + 500) {
            m.record(ev(now, "/", 200, IngressPath::CfTunnel, "x"));
        }
        let s = m.snapshot();
        assert_eq!(s.event_ring_len, MAX_EVENTS);
    }

    #[test]
    fn ua_class_routing() {
        assert_eq!(UaClass::from_ua(""), UaClass::Unknown);
        assert_eq!(UaClass::from_ua("curl/8.0"), UaClass::Api);
        assert_eq!(UaClass::from_ua("Googlebot/2.1"), UaClass::Bot);
        assert!(matches!(
            UaClass::from_ua("Mozilla/5.0 (Macintosh; Intel Mac OS X) Chrome/120"),
            UaClass::Browser
        ));
    }

    #[test]
    fn error_type_from_status() {
        assert_eq!(ErrorType::from_status(200), None);
        assert_eq!(ErrorType::from_status(502), Some(ErrorType::BadGateway));
        assert_eq!(ErrorType::from_status(403), Some(ErrorType::Forbidden));
        assert_eq!(ErrorType::from_status(504), Some(ErrorType::Timeout));
    }

    #[test]
    fn truncate_ip_shapes() {
        assert_eq!(truncate_ip("192.168.1.55"), "192.168.1.0/24");
        assert_eq!(truncate_ip("2001:db8::1"), "2001:db8:::/48");
        assert_eq!(truncate_ip(""), "");
    }

    #[test]
    fn rfc1918_detection() {
        assert_eq!(IngressPath::classify_direct_peer("10.0.0.1"), IngressPath::Lan);
        assert_eq!(IngressPath::classify_direct_peer("172.17.1.5"), IngressPath::Lan);
        assert_eq!(IngressPath::classify_direct_peer("172.32.0.1"), IngressPath::Direct);
        assert_eq!(IngressPath::classify_direct_peer("8.8.8.8"), IngressPath::Direct);
    }

    #[test]
    fn probe_detector_hits() {
        assert!(looks_like_probe("/.env"));
        assert!(looks_like_probe("/wp-admin/setup.php"));
        assert!(looks_like_probe("/phpmyadmin/"));
        assert!(!looks_like_probe("/"));
        assert!(!looks_like_probe("/api/v1/users"));
    }

    #[test]
    fn hourly_bucketing() {
        let m = MetricsCatalog::new();
        let t0 = 1_700_000_000;
        // two events in same hour, one in the next
        m.record(ev(t0, "/", 200, IngressPath::CfTunnel, "x"));
        m.record(ev(t0 + 30, "/", 200, IngressPath::CfTunnel, "x"));
        m.record(ev(t0 + 3600, "/", 500, IngressPath::CfTunnel, "x"));
        let h = m.hourly.read().unwrap();
        assert_eq!(h.len(), 2);
        assert_eq!(h[0].requests, 2);
        assert_eq!(h[1].requests, 1);
        assert_eq!(h[1].errors, 1);
    }

    #[test]
    fn public_snapshot_redacts() {
        let m = MetricsCatalog::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        m.record(ev(now, "/secret/path", 200, IngressPath::Direct, "app-a"));
        let p = m.snapshot_public();
        assert_eq!(p.total_requests, 1);
        // public snapshot has no per-route or per-path detail
        assert_eq!(p.per_country.get("US").copied(), Some(1));
        assert_eq!(p.ingress.get("direct").copied(), Some(1));
    }

    #[test]
    fn prometheus_format_is_parseable() {
        let m = MetricsCatalog::new();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        m.record(ev(now, "/", 200, IngressPath::CfTunnel, "cochranblock"));
        let text = m.prometheus_text();
        assert!(text.contains("approuter_requests_total 1"));
        assert!(text.contains("approuter_requests_by_ingress{path=\"cf-tunnel\"} 1"));
    }
}
