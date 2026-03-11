// Copyright (c) 2026 The Cochran Block. All rights reserved.
//! kova c2 ssh-ca — SSH host certificate authority. No host key churn when IPs change.

use std::fs;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

/// Node -> (hostname, IP) for cert principals. From update-hosts-kova.sh.
const NODE_PRINCIPALS: &[(&str, &str, &str)] = &[
    ("lf", "kova-legion-forge.kova.inside", "192.168.1.47"),
    ("gd", "kova-tunnel-god.kova.inside", "192.168.1.44"),
    ("bt", "kova-thick-beast.kova.inside", "192.168.1.45"),
    ("st", "kova-elite-support.kova.inside", "192.168.1.43"),
];

fn ssh_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")).join(".ssh")
}

fn ca_key_path() -> PathBuf {
    ssh_dir().join("kova-host-ca")
}

fn known_hosts_path() -> PathBuf {
    ssh_dir().join("known_hosts")
}

fn principals_for_node(node: &str) -> String {
    NODE_PRINCIPALS
        .iter()
        .find(|(n, _, _)| *n == node)
        .map(|(n, host, ip)| format!("{},{},{}", n, host, ip))
        .unwrap_or_else(|| node.to_string())
}

/// Create CA key and add @cert-authority to known_hosts.
pub fn run_init() -> anyhow::Result<()> {
    let ca = ca_key_path();
    let kh = known_hosts_path();

    if let Some(parent) = ca.parent() {
        let _ = fs::create_dir_all(parent);
    }
    let ca_str = ca.to_string_lossy();

    if !ca.with_extension("pub").exists() {
        eprintln!("[ssh-ca] Creating CA key at {}", ca_str);
        let status = Command::new("ssh-keygen")
            .args(["-t", "ed25519", "-f", &ca_str, "-C", "kova-host-ca", "-N", ""])
            .status()?;
        if !status.success() {
            anyhow::bail!("ssh-keygen failed");
        }
    } else {
        eprintln!("[ssh-ca] CA key exists: {}", ca_str);
    }

    let pub_contents = fs::read_to_string(ca.with_extension("pub"))?;
    let line = format!("@cert-authority *.kova.inside {}", pub_contents.trim());

    if kh.exists() {
        let existing = fs::read_to_string(&kh)?;
        if existing.contains("@cert-authority") && existing.contains("kova-host-ca") {
            eprintln!("[ssh-ca] known_hosts already has CA");
            return Ok(());
        }
    }

    eprintln!("[ssh-ca] Adding @cert-authority to known_hosts");
    let mut f = fs::OpenOptions::new().append(true).create(true).open(&kh)?;
    writeln!(f, "\n# KOVA host CA (kova c2 ssh-ca)")?;
    writeln!(f, "{}", line)?;
    f.sync_all()?;

    eprintln!("[ssh-ca] Init done.");
    Ok(())
}

/// Sign host cert for one node and deploy.
pub fn run_sign(node: &str) -> anyhow::Result<()> {
    let ca = ca_key_path();
    if !ca.with_extension("pub").exists() {
        anyhow::bail!("Run init first: kova c2 ssh-ca init");
    }

    let principals = principals_for_node(node);

    // Fetch host key
    eprintln!("[ssh-ca] Fetching host key from {}...", node);
    let keyscan = Command::new("ssh-keyscan")
        .args(["-t", "ed25519", "-T", "5", node])
        .output()?;
    let stdout = String::from_utf8_lossy(&keyscan.stdout);
    let key_line = stdout
        .lines()
        .find(|l| !l.starts_with('#') && l.contains("ssh-ed25519"))
        .ok_or_else(|| anyhow::anyhow!("No host key from {}", node))?;

    // ssh-keyscan outputs "host key-type key-data"; .pub format is "key-type key-data [comment]"
    let pub_line = key_line
        .split_whitespace()
        .skip_while(|w| *w != "ssh-ed25519")
        .take(2)
        .collect::<Vec<_>>()
        .join(" ");
    if pub_line.is_empty() {
        anyhow::bail!("Could not parse host key from {}", node);
    }

    let tmp_dir = std::env::temp_dir().join(format!("kova-ssh-ca-{}", node));
    fs::create_dir_all(&tmp_dir)?;
    let host_key_path = tmp_dir.join("host_key.pub");
    fs::write(&host_key_path, &pub_line)?;

    eprintln!("[ssh-ca] Signing host cert for {}...", node);
    let status = Command::new("ssh-keygen")
        .args([
            "-s", ca.to_string_lossy().as_ref(),
            "-h",
            "-I", node,
            "-n", &principals,
            host_key_path.to_string_lossy().as_ref(),
        ])
        .status()?;
    if !status.success() {
        anyhow::bail!("ssh-keygen -s failed");
    }

    let signed_cert = tmp_dir.join("host_key-cert.pub");
    if !signed_cert.exists() {
        anyhow::bail!("Signed cert not produced");
    }

    eprintln!("[ssh-ca] Deploying cert to {}...", node);
    let status = Command::new("scp")
        .arg(&signed_cert)
        .arg(format!("{}:/tmp/ssh_host_ed25519_key-cert.pub", node))
        .status()?;
    if !status.success() {
        anyhow::bail!("scp failed");
    }

    let _ = fs::remove_dir_all(&tmp_dir);

    eprintln!(
        "[ssh-ca] On {} run:\n  sudo mv /tmp/ssh_host_ed25519_key-cert.pub /etc/ssh/\n  echo 'HostCertificate /etc/ssh/ssh_host_ed25519_key-cert.pub' | sudo tee -a /etc/ssh/sshd_config\n  sudo systemctl restart sshd",
        node
    );
    Ok(())
}

/// Init + sign all workers.
pub fn run_setup() -> anyhow::Result<()> {
    run_init()?;
    for (node, _, _) in NODE_PRINCIPALS {
        if let Err(e) = run_sign(node) {
            eprintln!("[ssh-ca] {} failed: {}", node, e);
        }
    }
    eprintln!("[ssh-ca] Setup complete. Restart sshd on each worker per instructions above.");
    Ok(())
}
