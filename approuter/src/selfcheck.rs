#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

//! selfcheck — 60-second dual-path liveness probe. Runs two GETs every tick:
//!
//! 1. **CF path** — `https://cochranblock.org/health` via public DNS. Tests the
//!    `Visitor → Cloudflare → tunnel → approuter` path end-to-end.
//! 2. **Direct path** — `http://<external_ip>/health` on `INGRESS_DIRECT_PORT`.
//!    Tests the `Visitor → WAN → Orbi → approuter` path end-to-end.
//!
//! Records latency and success per path. On three consecutive failures of
//! either path, logs a WARN. **Does not auto-failover DNS** — that remains
//! explicit / manual per `approuter dns set A`.
//!
//! Exposes a `SelfCheckStore` snapshot suitable for the `/approuter/metrics`
//! endpoint. Events are bounded: last 2048 probes per path.

use serde::Serialize;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

const MAX_PROBES_PER_PATH: usize = 2048;
const WARN_AFTER_N_FAILS: u64 = 3;

/// Which probe path the record came from.
#[derive(Clone, Copy, Debug, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum ProbePath {
    Cf,
    Direct,
}

impl ProbePath {
    pub fn as_str(self) -> &'static str {
        match self {
            ProbePath::Cf => "cf",
            ProbePath::Direct => "direct",
        }
    }
}

/// One probe result.
#[derive(Clone, Debug, Serialize)]
pub struct SelfCheckProbe {
    pub ts: u64,
    pub path: ProbePath,
    pub ok: bool,
    pub status: u16,
    pub latency_ms: u64,
    pub error: Option<String>,
}

/// Per-path aggregate snapshot.
#[derive(Clone, Debug, Default, Serialize)]
pub struct PathSnapshot {
    pub total_probes: u64,
    pub successful_probes: u64,
    pub failed_probes: u64,
    pub consecutive_failures: u64,
    pub last_ok_ts: Option<u64>,
    pub last_fail_ts: Option<u64>,
    pub p50_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub recent: Vec<SelfCheckProbe>,
}

/// Overall snapshot surfaced to the metrics endpoint.
#[derive(Clone, Debug, Default, Serialize)]
pub struct SelfCheckSnapshot {
    pub last_check_ts: Option<u64>,
    pub external_ip: Option<String>,
    pub cf: PathSnapshot,
    pub direct: PathSnapshot,
}

#[derive(Default)]
struct PathState {
    probes: Vec<SelfCheckProbe>,
    total: u64,
    ok: u64,
    fail: u64,
    consecutive_failures: u64,
    last_ok_ts: Option<u64>,
    last_fail_ts: Option<u64>,
}

pub struct SelfCheckStore {
    cf: RwLock<PathState>,
    direct: RwLock<PathState>,
    external_ip: RwLock<Option<String>>,
    last_check_ts: AtomicU64,
}

impl Default for SelfCheckStore {
    fn default() -> Self {
        Self::new()
    }
}

impl SelfCheckStore {
    pub fn new() -> Self {
        Self {
            cf: RwLock::new(PathState::default()),
            direct: RwLock::new(PathState::default()),
            external_ip: RwLock::new(None),
            last_check_ts: AtomicU64::new(0),
        }
    }

    pub fn external_ip(&self) -> Option<String> {
        self.external_ip.read().ok().and_then(|g| g.clone())
    }

    pub fn set_external_ip(&self, ip: Option<String>) {
        if let Ok(mut g) = self.external_ip.write() {
            *g = ip;
        }
    }

    pub fn record(&self, probe: SelfCheckProbe) {
        self.last_check_ts.store(probe.ts, Ordering::Relaxed);
        let lock = match probe.path {
            ProbePath::Cf => &self.cf,
            ProbePath::Direct => &self.direct,
        };
        if let Ok(mut state) = lock.write() {
            state.total += 1;
            if probe.ok {
                state.ok += 1;
                state.consecutive_failures = 0;
                state.last_ok_ts = Some(probe.ts);
            } else {
                state.fail += 1;
                state.consecutive_failures += 1;
                state.last_fail_ts = Some(probe.ts);
                if state.consecutive_failures == WARN_AFTER_N_FAILS {
                    tracing::warn!(
                        "[selfcheck] {} path has failed {} checks in a row (last error: {:?}, status: {})",
                        probe.path.as_str(),
                        state.consecutive_failures,
                        probe.error,
                        probe.status,
                    );
                }
            }
            state.probes.push(probe);
            while state.probes.len() > MAX_PROBES_PER_PATH {
                state.probes.remove(0);
            }
        }
    }

    pub fn snapshot(&self) -> SelfCheckSnapshot {
        let last_ts = self.last_check_ts.load(Ordering::Relaxed);
        SelfCheckSnapshot {
            last_check_ts: if last_ts == 0 { None } else { Some(last_ts) },
            external_ip: self.external_ip(),
            cf: snapshot_path(&self.cf),
            direct: snapshot_path(&self.direct),
        }
    }
}

fn snapshot_path(lock: &RwLock<PathState>) -> PathSnapshot {
    let Ok(state) = lock.read() else {
        return PathSnapshot::default();
    };
    let mut latencies: Vec<u64> = state.probes.iter().filter(|p| p.ok).map(|p| p.latency_ms).collect();
    latencies.sort_unstable();
    let percentile = |pct: f64| -> u64 {
        if latencies.is_empty() {
            return 0;
        }
        let idx = ((latencies.len() as f64 * pct) as usize).min(latencies.len() - 1);
        latencies[idx]
    };
    let recent = state.probes.iter().rev().take(50).cloned().collect();
    PathSnapshot {
        total_probes: state.total,
        successful_probes: state.ok,
        failed_probes: state.fail,
        consecutive_failures: state.consecutive_failures,
        last_ok_ts: state.last_ok_ts,
        last_fail_ts: state.last_fail_ts,
        p50_latency_ms: percentile(0.50),
        p95_latency_ms: percentile(0.95),
        recent,
    }
}

