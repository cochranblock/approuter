// Unlicense — cochranblock.org
// Contributors: GotEmCoach, KOVA, Claude Opus 4.6, SuperNinja, Composer 1.5, Google Gemini Pro 3
#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals, dead_code)]

mod api;
mod cloudflare;
mod proxy;
mod registry;
mod restart;
mod run;
mod tunnel;

use approuter::setup;
use axum::routing::{delete, get, post};
use clap::{Parser, Subcommand};
use std::path::Path;
use std::sync::Arc;
use std::sync::Mutex;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};

#[derive(Parser)]
#[command(name = "approuter")]
struct Args {
    #[command(flatten)]
    pub serve: t28,
    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Subcommand)]
enum Cmd {
    PurgeCache,
    SetupCacheRules,
    SetupRateLimit,
    SetupTurnstile,
    SetupR2Bucket,
    SetupResendDns { #[arg(long, default_value = "roguerepo.io")] domain: String },
    SetupGoogleWorkspace { #[arg(long)] domain: String },
    SetupGoogleVerification { #[arg(long)] domain: String, #[arg(long)] value: String },
    AddCname { #[arg(long)] domain: String, #[arg(long)] name: String, #[arg(long)] target: String },
    FixTunnel,
    Build { #[arg(long, env = "BUILD_PACKAGE")] package: Option<String> },
    CloudflaredRun,
    MigrateData { #[arg(long, env = "MIGRATE_TARGET")] target: Option<std::path::PathBuf>, #[arg(long)] force: bool },
    SetupGoogleSa {
        #[arg(long, env = "GCP_PROJECT_ID")] project: Option<String>,
        #[arg(long, env = "GOOGLE_SA_NAME")] sa_name: Option<String>,
        #[arg(long, env = "GOOGLE_SA_KEY_FILE")] key_file: Option<String>,
    },
    EnvPrint,
    Restart,
    RestartOakilydokily,
    RestartCochranblock,
    RestartRonin,
    RestartRoguerepo,
    VerifyOrigin { #[arg(long, env = "VERIFY_ORIGIN_TIMEOUT", default_value = "120")] timeout: u64 },
    ListGoogleApis { #[arg(long)] free_only: bool, #[arg(long)] preferred: bool },
    SubmitSitemap {
        #[arg(long, default_value = "https://oakilydokily.com/")] site: String,
        #[arg(long, default_value = "https://oakilydokily.com/sitemap.xml")] sitemap: String,
    },
    StartAll {
        #[arg(long, default_value_t = false)] open: bool,
    },
    CfTokenCheck,
}

#[derive(Parser)]
pub struct t28 {
    #[arg(long, env = "ROUTER_PORT", default_value = "8080")]
    pub s16: u16,
    #[arg(long, env = "ROUTER_BIND", default_value = "127.0.0.1")]
    pub s17: String,
    #[arg(long, env = "ROUTER_COCHRANBLOCK_URL", default_value = "https://127.0.0.1:443")]
    pub s35: String,
    #[arg(long, env = "ROUTER_OAKILYDOKILY_URL", default_value = "http://127.0.0.1:3000")]
    pub s36: String,
    #[arg(long, env = "ROUTER_OAKILYDOKILY_HOST")]
    pub s37: Option<String>,
    #[arg(long, env = "ROUTER_OAKILYDOKILY_PATH")]
    pub s38: Option<String>,
    #[arg(long, env = "ROUTER_ROGUEREPO_URL", default_value = "http://127.0.0.1:3001")]
    pub s42: String,
    #[arg(long, env = "ROUTER_ROGUEREPO_HOST")]
    pub s43: Option<String>,
    #[arg(long, env = "ROUTER_RONIN_URL", default_value = "http://127.0.0.1:8000")]
    pub s49: String,
    #[arg(long, env = "ROUTER_RONIN_HOST")]
    pub s50: Option<String>,
    #[arg(long, env = "ROUTER_RONIN_SUFFIX")]
    pub s51: Option<String>,
    #[arg(long = "update-tunnel")]
    pub s39: bool,
    #[arg(long = "setup-oakilydokily")]
    pub s40: bool,
    #[arg(long = "setup-roguerepo")]
    pub s41: bool,
    #[arg(long = "setup-ronin")]
    pub s52: bool,
    #[arg(long = "no-tunnel", env = "ROUTER_NO_TUNNEL", default_value_t = false)]
    pub s44: bool,
    #[arg(long, env = "ROUTER_CONFIG_DIR")]
    pub s45: Option<std::path::PathBuf>,
}

fn main() {
    let _ = dotenvy::dotenv();
    let args = Args::parse();
    if let Some(cmd) = args.cmd {
        let root = setup::cb_root();
        let r: Result<(), Box<dyn std::error::Error + Send + Sync>> = match cmd {
            Cmd::PurgeCache => setup::f117(&root),
            Cmd::SetupCacheRules => setup::f118(&root),
            Cmd::SetupRateLimit => setup::f119(&root),
            Cmd::SetupTurnstile => setup::f120(&root),
            Cmd::SetupR2Bucket => setup::f121(&root),
            Cmd::SetupResendDns { domain } => setup::f122(&root, &domain),
            Cmd::SetupGoogleWorkspace { domain } => setup::f123(&root, &domain),
            Cmd::SetupGoogleVerification { domain, value } => setup::f124(&root, &domain, &value),
            Cmd::AddCname { domain, name, target } => setup::f125(&root, &domain, &name, &target),
            Cmd::FixTunnel => setup::f114(&root),
            Cmd::Build { package } => setup::f132(&root, package.as_deref()),
            Cmd::CloudflaredRun => setup::f133(&root),
            Cmd::MigrateData { target, force } => {
                let t = target.unwrap_or_else(|| std::path::PathBuf::from("/var/lib/cochranblock"));
                setup::f134(&root, &t, force)
            }
            Cmd::SetupGoogleSa { project, sa_name, key_file } => {
                setup::f135(&root, project.as_deref(), sa_name.as_deref(), key_file.as_deref())
            }
            Cmd::EnvPrint => {
                println!("export R=\"{}\"", std::env::var("R").unwrap_or_else(|_| ".".into()));
                println!("export U=\"{}\"", std::env::var("U").unwrap_or_else(|_| "http://127.0.0.1:8080".into()));
                Ok(())
            }
            Cmd::Restart => restart::f126(),
            Cmd::RestartOakilydokily => restart::f127(),
            Cmd::RestartCochranblock => restart::f128(),
            Cmd::RestartRonin => restart::f129(),
            Cmd::RestartRoguerepo => restart::f130(),
            Cmd::VerifyOrigin { timeout } => restart::f131(timeout),
            Cmd::ListGoogleApis { free_only, preferred } => setup::f136(free_only, preferred),
            Cmd::SubmitSitemap { site, sitemap } => setup::f137(&site, &sitemap),
            Cmd::StartAll { open } => run::start_all(open),
            Cmd::CfTokenCheck => run::cf_token_check(&root),
        };
        std::process::exit(r.map(|_| 0).unwrap_or_else(|_| 1));
    }
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("tokio runtime");
    if let Err(e) = rt.block_on(serve(args.serve)) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

async fn serve(p0: t28) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")))
        .init();

    if p0.s39 {
        cloudflare::f54(&p0).await?;
        return Ok(());
    }
    if p0.s40 {
        cloudflare::f53(&p0).await?;
        return Ok(());
    }
    if p0.s41 {
        cloudflare::f93().await?;
        return Ok(());
    }
    if p0.s52 {
        cloudflare::f94_ronin().await?;
        return Ok(());
    }

    let v0: Vec<String> = p0.s37
        .map(|v1| v1.split(',').map(|v2| v2.trim().to_string()).filter(|v2| !v2.is_empty()).collect())
        .unwrap_or_default();
    let v1: Vec<String> = p0.s43
        .map(|v2| v2.split(',').map(|v3| v3.trim().to_string()).filter(|v3| !v3.is_empty()).collect())
        .unwrap_or_default();
    let v3: Vec<String> = p0.s50
        .map(|v4| v4.split(',').map(|v5| v5.trim().to_string()).filter(|v5| !v5.is_empty()).collect())
        .unwrap_or_default();
    let v2 = Arc::new(proxy::t29 {
        s35: p0.s35.trim_end_matches('/').to_string(),
        s36: p0.s36.trim_end_matches('/').to_string(),
        s37: v0,
        s38: p0.s38,
        s42: p0.s42.trim_end_matches('/').to_string(),
        s43: v1,
        s49: p0.s49.trim_end_matches('/').to_string(),
        s50: v3,
        s51: p0.s51.clone(),
    });

    let base_dir = p0.s45
        .clone()
        .or_else(|| {
            std::env::var("COCHRANBLOCK_DATA_ROOT")
                .ok()
                .map(|r| std::path::PathBuf::from(r.trim_end_matches('/')).join("approuter"))
        })
        .or_else(|| dirs::data_dir().map(|p| p.join("cochranblock").join("approuter")))
        .or_else(|| std::env::current_dir().ok())
        .unwrap_or_else(|| Path::new(".").to_path_buf());
    let registry = Arc::new(registry::t32::new(&base_dir));

    if let Err(e) = tunnel::f91_gen(&base_dir, registry.as_ref(), p0.s16) {
        tracing::warn!("Could not generate tunnel config: {}", e);
    }

    let v0: Arc<Mutex<Option<std::process::Child>>> = if !p0.s44 {
        if let Err(e) = tunnel::f109(&base_dir).await {
            tracing::warn!("ensure-cloudflared failed: {}. Continuing without tunnel.", e);
            Arc::new(Mutex::new(None))
        } else if let Ok(v1) = tunnel::f92(&base_dir, registry.as_ref(), p0.s16) {
            Arc::new(Mutex::new(Some(v1)))
        } else {
            tracing::warn!("Tunnel spawn failed. Continuing without tunnel.");
            Arc::new(Mutex::new(None))
        }
    } else {
        Arc::new(Mutex::new(None))
    };

    let api_router = axum::Router::new()
        .route("/approuter", get(api::f109))
        .route("/approuter/", get(api::f109))
        .route("/approuter/register", post(api::f98))
        .route("/approuter/apps", get(api::f99))
        .route("/approuter/apps/:app_id", delete(api::f100))
        .route("/approuter/dns/update-a", post(api::f101))
        .route("/approuter/google/apis", get(api::f110))
        .route("/approuter/openapi.json", get(api::f103))
        .route("/approuter/tunnel", get(api::f104))
        .route("/approuter/tunnel/stop", post(api::f105))
        .route("/approuter/tunnel/ensure", post(api::f106))
        .route("/approuter/tunnel/restart", post(api::f107))
        .route("/approuter/tunnel/fix", post(api::f108))
        .with_state((registry.clone(), p0.s16, v0.clone(), base_dir.clone()));

    let r0 = api_router
        .merge(proxy::f55(v2, Some(registry)))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http());

    let v3 = format!("{}:{}", p0.s17, p0.s16);
    let v4 = tokio::net::TcpListener::bind(&v3).await?;
    tracing::info!("approuter listening on http://{}", v3);

    let v5 = v0.clone();
    let v6 = async move {
        tokio::signal::ctrl_c().await.ok();
        if let Ok(mut v7) = v5.lock() {
            if let Some(mut v8) = v7.take() {
                let _ = v8.kill();
            }
        }
    };

    axum::serve(v4, r0).with_graceful_shutdown(v6).await?;
    Ok(())
}
