//! Minimal HTTPS reverse proxy. Terminates TLS on the incoming listener,
//! forwards the plain-HTTP request to a single backend URL. Preserves
//! Host, X-Forwarded-For, X-Forwarded-Proto.

use anyhow::Result;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio_rustls::TlsAcceptor;
use tracing::{debug, warn};

pub struct Proxy {
    pub backend_url: String,
    pub client: reqwest::Client,
}

impl Proxy {
    pub fn new(backend_url: String) -> Self {
        Self {
            backend_url,
            client: reqwest::Client::builder()
                .redirect(reqwest::redirect::Policy::none())
                .timeout(std::time::Duration::from_secs(30))
                .build()
                .expect("reqwest client build"),
        }
    }

    pub async fn handle(
        self: Arc<Self>,
        peer: SocketAddr,
        req: Request<Incoming>,
    ) -> Result<Response<Full<Bytes>>> {
        let method = req.method().clone();
        // Clone as owned String so we release the borrow on `req` before consuming its body.
        let path_and_query: String = req
            .uri()
            .path_and_query()
            .map(|p| p.as_str())
            .unwrap_or("/")
            .to_string();
        let upstream_url = format!(
            "{}{}",
            self.backend_url.trim_end_matches('/'),
            path_and_query
        );
        let host_header = req
            .headers()
            .get(hyper::header::HOST)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_string();

        // Read original request body (consumes req).
        let body_bytes = req.into_body().collect().await?.to_bytes();

        // Build forwarded request.
        let mut builder = self
            .client
            .request(
                reqwest::Method::from_bytes(method.as_str().as_bytes())?,
                &upstream_url,
            )
            .body(body_bytes.to_vec())
            .header("X-Forwarded-For", peer.ip().to_string())
            .header("X-Forwarded-Proto", "https");
        if !host_header.is_empty() {
            builder = builder.header("Host", host_header);
        }

        debug!("proxying {} {} -> {}", method, path_and_query, upstream_url);
        let upstream_resp = builder.send().await?;
        let status = upstream_resp.status().as_u16();
        let mut builder = Response::builder().status(status);
        for (k, v) in upstream_resp.headers() {
            // hop-by-hop headers are dropped.
            let name = k.as_str().to_lowercase();
            if matches!(
                name.as_str(),
                "connection"
                    | "proxy-connection"
                    | "keep-alive"
                    | "transfer-encoding"
                    | "te"
                    | "trailer"
                    | "upgrade"
            ) {
                continue;
            }
            builder = builder.header(k.as_str(), v.as_bytes());
        }
        let body = upstream_resp.bytes().await?;
        Ok(builder.body(Full::new(body))?)
    }
}

/// Bind and serve HTTPS on `bind_addr` using `tls_acceptor`, forwarding
/// everything to `proxy`'s configured backend.
pub async fn serve(
    bind_addr: SocketAddr,
    tls_acceptor: TlsAcceptor,
    proxy: Arc<Proxy>,
) -> Result<()> {
    let listener = TcpListener::bind(bind_addr).await?;
    tracing::info!("approuter-acme TLS terminator listening on {}", bind_addr);

    loop {
        let (tcp, peer) = listener.accept().await?;
        let acceptor = tls_acceptor.clone();
        let proxy = proxy.clone();
        tokio::spawn(async move {
            let tls = match acceptor.accept(tcp).await {
                Ok(t) => t,
                Err(e) => {
                    warn!("tls accept from {}: {}", peer, e);
                    return;
                }
            };
            let io = TokioIo::new(tls);
            let service = hyper::service::service_fn(move |req| {
                let proxy = proxy.clone();
                async move {
                    match proxy.handle(peer, req).await {
                        Ok(resp) => Ok::<_, anyhow::Error>(resp),
                        Err(e) => {
                            warn!("proxy error from {}: {}", peer, e);
                            Ok(Response::builder()
                                .status(502)
                                .body(Full::new(Bytes::from("bad gateway")))?)
                        }
                    }
                }
            });
            if let Err(e) = hyper::server::conn::http1::Builder::new()
                .serve_connection(io, service)
                .await
            {
                debug!("connection from {} closed: {}", peer, e);
            }
        });
    }
}
