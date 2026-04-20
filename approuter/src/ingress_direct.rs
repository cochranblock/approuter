#![allow(non_camel_case_types, non_snake_case, dead_code)]

// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6

//! ingress_direct — second HTTP listener for non-tunnel ingress.
//!
//! Default approuter accepts traffic only via the cloudflared outbound tunnel.
//! When `INGRESS_DIRECT_PORT > 0`, this module binds a second listener so
//! traffic can also arrive directly (e.g. `WAN → Orbi → Mac mini:<port>`).
//!
//! The listener wraps the same axum router passed from `main.rs`, so routes
//! and the proxy fallback are identical. A per-request middleware layer
//! inspects the peer socket address and inserts an `IngressPath` extension
//! (`Direct` for public peers, `Lan` for RFC1918 / loopback). Downstream the
//! proxy handler (f58) reads that extension and tags every `RequestEvent`.
//!
//! Default: NOT bound. Rollout is config-gated, not build-gated — set
//! `INGRESS_DIRECT_PORT` and `INGRESS_DIRECT_BIND` to enable. This matches the
//! "build-only, no deploy" posture for the Orbi rollout.

use axum::{
    extract::ConnectInfo,
    http::Request,
    middleware::{self, Next},
    response::Response,
};
use std::net::SocketAddr;

use crate::metrics_catalog::IngressPath;

/// Configuration pulled from env. `port == 0` means "do not bind".
#[derive(Clone, Debug)]
pub struct DirectIngressConfig {
    pub bind: String,
    pub port: u16,
}

impl DirectIngressConfig {
    pub fn from_env() -> Self {
        let bind = std::env::var("INGRESS_DIRECT_BIND").unwrap_or_else(|_| "0.0.0.0".into());
        let port: u16 = std::env::var("INGRESS_DIRECT_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(0);
        Self { bind, port }
    }

    pub fn enabled(&self) -> bool {
        self.port > 0
    }
}

/// Spawn the direct listener if enabled. Returns `Ok(())` whether or not a
/// listener was bound — `port == 0` is a successful no-op.
pub async fn spawn_if_enabled(
    cfg: DirectIngressConfig,
    router: axum::Router,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if !cfg.enabled() {
        tracing::info!("[ingress-direct] disabled (INGRESS_DIRECT_PORT=0)");
        return Ok(());
    }
    let addr = format!("{}:{}", cfg.bind, cfg.port);
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("[ingress-direct] listening on http://{}", addr);

    let wrapped = router.layer(middleware::from_fn(tag_ingress_layer));

    tokio::spawn(async move {
        if let Err(e) = axum::serve(
            listener,
            wrapped.into_make_service_with_connect_info::<SocketAddr>(),
        )
        .await
        {
            tracing::error!("[ingress-direct] serve failed: {}", e);
        }
    });
    Ok(())
}

/// Per-request layer — tags the request with its ingress classification.
async fn tag_ingress_layer(mut req: Request<axum::body::Body>, next: Next) -> Response {
    let peer_ip = req
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ci| ci.0.ip().to_string())
        .unwrap_or_default();
    let tag = if peer_ip.is_empty() {
        IngressPath::Direct
    } else {
        IngressPath::classify_direct_peer(&peer_ip)
    };
    req.extensions_mut().insert(tag);
    next.run(req).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_defaults_disabled() {
        // Cannot toggle env vars safely in a multi-threaded test. Just assert
        // that an explicit port=0 config reads as disabled.
        let cfg = DirectIngressConfig {
            bind: "0.0.0.0".into(),
            port: 0,
        };
        assert!(!cfg.enabled());
    }

    #[test]
    fn config_enabled_when_port_set() {
        let cfg = DirectIngressConfig {
            bind: "0.0.0.0".into(),
            port: 443,
        };
        assert!(cfg.enabled());
    }
}
