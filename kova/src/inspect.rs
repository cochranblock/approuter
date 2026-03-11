// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! kova c2 inspect — Gather CPU, RAM, disk, GPU from c2-core + workers.

use std::process::Command;

/// Host resource snapshot.
#[derive(Debug, Clone, Default)]
pub struct HostInfo {
    pub id: String,
    pub cores: Option<i32>,
    pub ram_gb: Option<i32>,
    pub disk_free_gb: Option<i64>,
    pub gpu: Option<String>,
    pub unreachable: bool,
}

impl HostInfo {
    fn unreachable(id: &str) -> Self {
        Self {
            id: id.to_string(),
            cores: None,
            ram_gb: None,
            disk_free_gb: None,
            gpu: None,
            unreachable: true,
        }
    }
}

/// Inspect c2-core (local) — macOS.
fn inspect_local_macos() -> HostInfo {
    let mut info = HostInfo {
        id: "c2-core".to_string(),
        ..Default::default()
    };

    // CPU cores
    if let Ok(out) = Command::new("sysctl").args(["-n", "hw.ncpu"]).output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Ok(n) = s.parse::<i32>() {
                info.cores = Some(n);
            }
        }
    }

    // RAM (bytes -> GB)
    if let Ok(out) = Command::new("sysctl").args(["-n", "hw.memsize"]).output() {
        if out.status.success() {
            let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
            if let Ok(bytes) = s.parse::<i64>() {
                info.ram_gb = Some((bytes / (1024 * 1024 * 1024)) as i32);
            }
        }
    }

    // Disk free (df / — 512-byte blocks on macOS; column 4 = Available)
    if let Ok(out) = Command::new("df").arg("/").output() {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
            let lines: Vec<&str> = text.lines().collect();
            if lines.len() >= 2 {
                let parts: Vec<&str> = lines[1].split_whitespace().collect();
                if parts.len() >= 4 {
                    if let Ok(blocks) = parts[3].parse::<i64>() {
                        info.disk_free_gb = Some((blocks * 512) / (1024 * 1024 * 1024));
                    }
                }
            }
        }
    }

    // GPU (system_profiler SPDisplaysDataType)
    if let Ok(out) = Command::new("system_profiler")
        .args(["SPDisplaysDataType"])
        .output()
    {
        if out.status.success() {
            let text = String::from_utf8_lossy(&out.stdout);
            // Look for "Chipset Model:" or "Metal:" line
            for line in text.lines() {
                let line = line.trim();
                if line.starts_with("Chipset Model:") {
                    info.gpu = Some(line.trim_start_matches("Chipset Model:").trim().to_string());
                    break;
                }
                if line.starts_with("Metal:") {
                    let rest = line.trim_start_matches("Metal:").trim();
                    if !rest.is_empty() && info.gpu.is_none() {
                        info.gpu = Some(rest.to_string());
                    }
                }
            }
        }
    }

    info
}

/// Inspect remote worker via SSH — Linux.
/// Expects SSH CA for host verification (@cert-authority in known_hosts); no host key churn when IPs change.
fn inspect_remote(node: &str) -> HostInfo {
    let cmd = r#"cores=$(nproc 2>/dev/null || echo 0)
ram=$(free -b 2>/dev/null | awk '/^Mem:/{print int($2/1024/1024/1024)}' || echo 0)
disk=$(df -B1 / 2>/dev/null | tail -1 | awk '{print $4}' || echo 0)
gpu=$(nvidia-smi --query-gpu=name,memory.total --format=csv,noheader 2>/dev/null | head -1 || lspci 2>/dev/null | grep -i vga | head -1 | sed 's/.*: //' || echo "")
echo "$cores"
echo "$ram"
echo "$disk"
echo "$gpu"
"#;

    let output = Command::new("ssh")
        .args([
            "-o", "ConnectTimeout=5",
            "-o", "StrictHostKeyChecking=accept-new",
            "-o", "BatchMode=yes",
            node,
            cmd,
        ])
        .output();

    match output {
        Ok(out) if out.status.success() => {
            let text = String::from_utf8_lossy(&out.stdout);
            let parts: Vec<&str> = text.lines().map(|s| s.trim()).collect();
            let mut info = HostInfo {
                id: node.to_string(),
                ..Default::default()
            };
            if !parts.is_empty() && !parts[0].is_empty() {
                if let Ok(n) = parts[0].parse::<i32>() {
                    info.cores = Some(n);
                }
            }
            if parts.len() >= 2 && !parts[1].is_empty() {
                if let Ok(n) = parts[1].parse::<i32>() {
                    info.ram_gb = Some(n);
                }
            }
            if parts.len() >= 3 && !parts[2].is_empty() {
                if let Ok(bytes) = parts[2].parse::<i64>() {
                    info.disk_free_gb = Some(bytes / (1024 * 1024 * 1024));
                }
            }
            if parts.len() >= 4 && !parts[3].is_empty() {
                info.gpu = Some(parts[3].to_string());
            }
            info
        }
        _ => HostInfo::unreachable(node),
    }
}