/// Configuration for the self-check loop. All values read once at spawn.
#[derive(Clone, Debug)]
pub struct SelfCheckConfig {
    pub interval_secs: u64,
    pub cf_url: String,
    pub ext_ip_lookup_url: String,
    pub direct_port: u16,
    pub timeout: Duration,
}

impl SelfCheckConfig {
    pub fn from_env(direct_port: u16) -> Self {
        let interval_secs = std::env::var("SELFCHECK_INTERVAL_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);
        let cf_url = std::env::var("SELFCHECK_CF_URL")
            .unwrap_or_else(|_| "https://cochranblock.org/health".into());
        let ext_ip_lookup_url = std::env::var("SELFCHECK_EXT_IP_URL")
            .unwrap_or_else(|_| "https://api.ipify.org".into());
        let timeout_secs = std::env::var("SELFCHECK_TIMEOUT_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(5);
        Self {
            interval_secs,
            cf_url,
            ext_ip_lookup_url,
            direct_port,
            timeout: Duration::from_secs(timeout_secs),
        }
    }
}

/// Spawn the self-check loop. Returns immediately; the tokio task runs forever.
/// `direct_port == 0` skips the direct probe (no public ingress configured).
pub fn spawn_loop(store: std::sync::Arc<SelfCheckStore>, cfg: SelfCheckConfig) {
    tokio::spawn(async move {
        let client = match reqwest::Client::builder()
            .timeout(cfg.timeout)
            .redirect(reqwest::redirect::Policy::none())
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                tracing::warn!("[selfcheck] could not build client: {}", e);
                return;
            }
        };
        tracing::info!(
            "[selfcheck] starting — interval={}s cf_url={} direct_port={}",
            cfg.interval_secs,
            cfg.cf_url,
            cfg.direct_port
        );
        loop {
            tokio::time::sleep(Duration::from_secs(cfg.interval_secs)).await;
            let ext_ip = match client.get(&cfg.ext_ip_lookup_url).send().await {
                Ok(r) => r.text().await.ok().map(|s| s.trim().to_string()),
                Err(_) => None,
            };
            store.set_external_ip(ext_ip.clone());

            // CF path probe
            let probe = probe_once(&client, ProbePath::Cf, &cfg.cf_url).await;
            store.record(probe);

            // Direct path probe — only if a port is configured AND we know our IP
            if cfg.direct_port > 0 {
                if let Some(ref ip) = ext_ip {
                    let url = format!("http://{}:{}/health", ip, cfg.direct_port);
                    let probe = probe_once(&client, ProbePath::Direct, &url).await;
                    store.record(probe);
                } else {
                    store.record(SelfCheckProbe {
                        ts: now_secs(),
                        path: ProbePath::Direct,
                        ok: false,
                        status: 0,
                        latency_ms: 0,
                        error: Some("external IP unknown".into()),
                    });
                }
            }
        }
    });
}

async fn probe_once(client: &reqwest::Client, path: ProbePath, url: &str) -> SelfCheckProbe {
    let start = Instant::now();
    let ts = now_secs();
    match client.get(url).send().await {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let ok = resp.status().is_success();
            SelfCheckProbe {
                ts,
                path,
                ok,
                status,
                latency_ms: start.elapsed().as_millis() as u64,
                error: if ok { None } else { Some(format!("status {}", status)) },
            }
        }
        Err(e) => SelfCheckProbe {
            ts,
            path,
            ok: false,
            status: 0,
            latency_ms: start.elapsed().as_millis() as u64,
            error: Some(e.to_string()),
        },
    }
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn record_tracks_consecutive_failures() {
        let s = SelfCheckStore::new();
        for i in 0..4 {
            s.record(SelfCheckProbe {
                ts: 100 + i,
                path: ProbePath::Cf,
                ok: false,
                status: 502,
                latency_ms: 50,
                error: Some("boom".into()),
            });
        }
        let snap = s.snapshot();
        assert_eq!(snap.cf.consecutive_failures, 4);
        assert_eq!(snap.cf.failed_probes, 4);
    }

    #[test]
    fn one_success_resets_streak() {
        let s = SelfCheckStore::new();
        for _ in 0..5 {
            s.record(SelfCheckProbe {
                ts: 1, path: ProbePath::Direct, ok: false, status: 0, latency_ms: 5, error: None,
            });
        }
        s.record(SelfCheckProbe {
            ts: 2, path: ProbePath::Direct, ok: true, status: 200, latency_ms: 3, error: None,
        });
        let snap = s.snapshot();
        assert_eq!(snap.direct.consecutive_failures, 0);
        assert_eq!(snap.direct.successful_probes, 1);
    }

    #[test]
    fn ring_is_bounded() {
        let s = SelfCheckStore::new();
        for i in 0..(MAX_PROBES_PER_PATH + 100) {
            s.record(SelfCheckProbe {
                ts: i as u64,
                path: ProbePath::Cf,
                ok: true,
                status: 200,
                latency_ms: 1,
                error: None,
            });
        }
        let snap = s.snapshot();
        assert_eq!(snap.cf.total_probes, (MAX_PROBES_PER_PATH + 100) as u64);
        // `recent` is capped at 50
        assert_eq!(snap.cf.recent.len(), 50);
    }
}
