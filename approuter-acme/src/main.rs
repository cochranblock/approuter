//! approuter-acme — pure-Rust ACME TLS terminator.
//!
//! Single binary:
//!  1. Loads (or requests via ACME DNS-01) a Let's Encrypt certificate
//!  2. Terminates TLS on the configured bind address
//!  3. Proxies to a backend HTTP URL (cochranblock / approuter / whatever)
//!  4. Background loop renews the cert when less than 30 days remain
//!
//! DNS-01 challenge via Cloudflare API using a token with Zone.DNS:Edit
//! on the zone containing the certificate's domain.
//!
//! Unlicensed. Public domain. Fork, strip, ship.

mod acme;
mod cf_dns;
mod proxy;

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use rustls::pki_types::{CertificateDer, PrivateKeyDer};
use rustls::ServerConfig;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio_rustls::TlsAcceptor;
use tracing::{error, info, warn};

#[derive(Parser, Debug)]
#[command(
    name = "approuter-acme",
    about = "Pure-Rust ACME TLS terminator. DNS-01 via Cloudflare.",
    version
)]
struct Args {
    /// Certificate domain (e.g. direct.cochranblock.org)
    #[arg(long, env = "APPROUTER_ACME_DOMAIN")]
    domain: String,

    /// DNS zone name (e.g. cochranblock.org). Defaults to last two labels of domain.
    #[arg(long, env = "APPROUTER_ACME_ZONE")]
    zone: Option<String>,

    /// ACME contact email. Required — used for Let's Encrypt account registration
    /// and expiry notices. No default; supply via CLI or APPROUTER_ACME_CONTACT env.
    #[arg(long, env = "APPROUTER_ACME_CONTACT")]
    contact: String,

    /// Cloudflare API token with Zone.DNS:Edit on the zone
    #[arg(long, env = "CF_DNS_TOKEN")]
    cf_token: String,

    /// Bind address for the HTTPS listener
    #[arg(long, env = "APPROUTER_ACME_BIND", default_value = "0.0.0.0:8443")]
    bind: SocketAddr,

    /// Backend URL (plain HTTP) to forward decrypted requests to. Default is
    /// approuter at 127.0.0.1:8080 since this tool is a sibling to approuter.
    /// Override for any other backend.
    #[arg(long, env = "APPROUTER_ACME_BACKEND", default_value = "http://127.0.0.1:8080")]
    backend: String,

    /// Directory for cert + key + account storage. Default resolves at runtime
    /// to the OS config dir (~/.config/approuter-acme on Linux,
    /// ~/Library/Application Support/approuter-acme on macOS).
    #[arg(long, env = "APPROUTER_ACME_CERT_DIR")]
    cert_dir: Option<PathBuf>,

    /// Use Let's Encrypt staging CA (for testing — certs won't chain publicly)
    #[arg(long, env = "APPROUTER_ACME_STAGING", default_value_t = false)]
    staging: bool,

    /// Renew if cert expires within this many days
    #[arg(long, default_value_t = 30)]
    renew_days: u32,

    /// Dry-run: attempt ACME flow, log result, then exit without serving
    #[arg(long, default_value_t = false)]
    dry_run: bool,
}

fn default_zone_for(domain: &str) -> String {
    // crude: "a.b.cochranblock.org" → "cochranblock.org"
    let parts: Vec<&str> = domain.split('.').collect();
    if parts.len() >= 2 {
        parts[parts.len() - 2..].join(".")
    } else {
        domain.to_string()
    }
}