/// Run inspection on c2-core + all workers. Returns vec: [c2-core, lf, gd, bt, st].
pub fn run_inspect() -> Vec<HostInfo> {
    let mut results = Vec::new();

    // Local (c2-core)
    results.push(inspect_local_macos());

    // Remote workers (parallel via spawn)
    let nodes = crate::c2::default_nodes();
    let handles: Vec<_> = nodes
        .iter()
        .map(|n| {
            let node = *n;
            std::thread::spawn(move || inspect_remote(node))
        })
        .collect();

    for h in handles {
        if let Ok(info) = h.join() {
            results.push(info);
        }
    }

    results
}

/// Print human-readable table.
pub fn print_table(hosts: &[HostInfo]) {
    println!("{:<12} {:<8} {:<10} {:<14} GPU", "Host", "Cores", "RAM(GB)", "Disk(GB free)");
    println!("{}", "-".repeat(70));
    for h in hosts {
        let cores = h.cores.map(|n| n.to_string()).unwrap_or_else(|| "—".to_string());
        let ram = h.ram_gb.map(|n| n.to_string()).unwrap_or_else(|| "—".to_string());
        let disk = h.disk_free_gb.map(|n| n.to_string()).unwrap_or_else(|| "—".to_string());
        let gpu = h.gpu.as_deref().unwrap_or("—");
        let id = if h.unreachable {
            format!("{} (unreachable)", h.id)
        } else {
            h.id.clone()
        };
        println!("{:<12} {:<8} {:<10} {:<14} {}", id, cores, ram, disk, gpu);
    }
}

/// Print placement recommendations based on inspect data.
pub fn print_recommend(hosts: &[HostInfo]) {
    let workers: Vec<_> = hosts.iter().filter(|h| h.id != "c2-core" && !h.unreachable).collect();
    let max_ram = workers.iter().filter_map(|h| h.ram_gb).max().unwrap_or(0);
    let max_cores = workers.iter().filter_map(|h| h.cores).max().unwrap_or(0);
    let heavy_ram = workers.iter().find(|h| h.ram_gb == Some(max_ram) && max_ram >= 40);
    let max_cores_hosts: Vec<_> = workers.iter().filter(|h| h.cores == Some(max_cores)).collect();

    println!("\n--- Placement recommendations ---\n");

    println!("c2-core: Tunnel, approuter, web backends, kova GUI + LLM (Apple GPU)");
    println!("  -> Must run where tunnel terminates\n");

    if let Some(h) = heavy_ram {
        println!("Heavy cargo build (link-heavy, large tests): {}", h.id);
        println!("  -> {} GB RAM (most in swarm)\n", max_ram);
    }

    if !max_cores_hosts.is_empty() {
        let names: Vec<_> = max_cores_hosts.iter().map(|h| h.id.as_str()).collect();
        println!("Parallel crate builds (max {} cores): {}", max_cores, names.join(", "));
        println!("  -> Use sshallp or kova c2 run f18 --broadcast\n");
    }

    if let Some(st) = workers.iter().find(|h| h.id == "st") {
        if let Some(d) = st.disk_free_gb {
            if d < 50 {
                println!("WARNING: st disk low ({} GB free). Hive/NFS export at risk.", d);
            } else {
                println!("st: NFS export /mnt/hive. Use for rsync target, hive setup.");
            }
        }
    }

    let gpu_workers: Vec<_> = workers.iter().filter(|h| h.gpu.as_ref().map(|s| !s.is_empty()).unwrap_or(false)).collect();
    if !gpu_workers.is_empty() {
        let list: Vec<String> = gpu_workers.iter().map(|h| format!("{} ({})", h.id, h.gpu.as_deref().unwrap_or(""))).collect();
        println!("\nWorkers with GPU: {}", list.join(", "));
        println!("  -> Offload batch inference, training from c2-core");
    }

    // Actionable copy-paste commands
    println!("\n--- Copy-paste commands ---\n");
    let hive_note = if workers.is_empty() {
        "Run kova c2 sync first."
    } else {
        ""
    };
    if let Some(h) = heavy_ram {
        println!(
            "# Heavy build ({}):\nssh {} \"cd /mnt/hive/projects/workspace && cargo build --release -p rogue-repo\"\n",
            h.id, h.id
        );
    }
    if !workers.is_empty() {
        println!(
            "# Parallel build (all workers):\nsshallp \"cd /mnt/hive/projects/workspace && cargo build --release\"\n"
        );
        println!(
            "# Full pipeline broadcast:\nkova c2 run f20 --project ~/hive-vault/projects/workspace/rogue-repo --broadcast\n"
        );
    }
    if !hive_note.is_empty() {
        println!("{}", hive_note);
    }
}

/// Serialize to JSON for scripting.
pub fn print_json(hosts: &[HostInfo]) {
    let items: Vec<serde_json::Value> = hosts
        .iter()
        .map(|h| {
            serde_json::json!({
                "id": h.id,
                "cores": h.cores,
                "ram_gb": h.ram_gb,
                "disk_free_gb": h.disk_free_gb,
                "gpu": h.gpu,
                "unreachable": h.unreachable,
            })
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({ "hosts": items })).unwrap_or_default());
}
