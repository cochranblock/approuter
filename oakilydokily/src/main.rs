// Copyright (c) 2026 The Cochran Block. All rights reserved.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

#[cfg(feature = "approuter")]
use approuter_client::{f116, RegisterConfig};
use oakilydokily::web::router;
use oakilydokily::{waiver, AppState};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if std::env::args().nth(1).as_deref() == Some("hash-password") {
        let pw = std::env::args().nth(2).unwrap_or_else(|| {
            eprintln!("Usage: oakilydokily hash-password PASSWORD");
            std::process::exit(1);
        });
        let hash = bcrypt::hash(&pw, bcrypt::DEFAULT_COST).expect("bcrypt hash");
        println!("{}", hash);
        return Ok(());
    }
    if std::env::args().nth(1).as_deref() == Some("hash-email") {
        let email = std::env::args().nth(2).unwrap_or_else(|| {
            eprintln!("Usage: oakilydokily hash-email EMAIL");
            std::process::exit(1);
        });
        let h = oakilydokily::web::auth::hash_email(&email);
        println!("{}", h);
        return Ok(());
    }
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    let data_dir = std::env::var("OAKILYDOKILY_DATA_DIR").unwrap_or_else(|_| {
        std::env::var("COCHRANBLOCK_DATA_ROOT")
            .ok()
            .map(|r| format!("{}/oakilydokily", r.trim_end_matches('/')))
            .or_else(|| {
                dirs::data_dir()
                    .map(|p| p.join("cochranblock").join("oakilydokily"))
                    .and_then(|p| p.to_str().map(String::from))
            })
            .unwrap_or_else(|| "data".into())
    });
    let pool = waiver::init_pool(Path::new(&data_dir)).await?;
    tracing::info!("waiver DB initialized at {}/waivers.sqlite", data_dir);

    let d1 = oakilydokily::d1_auth::f80_from_env();
    if d1.is_some() {
        tracing::info!("auth: D1 sharded storage enabled ({} shards)", d1.as_ref().map(|c| c.shard_count()).unwrap_or(0));
    }

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3000);
    let bind = std::env::var("BIND").unwrap_or_else(|_| "0.0.0.0".into());
    let addr = format!("{}:{}", bind, port);

    let app = router::router(AppState {
        s0: pool,
        s1: d1,
    });
    #[cfg(feature = "approuter")]
    f116(RegisterConfig {
        app_id: "oakilydokily",
        hostnames: std::env::var("OD_HOSTNAMES")
            .unwrap_or_else(|_| {
                "oakilydokily.com,www.oakilydokily.com,kaylie.cochranblock.org".into()
            })
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect(),
        backend_url: std::env::var("OD_BACKEND_URL")
            .unwrap_or_else(|_| "http://127.0.0.1:3000".into()),
    })
    .await;
    let listener = tokio::net::TcpListener::bind(&addr).await?;
    tracing::info!("oakilydokily listening on http://{}", addr);
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}
