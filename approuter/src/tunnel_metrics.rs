//! tunnel_metrics — Per-provider latency, uptime, error tracking. Competitive comparison.

// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::RwLock;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::tunnel_provider::t44;

/// t48 = Single probe result.
#[derive(Clone, Serialize, Deserialize)]
pub struct t48 {
    pub ts: u64,
    pub provider: String,
    pub ok: bool,
    pub latency_ms: u64,
}

/// t49 = Per-provider aggregate metrics.
#[derive(Clone, Serialize, Deserialize, Default)]
pub struct t49 {
    pub provider: String,
    pub total_probes: u64,
    pub successful_probes: u64,
    pub failed_probes: u64,
    pub avg_latency_ms: f64,
    pub p50_latency_ms: u64,
    pub p95_latency_ms: u64,
    pub p99_latency_ms: u64,
    pub min_latency_ms: u64,
    pub max_latency_ms: u64,
    pub uptime_pct: f64,
    pub total_uptime_secs: u64,
    pub total_downtime_secs: u64,
    pub start_count: u64,
    pub current_streak_ok: u64,
    pub longest_streak_ok: u64,
}

/// t50 = MetricsStore. Thread-safe metrics collector for all providers.
pub struct t50 {
    probes: RwLock<Vec<t48>>,
    /// Per-provider: (start_time, is_running)
    lifecycle: RwLock<HashMap<String, ProviderLifecycle>>,
    max_probes: usize,
}

struct ProviderLifecycle {
    start_count: u64,
    last_start: Option<u64>,
    total_uptime_secs: u64,
    total_downtime_secs: u64,
    last_stop: Option<u64>,
    running: bool,
}

impl Default for ProviderLifecycle {
    fn default() -> Self {
        Self {
            start_count: 0,
            last_start: None,
            total_uptime_secs: 0,
            total_downtime_secs: 0,
            last_stop: None,
            running: false,
        }
    }
}

impl t50 {
    pub fn new() -> Self {
        Self {
            probes: RwLock::new(Vec::new()),
            lifecycle: RwLock::new(HashMap::new()),
            max_probes: 50_000,
        }
    }

    /// Record a health probe result.
    pub fn record_probe(&self, kind: &t44, ok: bool, latency_ms: u64) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let probe = t48 {
            ts: now,
            provider: kind.name().to_string(),
            ok,
            latency_ms,
        };
        let mut probes = self.probes.write().unwrap();
        probes.push(probe);
        if probes.len() > self.max_probes {
            let drain = probes.len() - self.max_probes;
            probes.drain(..drain);
        }
    }

    /// Record provider start.
    pub fn record_start(&self, kind: &t44) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let mut lc = self.lifecycle.write().unwrap();
        let entry = lc.entry(kind.name().to_string()).or_default();
        if entry.running {
            // Already running, accumulate uptime
            if let Some(start) = entry.last_start {
                entry.total_uptime_secs += now.saturating_sub(start);
            }
        } else if let Some(stop) = entry.last_stop {
            entry.total_downtime_secs += now.saturating_sub(stop);
        }
        entry.start_count += 1;
        entry.last_start = Some(now);
        entry.running = true;
    }

    /// Record provider stop.
    pub fn record_stop(&self, kind: &t44) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let mut lc = self.lifecycle.write().unwrap();
        let entry = lc.entry(kind.name().to_string()).or_default();
        if entry.running {
            if let Some(start) = entry.last_start {
                entry.total_uptime_secs += now.saturating_sub(start);
            }
        }
        entry.last_stop = Some(now);
        entry.running = false;
    }

    /// Get aggregate metrics for a single provider.
    pub fn provider_stats(&self, kind: &t44, hours: Option<u64>) -> t49 {
        let probes = self.probes.read().unwrap();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
        let cutoff = hours.map(|h| now.saturating_sub(h * 3600)).unwrap_or(0);
        let name = kind.name();

        let relevant: Vec<&t48> = probes.iter()
            .filter(|p| p.provider == name && p.ts >= cutoff)
            .collect();

        let mut latencies: Vec<u64> = relevant.iter().filter(|p| p.ok).map(|p| p.latency_ms).collect();
        latencies.sort();

        let total = relevant.len() as u64;
        let ok_count = relevant.iter().filter(|p| p.ok).count() as u64;
        let fail_count = total - ok_count;

        let avg = if !latencies.is_empty() {
            latencies.iter().sum::<u64>() as f64 / latencies.len() as f64
        } else { 0.0 };

        let percentile = |pct: f64| -> u64 {
            if latencies.is_empty() { return 0; }
            let idx = ((latencies.len() as f64 * pct) as usize).min(latencies.len() - 1);
            latencies[idx]
        };

        // Streak calculation
        let mut current_streak = 0u64;
        let mut longest_streak = 0u64;
        for p in relevant.iter().rev() {
            if p.ok {
                current_streak += 1;
                longest_streak = longest_streak.max(current_streak);
            } else {
                if current_streak > 0 { break; } // current streak broken
            }
        }

        // Lifecycle stats
        let lc = self.lifecycle.read().unwrap();
        let (uptime_secs, downtime_secs, start_count) = if let Some(entry) = lc.get(name) {
            let mut up = entry.total_uptime_secs;
            let mut down = entry.total_downtime_secs;
            // Add current running time
            if entry.running {
                if let Some(start) = entry.last_start {
                    up += now.saturating_sub(start);
                }
            } else if let Some(stop) = entry.last_stop {
                down += now.saturating_sub(stop);
            }
            (up, down, entry.start_count)
        } else {
            (0, 0, 0)
        };

        let uptime_pct = if uptime_secs + downtime_secs > 0 {
            (uptime_secs as f64 / (uptime_secs + downtime_secs) as f64) * 100.0
        } else { 0.0 };

        t49 {
            provider: name.to_string(),
            total_probes: total,
            successful_probes: ok_count,
            failed_probes: fail_count,
            avg_latency_ms: avg,
            p50_latency_ms: percentile(0.50),
            p95_latency_ms: percentile(0.95),
            p99_latency_ms: percentile(0.99),
            min_latency_ms: latencies.first().copied().unwrap_or(0),
            max_latency_ms: latencies.last().copied().unwrap_or(0),
            uptime_pct,
            total_uptime_secs: uptime_secs,
            total_downtime_secs: downtime_secs,
            start_count,
            current_streak_ok: current_streak,
            longest_streak_ok: longest_streak,
        }
    }

    /// Get comparison across all providers.
    pub fn comparison(&self, hours: Option<u64>) -> Vec<t49> {
        t44::all().iter().map(|k| self.provider_stats(k, hours)).filter(|s| s.total_probes > 0 || s.start_count > 0).collect()
    }

    /// Get recent probes (for live view).
    pub fn recent_probes(&self, limit: usize) -> Vec<t48> {
        let probes = self.probes.read().unwrap();
        probes.iter().rev().take(limit).cloned().collect()
    }

    /// Get raw probe history for a provider (for charting).
    pub fn probe_history(&self, kind: &t44, limit: usize) -> Vec<t48> {
        let probes = self.probes.read().unwrap();
        let name = kind.name();
        probes.iter().rev().filter(|p| p.provider == name).take(limit).cloned().collect()
    }
}