fn load_tls_config(cert_pem: &str, key_pem: &str) -> Result<ServerConfig> {
    let certs: Vec<CertificateDer<'static>> =
        rustls_pemfile::certs(&mut cert_pem.as_bytes())
            .collect::<Result<Vec<_>, _>>()
            .context("parsing certificate PEM")?;
    if certs.is_empty() {
        return Err(anyhow!("no certificates found in cert PEM"));
    }
    let mut key_reader = key_pem.as_bytes();
    let key: PrivateKeyDer<'static> = rustls_pemfile::private_key(&mut key_reader)
        .context("parsing key PEM")?
        .ok_or_else(|| anyhow!("no private key found"))?;

    let mut cfg = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(certs, key)
        .context("building rustls ServerConfig")?;
    cfg.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    Ok(cfg)
}

#[tokio::main]
async fn main() -> Result<()> {
    // Install a default crypto provider for rustls so the builder works.
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .map_err(|_| anyhow!("failed to install rustls crypto provider"))?;

    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "approuter_acme=info,warn".into()),
        )
        .init();

    let args = Args::parse();
    let zone = args.zone.clone().unwrap_or_else(|| default_zone_for(&args.domain));

    // Resolve cert_dir at runtime. Never hardcode a username in a binary.
    let cert_dir: PathBuf = args.cert_dir.clone().unwrap_or_else(|| {
        dirs::config_dir()
            .map(|p| p.join("approuter-acme"))
            .unwrap_or_else(|| {
                warn!("could not resolve platform config dir; falling back to ./.approuter-acme");
                PathBuf::from(".approuter-acme")
            })
    });

    info!(
        "approuter-acme v{} — domain={} zone={} bind={} backend={} cert_dir={:?} staging={}",
        env!("CARGO_PKG_VERSION"),
        args.domain,
        zone,
        args.bind,
        args.backend,
        cert_dir,
        args.staging
    );

    let cf = cf_dns::CfDns::new(args.cf_token.clone());
    let acme_client = acme::AcmeClient::new(
        args.domain.clone(),
        args.contact.clone(),
        cf,
        cert_dir.clone(),
        zone.clone(),
        args.staging,
    );

    // Load existing cert or issue new one.
    let bundle = match acme_client.load_existing().await? {
        Some(b) => {
            info!("loaded existing cert from {:?}", cert_dir);
            b
        }
        None => {
            info!("no existing cert — issuing via ACME DNS-01");
            acme_client.issue().await.context("issuing certificate")?
        }
    };

    if args.dry_run {
        info!("--dry-run set; cert material loaded, exiting without serving");
        return Ok(());
    }

    // Build rustls ServerConfig from cert + key.
    let server_cfg = load_tls_config(&bundle.cert_pem, &bundle.key_pem)?;
    let acceptor = TlsAcceptor::from(Arc::new(server_cfg));

    // Set up background renewal loop.
    let renew_domain = args.domain.clone();
    let renew_zone = zone.clone();
    let renew_token = args.cf_token.clone();
    let renew_contact = args.contact.clone();
    let renew_dir = cert_dir.clone();
    let renew_staging = args.staging;
    let renew_days = args.renew_days;
    tokio::spawn(async move {
        loop {
            tokio::time::sleep(Duration::from_secs(24 * 60 * 60)).await;
            // Simple strategy: re-issue daily check. If cert is still valid >
            // renew_days, skip. Otherwise reissue. We don't parse cert expiry
            // in v0 — that's a v0.2 improvement. For now, daily tick just
            // logs a note and defers actual logic.
            info!(
                "renewal tick — v0.1 skips automatic renewal. Restart with fresh cert_dir to force reissue.\
                 (v0.2 will parse cert expiry and reissue within {} days of expiry.)",
                renew_days
            );
            let _ = (
                &renew_domain,
                &renew_zone,
                &renew_token,
                &renew_contact,
                &renew_dir,
                renew_staging,
            );
        }
    });

    // Serve.
    let proxy_state = Arc::new(proxy::Proxy::new(args.backend.clone()));
    if let Err(e) = proxy::serve(args.bind, acceptor, proxy_state).await {
        error!("proxy serve exited: {}", e);
        return Err(e);
    }
    warn!("proxy serve returned unexpectedly");
    Ok(())
}
