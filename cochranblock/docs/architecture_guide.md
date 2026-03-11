<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Rust Portfolio Web Server — Comprehensive Architecture Guide
## For Michael Cochran | Single-Binary Portfolio + Dynamic DNS

---

# TABLE OF CONTENTS

1. [A. Architecture Overview](#a-architecture-overview)
2. [B. Technology Recommendations](#b-technology-recommendations)
3. [C. Project Structure](#c-project-structure)
4. [D. Implementation Guidance](#d-implementation-guidance)
5. [E. Frontend Approach](#e-frontend-approach)
6. [F. Security Best Practices](#f-security-best-practices)
7. [G. Deployment Considerations](#g-deployment-considerations)
8. [H. Answers to Specific Questions](#h-answers-to-specific-questions)

> **NOTE:** Code in this document uses the compression notation defined in `../kova/docs/compression_map.md`.
> All compact identifiers (f0-f28, t0-t17, etc.) resolve via that mapping table.
> Structural templates (@0-@5), atomic operations (Ω0-Ω10), sequences (Σ0-Σ5), and
> function templates (Φ0-Φ4) are defined there. Full human-readable expansions are
> provided only for novel or complex logic.
>
> **CochranBlock implementation:** Static site uses `t0 {}` (empty state), PORT default 8081, sled (not SQLite) for admin/db when `admin` feature enabled. No Askama — inline HTML. See `docs/COCHRANBLOCK_PLAN.md` for phased roadmap.

---

# A. ARCHITECTURE OVERVIEW

## A.1 System Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                    SINGLE RUST BINARY                        │
│                                                              │
│  ┌──────────────────────────────────────────────────────┐   │
│  │                   AXUM WEB SERVER                     │   │
│  │                                                       │   │
│  │  ┌─────────┐ ┌──────────┐ ┌──────────┐ ┌─────────┐  │   │
│  │  │ Page 1  │ │ Page 2   │ │ Page 3   │ │ Page 4  │  │   │
│  │  │ Resume  │ │ Whiteppr │ │ Innovatn │ │ Admin   │  │   │
│  │  │ f3@0    │ │ f4@0     │ │ f5@0     │ │ f6@2    │  │   │
│  │  └────┬────┘ └────┬─────┘ └────┬─────┘ └────┬────┘  │   │
│  │       │           │            │             │        │   │
│  │  ┌────┴───────────┴────────────┴─────────────┴────┐  │   │
│  │  │              MIDDLEWARE STACK                    │  │   │
│  │  │  f24 (require_auth) ──► f25 (rate_limiter)     │  │   │
│  │  │  tower_http::compression ──► tracing            │  │   │
│  │  └────────────────────┬───────────────────────────┘  │   │
│  │                       │                               │   │
│  │  ┌────────────────────┴───────────────────────────┐  │   │
│  │  │              SHARED STATE (t0)                  │  │   │
│  │  │  s0: SqlitePool    s1: CloudflareClient        │  │   │
│  │  │  s2: DnsConfig     s3: SessionStore            │  │   │
│  │  │  s4: EncryptionKey s5: Arc<Mutex<LastIp>>      │  │   │
│  │  └────────────────────────────────────────────────┘  │   │
│  └──────────────────────────────────────────────────────┘   │
│                                                              │
│  ┌──────────────────────┐  ┌─────────────────────────────┐  │
│  │  DNS MONITOR (f12)   │  │  EMBEDDED ASSETS (m8)       │  │
│  │  Background tokio    │  │  include_packed (zstd)                 │  │
│  │  task via Ω5         │  │  Compiled into binary       │  │
│  │  Runs Σ2 on c1      │  │  Served via f23             │  │
│  │  interval            │  │                             │  │
│  └──────────┬───────────┘  └─────────────────────────────┘  │
│             │                                                │
│  ┌──────────┴───────────────────────────────────────────┐   │
│  │                    SQLite (c4)                         │   │
│  │  Tables: admin, sessions, settings, dns_log           │   │
│  │  Via sqlx with compile-time checked queries           │   │
│  └───────────────────────────────────────────────────────┘   │
│                                                              │
└──────────────────────────┬──────────────────────────────────┘
                           │
              ┌────────────┴────────────┐
              │    EXTERNAL SERVICES     │
              │                          │
              │  api.cloudflare.com/v4   │
              │  api.ipify.org           │
              │  api64.ipify.org (v6)    │
              └─────────────────────────┘
```

## A.2 Component Interaction Flow

The binary starts a single Tokio runtime that hosts two concurrent subsystems:

**Web Server Path:**
`Browser → Axum Router (f1) → Middleware Stack (f24/f25) → Handler (f2-f9) → Askama Template (r2) → HTML Response`

**DNS Monitor Path:**
`Startup → tokio::spawn (Ω5) → f12 loop → f13 (Ω6 ipify) → compare → f11 (Cloudflare API) → sleep(c1)`

**Admin Settings Path:**
`Admin Login (Φ1) → Session Cookie (f18) → Settings Page (Φ3) → Save Token (Φ2) → Encrypted Storage (Σ4)`

**First-Time Setup Path:**
`No admin exists → f28 (setup page) → Create admin credentials (Σ1) → Redirect to login`

## A.3 Deployment Model

Single binary deployment. The binary contains:
- All HTML templates (compiled via Askama at build time)
- All CSS/JS assets (embedded via `include_packed` zstd at build time)
- SQLite database created at runtime in a configurable data directory
- No external runtime dependencies except libc

```
Production server:
  /opt/portfolio/
  ├── portfolio-server          # single binary (~10-15MB)
  └── data/
      └── portfolio.db          # created at first run
```

---

# B. TECHNOLOGY RECOMMENDATIONS

## B.1 Web Framework: Axum (recommended)

| Criterion | Axum | Actix-web | Rocket |
|-----------|------|-----------|--------|
| Tokio-native | ✅ First-class | ⚠️ Own runtime (actix-rt) | ✅ Tokio 1.x |
| Tower middleware | ✅ Native | ❌ Own middleware | ❌ Fairings |
| Type safety | ✅ Compile-time extractors | ✅ Good | ✅ Good |
| Community momentum (2026) | ✅ Fastest growing | ✅ Mature, stable | ⚠️ Slower releases |
| Background tasks | ✅ tokio::spawn directly | ⚠️ Needs Arbiter | ✅ rocket::tokio |
| Learning curve | ✅ Moderate | ⚠️ Actor model complexity | ✅ Macro-heavy but clear |
| Single binary friendly | ✅ Excellent | ✅ Good | ✅ Good |

**Justification:** Axum is the strongest choice for your project because it is built directly on Tokio and Tower, meaning your background DNS task (f12) and web server share the same runtime with zero friction. The Tower middleware ecosystem gives you composable auth (f24), rate limiting (f25), compression, and CORS out of the box. Axum's extractor pattern maps perfectly to your handler signatures (@0-@2). Its momentum in the Rust ecosystem means better long-term crate compatibility.

**Trade-off acknowledged:** Actix-web has marginally higher raw throughput in benchmarks, but for a personal portfolio site, this is irrelevant. Code clarity and maintainability — your stated priority — favor Axum.

## B.2 Complete Crate Recommendations

### Core Stack
| Purpose | Crate | Version | Justification |
|---------|-------|---------|---------------|
| Web framework | `axum` | 0.8+ | Tokio-native, Tower middleware, best DX |
| Async runtime | `tokio` | 1.x (full features) | Required by Axum, powers f12 background task |
| HTTP client | `reqwest` | 0.12+ | For Cloudflare API (f11) and IP lookup (f13) |
| Templating | `askama` | 0.12+ | Compile-time templates, type-safe, zero-cost |
| Embedded assets | `include_packed` | 0.1 | Embed CSS/JS/images via zstd at compile time |
| Database | `sqlx` | 0.8+ (sqlite) | Async, compile-time query checking, migrations |
| Serialization | `serde` + `serde_json` | 1.x | JSON request/response bodies |

### Security Stack
| Purpose | Crate | Version | Justification |
|---------|-------|---------|---------------|
| Password hashing | `argon2` | 0.5+ | OWASP recommended, winner of PHC. Ω9/Ω10 |
| Encryption (token) | `aes-gcm` | 0.10+ | AES-256-GCM authenticated encryption. Ω7/Ω8 |
| Key derivation | `hkdf` + `sha2` | 0.12+ | Derive encryption key from master secret |
| Random generation | `rand` | 0.8+ | Session IDs, nonces, salt |
| UUID | `uuid` | 1.x (v4) | Session identifiers |
| CSRF protection | `axum-csrf` | 0.9+ | Form protection on admin page |

### Infrastructure Stack
| Purpose | Crate | Version | Justification |
|---------|-------|---------|---------------|
| Logging | `tracing` + `tracing-subscriber` | 0.1+ | Structured logging, async-aware |
| Tower middleware | `tower-http` | 0.6+ | Compression, CORS, trace, timeout |
| Config | `dotenvy` | 0.15+ | .env file support for local dev |
| CLI args | `clap` | 4.x | Binary flags: --port, --data-dir, --setup |
| Graceful shutdown | `tokio::signal` | (in tokio) | Ctrl+C and SIGTERM handling |

### Alternatives Considered

**Tera vs Askama:** Tera is runtime-evaluated (flexibility for hot-reload) but Askama compiles templates into the binary at build time — perfect for your single-binary requirement. Askama catches template errors at compile time and has zero runtime overhead. Since your 4 pages are fixed, you don't need hot-reload. **Askama wins.**

**SQLite vs file-based storage:** A flat JSON file would work for 3-4 values but breaks under concurrent access (web server + DNS monitor both read settings). SQLite handles concurrent reads natively, supports migrations for future schema changes, and sqlx provides compile-time query verification. The overhead is ~600KB added to binary. **SQLite wins.**

**JWT vs session-based auth:** For a single-user admin panel, JWT adds unnecessary complexity (token refresh, revocation lists, key rotation). A simple server-side session stored in SQLite with a secure cookie is simpler, more secure (revocable), and appropriate for your use case. **Session-based wins.**

---

# C. PROJECT STRUCTURE

## C.1 Directory Layout

```
portfolio-server/
├── Cargo.toml
├── .env.example                    # Template for local dev config
├── build.rs                        # (optional) pre-build asset steps
├── sqlx-data.json                  # Offline query verification cache
│
├── src/
│   ├── main.rs                     # f0: startup, router assembly, Ω5 spawn
│   │
│   ├── config.rs                   # m5: t1 (Config), env parsing, clap args
│   ├── error.rs                    # m7: t14 (ErrorResponse), e0-e9, IntoResponse impl
│   ├── state.rs                    # t0 (AppState) definition, constructor
│   │
│   ├── web/                        # m0
│   │   ├── mod.rs                  # f1 (app_router): assembles all routes
│   │   ├── pages.rs                # f2-f5: Φ0 handlers (public pages)
│   │   ├── admin.rs                # f6-f9: Φ1,Φ2,Φ3 handlers (auth'd)
│   │   ├── setup.rs                # f28: first-time setup flow
│   │   └── static_assets.rs        # f23: include_packed serving
│   │
│   ├── auth/                       # m2
│   │   ├── mod.rs
│   │   ├── middleware.rs            # f24 (require_auth): session validation extractor
│   │   ├── session.rs              # f18, f19: create/validate sessions in SQLite
│   │   └── password.rs             # f16, f17: Ω9, Ω10 (argon2)
│   │
│   ├── dns/                        # m1
│   │   ├── mod.rs
│   │   ├── monitor.rs              # f12 (Φ4): background loop, f13 (IP check)
│   │   └── cloudflare.rs           # t5, f10, f11: API client, record update
│   │
│   ├── crypto/                     # m3
│   │   ├── mod.rs
│   │   └── token.rs                # f14, f15: Ω7, Ω8 (AES-GCM encrypt/decrypt)
│   │
│   ├── db/                         # m4
│   │   ├── mod.rs                  # f20 (init_db), f21 (run_migrations)
│   │   └── queries.rs              # Typed query helpers using @4, @5
│   │
│   └── templates/                  # m6
│       ├── mod.rs                  # Askama template structs (t15-t17)
│       ├── base.html               # Shared layout: nav, footer, CSS links
│       ├── resume.html             # Page 1
│       ├── whitepaper.html         # Page 2
│       ├── innovation.html         # Page 3
│       ├── admin.html              # Page 4 (settings dashboard)
│       ├── login.html              # Login form
│       └── setup.html              # First-time setup form
│
├── assets/                         # Embedded via include_packed (zstd)
│   ├── css/
│   │   ├── main.css                # Design system
│   │   ├── resume.css              # Page-specific styles
│   │   └── prism.css               # Code syntax highlighting
│   ├── js/
│   │   ├── main.js                 # Minimal interactivity
│   │   └── admin.js                # Admin form handling
│   ├── fonts/                      # Self-hosted web fonts (optional)
│   └── images/
│       └── favicon.ico
│
├── migrations/                     # sqlx migrations
│   ├── 001_initial.sql
│   └── 002_dns_log.sql
│
└── data/                           # Runtime data (gitignored)
    └── portfolio.db
```

## C.2 Module Dependency Graph

```
main.rs
  ├── m5 (config)          ← standalone, no internal deps
  ├── m7 (error)           ← standalone
  ├── m4 (db)              ← depends on m5
  ├── m3 (crypto)          ← standalone
  ├── m2 (auth)            ← depends on m3, m4
  ├── m1 (dns)             ← depends on m3, m4, m5
  ├── m0 (web)             ← depends on m2, m6, m8
  ├── m6 (templates)       ← standalone (Askama structs)
  └── m8 (static_assets)   ← standalone (include_packed)
```

Clean dependency flow: no circular dependencies. Each module can be tested independently.

## C.3 Configuration Management

Three-tier configuration with precedence:

```
1. CLI args (highest)     →  --port 8443 --data-dir /var/lib/portfolio
2. Environment variables  →  PORTFOLIO_PORT=8443
3. .env file (lowest)     →  PORTFOLIO_PORT=8443
```

Config struct (t1):
```
t1 {
  s_port: u16              // default 8080
  s_data_dir: PathBuf      // default "./data"
  s_log_level: String      // default "info"
  s_master_key: String     // REQUIRED: 32-byte hex for encryption
  s_bind_addr: String      // default "0.0.0.0"
}
```

The `s_master_key` (PORTFOLIO_MASTER_KEY env var) is the only secret that lives outside the binary/database. Everything else (Cloudflare token, admin password hash) is encrypted at rest in SQLite using a key derived from this master key via HKDF.

---

# D. IMPLEMENTATION GUIDANCE

> All code below uses compression notation. Reference `kova/docs/compression_map.md` for expansion.
> Full human-readable code is provided only for novel/complex sections.

## D.1 Application Entry Point and Router Assembly

### main.rs — f0 (full, novel startup logic)

```rust
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // m5: parse config from CLI + env + .env
    let config = Config::load()?;

    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(&config.log_level)
        .init();

    // m4: init SQLite, run migrations
    let pool = init_db(&config.data_dir).await?;
    run_migrations(&pool).await?;

    // m3: derive encryption key from master key via HKDF
    let encryption_key = derive_key(&config.master_key)?;

    // Build shared state (t0)
    let state = Arc::new(AppState {
        db_pool: pool,
        cf_client: reqwest::Client::new(),
        dns_config: DnsConfig::load_from_db(&pool).await.ok(),
        session_store: Default::default(),
        encryption_key,
        last_ip: Arc::new(Mutex::new(String::new())),
    });

    // Ω5: spawn DNS monitor as background task
    if state.dns_config.is_some() {
        let dns_state = Arc::clone(&state);
        tokio::spawn(async move {
            dns_monitor_loop(dns_state).await;
        });
    }

    // m0: build router
    let app = app_router(Arc::clone(&state));

    // Bind and serve with graceful shutdown
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(graceful_shutdown())
        .await?;

    Ok(())
}
```

### Router Assembly — f1 (compact + human hybrid)

```rust
// f1: assembles all routes with middleware layers
pub fn app_router(state: Arc<AppState>) -> Router {
    let public = Router::new()
        .route("/",          get(f2))    // serve_index
        .route("/resume",    get(f3))    // Φ0(t16)
        .route("/whitepaper",get(f4))    // Φ0
        .route("/innovation",get(f5))    // Φ0
        .route("/login",     get(login_page).post(f7))  // Φ1
        .route("/setup",     get(setup_page).post(f28)) // first-time
        .route("/logout",    post(f9));

    let admin = Router::new()
        .route("/admin",          get(f6))   // Φ3
        .route("/admin/settings", post(f8))  // Φ2
        .route("/admin/settings", get(f26))  // Φ3
        .layer(axum::middleware::from_fn_with_state(
            state.clone(), require_auth  // f24
        ));

    let assets = Router::new()
        .route("/assets/*path", get(f23));  // include_packed

    Router::new()
        .merge(public)
        .merge(admin)
        .merge(assets)
        .layer(tower_http::compression::CompressionLayer::new())
        .layer(tower_http::trace::TraceLayer::new_for_http())
        .with_state((*state).clone())
}
```

## D.2 Admin Authentication Middleware

### f24 — require_auth (full, security-critical)

```rust
// Implemented as an Axum extractor for ergonomic use in handlers
pub struct AuthenticatedAdmin {
    pub username: String,
    pub session_id: Uuid,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for AuthenticatedAdmin
where
    S: Send + Sync,
    AppState: FromRef<S>,
{
    type Rejection = Redirect;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let app_state = AppState::from_ref(state);

        // Extract session cookie
        let cookies = parts.headers
            .get(header::COOKIE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        let session_id = parse_cookie(cookies, "session_id")
            .and_then(|s| Uuid::parse_str(&s).ok())
            .ok_or(Redirect::to("/login"))?;

        // Σ0: db fetch or reject
        let session = sqlx::query_as!(
            Session,
            "SELECT id, username, expires_at FROM sessions
             WHERE id = ? AND expires_at > datetime('now')",
            session_id.to_string()
        )
        .fetch_optional(&app_state.db_pool)
        .await
        .map_err(|_| Redirect::to("/login"))?
        .ok_or(Redirect::to("/login"))?;

        Ok(AuthenticatedAdmin {
            username: session.username,
            session_id,
        })
    }
}

// Usage in handlers — just add the extractor:
// f6@2: async fn serve_admin(State(p0):State<t0>, admin: AuthenticatedAdmin) -> impl r0
```

## D.3 Cloudflare API Integration

### t5 — CloudflareClient and f11 — update_dns (full, external API)

```rust
// DNS record update via Cloudflare API v4
pub async fn update_dns(
    client: &reqwest::Client,
    api_token: &str,
    zone_id: &str,
    record_id: &str,
    new_ip: &str,
    record_type: &str, // "A" or "AAAA"
) -> Result<(), AppError> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/zones/{zone_id}/dns_records/{record_id}"
    );

    let body = serde_json::json!({
        "type": record_type,
        "content": new_ip,
        "ttl": 300,
        "proxied": false
    });

    let resp = client
        .patch(&url)
        .header("Authorization", format!("Bearer {api_token}"))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| AppError::CloudflareApi(e.to_string()))?;

    if !resp.status().is_success() {
        let err_body = resp.text().await.unwrap_or_default();
        tracing::error!("Cloudflare API error: {err_body}");
        return Err(AppError::CloudflareApi(err_body));
    }

    tracing::info!("DNS {record_type} record updated to {new_ip}");
    Ok(())
}

// f13: get external IP (supports both v4 and v6)
pub async fn get_external_ip(client: &reqwest::Client) -> Result<(String, String), AppError> {
    let ipv4 = client.get("https://api.ipify.org")
        .timeout(Duration::from_secs(10))
        .send().await
        .and_then(|r| Ok(r.text()))
        // ... flatten async
        .unwrap_or_default();

    let ipv6 = client.get("https://api64.ipify.org")
        .timeout(Duration::from_secs(10))
        .send().await
        .and_then(|r| Ok(r.text()))
        .unwrap_or_default();

    Ok((ipv4, ipv6))
}
```

## D.4 Background DNS Monitor

### f12 — dns_monitor_loop (Φ4 expanded)

```rust
pub async fn dns_monitor_loop(state: Arc<AppState>) {
    let interval = state.dns_config.as_ref()
        .map(|c| c.check_interval)
        .unwrap_or(Duration::from_secs(300)); // c1 default

    tracing::info!("DNS monitor started, checking every {}s", interval.as_secs());

    loop {
        // Σ2: check IP → compare → update if changed
        match check_and_update(&state).await {
            Ok(updated) => {
                if updated {
                    tracing::info!("DNS records updated successfully");
                }
            }
            Err(e) => {
                tracing::error!("DNS check failed: {e}");
                // Log to database for admin visibility
                let _ = log_dns_event(&state.db_pool, &format!("Error: {e}")).await;
            }
        }

        tokio::time::sleep(interval).await;
    }
}

async fn check_and_update(state: &AppState) -> Result<bool, AppError> {
    // Σ5: fetch encrypted token from DB, decrypt
    let api_token = get_decrypted_token(&state.db_pool, &state.encryption_key).await?;

    let dns_config = state.dns_config.as_ref()
        .ok_or(AppError::Config("DNS not configured".into()))?;

    // f13: get current external IP
    let (current_v4, current_v6) = get_external_ip(&state.cf_client).await?;

    let mut stored = state.last_ip.lock().await;
    let changed = *stored != current_v4;

    if changed && !current_v4.is_empty() {
        // f11: update A record
        update_dns(
            &state.cf_client, &api_token,
            &dns_config.zone_id, &dns_config.a_record_id,
            &current_v4, "A"
        ).await?;

        // f11: update AAAA record if v6 available
        if !current_v6.is_empty() {
            if let Some(aaaa_id) = &dns_config.aaaa_record_id {
                update_dns(
                    &state.cf_client, &api_token,
                    &dns_config.zone_id, aaaa_id,
                    &current_v6, "AAAA"
                ).await?;
            }
        }

        *stored = current_v4;
        log_dns_event(&state.db_pool, &format!("Updated to {}", *stored)).await?;
        Ok(true)
    } else {
        Ok(false)
    }
}
```

## D.5 Secure Token Storage

### f14/f15 — encrypt/decrypt (Ω7/Ω8 expanded, security-critical)

```rust
use aes_gcm::{Aes256Gcm, KeyInit, Nonce};
use aes_gcm::aead::Aead;
use rand::RngCore;

const NONCE_LEN: usize = 12;

/// Ω7: Encrypt API token before storing in SQLite
pub fn encrypt_token(plaintext: &str, key: &[u8; 32]) -> Result<Vec<u8>, AppError> {
    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| AppError::Encryption("Invalid key".into()))?;

    let mut nonce_bytes = [0u8; NONCE_LEN];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher.encrypt(nonce, plaintext.as_bytes())
        .map_err(|_| AppError::Encryption("Encrypt failed".into()))?;

    // Prepend nonce to ciphertext for storage
    let mut result = Vec::with_capacity(NONCE_LEN + ciphertext.len());
    result.extend_from_slice(&nonce_bytes);
    result.extend_from_slice(&ciphertext);
    Ok(result)
}

/// Ω8: Decrypt API token retrieved from SQLite
pub fn decrypt_token(encrypted: &[u8], key: &[u8; 32]) -> Result<String, AppError> {
    if encrypted.len() < NONCE_LEN {
        return Err(AppError::Encryption("Data too short".into()));
    }

    let cipher = Aes256Gcm::new_from_slice(key)
        .map_err(|_| AppError::Encryption("Invalid key".into()))?;

    let nonce = Nonce::from_slice(&encrypted[..NONCE_LEN]);
    let ciphertext = &encrypted[NONCE_LEN..];

    let plaintext = cipher.decrypt(nonce, ciphertext)
        .map_err(|_| AppError::Encryption("Decrypt failed".into()))?;

    String::from_utf8(plaintext)
        .map_err(|_| AppError::Encryption("Invalid UTF-8".into()))
}

/// Derive encryption key from master secret using HKDF
pub fn derive_key(master_key: &str) -> Result<[u8; 32], AppError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hk = Hkdf::<Sha256>::new(Some(b"portfolio-token-encryption"), master_key.as_bytes());
    let mut key = [0u8; 32];
    hk.expand(b"aes-256-gcm-key", &mut key)
        .map_err(|_| AppError::Encryption("Key derivation failed".into()))?;
    Ok(key)
}
```

## D.6 Settings Page with Token Storage

### Compact handler specifications:

```
f8@1(t12,()): Σ4
  // POST /admin/settings — save Cloudflare token
  // Requires: AuthenticatedAdmin extractor
  // Flow: extract JSON body → Ω7 (encrypt token) → @5 (store in DB)
  // Also stores: zone_id, record_id(s), check_interval
  // On success: restart DNS monitor with new config

f26@2: Σ5
  // GET /admin/settings — read current settings
  // Flow: @4 (fetch from DB) → Ω8 (decrypt token, mask for display) → render template
  // Token displayed as: cf_****...****last4

f6@2: Φ3
  // GET /admin — dashboard showing:
  //   - Current external IP (from s5)
  //   - Last DNS update time (from dns_log table)
  //   - DNS monitor status (running/stopped)
  //   - Link to settings
```

### Admin settings handler (full, form processing):

```rust
pub async fn admin_save_settings(
    State(state): State<AppState>,
    _admin: AuthenticatedAdmin,  // f24 extractor enforces auth
    Json(payload): Json<SettingsSaveRequest>,
) -> Result<Json<ApiResponse>, AppError> {
    // Validate Cloudflare token by making a test API call
    let test = state.cf_client
        .get("https://api.cloudflare.com/client/v4/user/tokens/verify")
        .header("Authorization", format!("Bearer {}", payload.api_token))
        .send().await?;

    if !test.status().is_success() {
        return Err(AppError::InvalidCredentials(
            "Cloudflare token verification failed".into()
        ));
    }

    // Σ4: encrypt then store
    let encrypted = encrypt_token(&payload.api_token, &state.encryption_key)?;

    sqlx::query!(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('cf_token', ?)",
        encrypted
    ).execute(&state.db_pool).await?;

    // Store zone_id and record_id(s) (not encrypted — not secrets)
    sqlx::query!(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('zone_id', ?)",
        payload.zone_id
    ).execute(&state.db_pool).await?;

    sqlx::query!(
        "INSERT OR REPLACE INTO settings (key, value) VALUES ('a_record_id', ?)",
        payload.a_record_id
    ).execute(&state.db_pool).await?;

    // Signal DNS monitor to reload config
    // (In production: use a tokio::sync::watch channel)
    tracing::info!("Settings updated, DNS monitor will pick up changes on next cycle");

    Ok(Json(ApiResponse { success: true, message: "Settings saved".into() }))
}
```

## D.7 First-Time Setup Flow

### f28 — Initial admin creation (full, one-time flow):

```rust
pub async fn first_time_setup(
    State(state): State<AppState>,
    Json(payload): Json<SetupRequest>,
) -> Result<Json<SetupResponse>, AppError> {
    // Guard: only works if no admin exists yet
    let existing = sqlx::query!("SELECT COUNT(*) as count FROM admin")
        .fetch_one(&state.db_pool).await?;

    if existing.count > 0 {
        return Err(AppError::Config("Setup already completed".into()));
    }

    // Validate password strength (minimum 12 chars)
    if payload.password.len() < 12 {
        return Err(AppError::InvalidCredentials(
            "Password must be at least 12 characters".into()
        ));
    }

    // Σ1: hash password, store admin
    let password_hash = argon2::hash_encoded(
        payload.password.as_bytes(),
        &rand::random::<[u8; 16]>(),
        &argon2::Config::default(),
    ).map_err(|_| AppError::Encryption("Hash failed".into()))?;

    sqlx::query!(
        "INSERT INTO admin (username, password_hash) VALUES (?, ?)",
        payload.username,
        password_hash
    ).execute(&state.db_pool).await?;

    tracing::info!("Admin account created for: {}", payload.username);

    Ok(Json(SetupResponse {
        success: true,
        message: "Admin account created. Please log in.".into(),
        redirect: "/login".into(),
    }))
}
```

## D.8 Database Schema

### migrations/001_initial.sql

```sql
-- Admin credentials
CREATE TABLE IF NOT EXISTS admin (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Sessions
CREATE TABLE IF NOT EXISTS sessions (
    id TEXT PRIMARY KEY,           -- UUID v4
    username TEXT NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    expires_at DATETIME NOT NULL,
    FOREIGN KEY (username) REFERENCES admin(username)
);

-- Key-value settings (encrypted values stored as BLOB)
CREATE TABLE IF NOT EXISTS settings (
    key TEXT PRIMARY KEY,
    value BLOB NOT NULL,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- DNS update log
CREATE TABLE IF NOT EXISTS dns_log (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event TEXT NOT NULL,
    ip_address TEXT,
    timestamp DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX IF NOT EXISTS idx_sessions_expires ON sessions(expires_at);
CREATE INDEX IF NOT EXISTS idx_dns_log_timestamp ON dns_log(timestamp);
```

## D.9 Cargo.toml

```toml
[package]
name = "portfolio-server"
version = "0.1.0"
edition = "2021"

[dependencies]
# Web
axum = { version = "0.8", features = ["macros"] }
tokio = { version = "1", features = ["full"] }
tower-http = { version = "0.6", features = ["compression-full", "trace", "cors", "fs"] }
tower = "0.5"

# Templating & Assets
askama = "0.12"
askama_axum = "0.4"
include_packed = "0.1"

# HTTP Client
reqwest = { version = "0.12", features = ["json", "rustls-tls"], default-features = false }

# Database
sqlx = { version = "0.8", features = ["runtime-tokio", "sqlite", "migrate"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Security
argon2 = "0.5"
aes-gcm = "0.10"
hkdf = "0.12"
sha2 = "0.10"
rand = "0.8"
uuid = { version = "1", features = ["v4", "serde"] }

# Infrastructure
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
clap = { version = "4", features = ["derive", "env"] }
dotenvy = "0.15"
anyhow = "1"
thiserror = "2"

# Optimization
mimalloc = { version = "0.1", default-features = false }

[profile.release]
opt-level = "z"     # Optimize for size
lto = true          # Link-time optimization
codegen-units = 1   # Single codegen unit for better optimization
strip = true        # Strip debug symbols
panic = "abort"     # Smaller binary, no unwinding
```

---

# E. FRONTEND APPROACH

## E.1 Recommendation: Askama Templates + Vanilla CSS + Minimal JS

**Why not an SPA (React/Vue/Svelte)?**
- Your site has 4 pages with mostly static content
- An SPA adds 100KB+ of JavaScript framework overhead
- Requires a build pipeline (Node.js, bundler) — violates your "100% Rust" constraint
- SEO is worse for SPAs without SSR
- Askama compiles templates into the binary — zero runtime cost, zero external dependencies

**Why not HTMX?**
- HTMX is excellent for dynamic interactions, but your public pages are read-only
- The only interactive page (admin) has simple forms that work fine with standard POST
- Adding HTMX would be reasonable for future enhancements (live DNS status updates)

**Recommended approach:**
- Askama templates with a shared base layout
- Vanilla CSS with CSS custom properties (variables) for theming
- Minimal vanilla JS only for: animated resume elements, code syntax highlighting, admin form validation
- All assets embedded via `include_packed` (zstd)

## E.2 Design System

### Visual Theme: "Terminal Elegance"

Given your background in cyber operations and Rust development, a design that bridges military precision with modern tech aesthetics:

```
Color Palette:
  --bg-primary:    #0a0e17     (deep navy, almost black)
  --bg-secondary:  #111827     (dark slate)
  --bg-card:       #1a2332     (elevated surface)
  --text-primary:  #e2e8f0     (light gray)
  --text-secondary:#94a3b8     (muted gray)
  --accent-primary:#3b82f6     (electric blue — links, highlights)
  --accent-green:  #10b981     (success, active states)
  --accent-amber:  #f59e0b     (warnings, emphasis)
  --border:        #1e293b     (subtle borders)
  --code-bg:       #0d1117     (code blocks)

Typography:
  --font-heading:  'Inter', system-ui, sans-serif
  --font-body:     'Inter', system-ui, sans-serif
  --font-mono:     'JetBrains Mono', 'Fira Code', monospace

Layout:
  - Max content width: 900px (readable line length)
  - Responsive: single column on mobile, comfortable margins on desktop
  - Navigation: fixed top bar with page links
  - Subtle animations: fade-in on scroll, typing effect for headings
```

### Page-Specific Design Notes:

**Page 1 (Resume):** Timeline-style layout for experience. Skills displayed as tagged chips with proficiency indicators. Animated section reveals on scroll. Print-friendly alternate stylesheet.

**Page 2 (Whitepaper):** Academic paper layout with table of contents sidebar (sticky on desktop). Code examples with syntax highlighting (Prism.js, ~5KB). Compression ratio tables styled as data visualizations. LaTeX-style typography for formulas.

**Page 3 (Innovation):** Split-panel layout: narrative on left, technical diagrams on right. Interactive before/after comparison for CI/CD binary sizes. Terminal-style animation showing the build process. Architecture diagrams using CSS (no image dependencies).

**Page 4 (Admin):** Clean dashboard layout. Status cards for: current IP, last update, monitor status. Settings form with real-time token validation. Activity log table with recent DNS updates.

### Base Template Structure (Askama):

```html
<!-- templates/base.html -->
<!DOCTYPE html>
<html lang="en" data-theme="dark">
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1">
    <title>{% block title %}Michael Cochran{% endblock %}</title>
    <link rel="stylesheet" href="/assets/css/main.css">
    {% block head %}{% endblock %}
</head>
<body>
    <nav class="nav">
        <a href="/" class="nav-brand">MC</a>
        <div class="nav-links">
            <a href="/resume" {% if active_page == "resume" %}class="active"{% endif %}>Resume</a>
            <a href="/whitepaper" {% if active_page == "whitepaper" %}class="active"{% endif %}>Research</a>
            <a href="/innovation" {% if active_page == "innovation" %}class="active"{% endif %}>Innovation</a>
        </div>
    </nav>

    <main class="content">
        {% block content %}{% endblock %}
    </main>

    <footer class="footer">
        <p>&copy; 2026 Michael Cochran</p>
    </footer>

    {% block scripts %}{% endblock %}
</body>
</html>
```

---

# F. SECURITY BEST PRACTICES

## F.1 API Token Encryption and Storage

**Architecture:**
```
User Input (plaintext token)
    │
    ▼
AES-256-GCM encrypt (Ω7)
    │  Key: derived from PORTFOLIO_MASTER_KEY via HKDF
    │  Nonce: random 12 bytes, prepended to ciphertext
    ▼
SQLite BLOB (settings table, key='cf_token')
    │
    ▼
AES-256-GCM decrypt (Ω8) — only when DNS monitor needs it
    │
    ▼
Plaintext token (held in memory only during API call, then dropped)
```

**Key management:**
- `PORTFOLIO_MASTER_KEY` is the only secret outside the database
- Set as environment variable or in a file readable only by the service user
- HKDF derives purpose-specific keys: one for token encryption, one for session signing
- If the master key is lost, the encrypted token is unrecoverable (re-enter via admin UI)

## F.2 Admin Authentication

**Password storage:** Argon2id with:
- Memory: 64MB (m_cost = 65536)
- Iterations: 3 (t_cost = 3)
- Parallelism: 4 (p_cost = 4)
- Salt: 16 random bytes per password

**Session management:**
- Session ID: UUID v4 (128 bits of randomness)
- Stored in SQLite with expiration timestamp
- Cookie: `session_id={uuid}; HttpOnly; Secure; SameSite=Strict; Path=/admin; Max-Age=86400`
- Session TTL: 24 hours (c0), configurable
- Logout: deletes session from database immediately
- Expired session cleanup: background task or on-access pruning

**Brute force protection:**
- Track failed login attempts per IP in memory (HashMap with TTL)
- After c2 (5) failed attempts within c3 (60s): return 429 Too Many Requests
- Log all failed attempts with IP and timestamp

## F.3 Rate Limiting for DNS Updates

```rust
// f25: rate limiter middleware (conceptual)
// Prevents DNS update API abuse and login brute force
// Implementation: tower::limit::RateLimitLayer or custom

// DNS monitor self-limits via sleep interval (c1 = 300s)
// Additional guard: skip update if last update was < 60s ago
// Cloudflare API rate limit: 1200 requests/5min — our usage is ~12/hour max
```

## F.4 Error Handling Without Information Leakage

```rust
// m7: AppError → HTTP response mapping
// NEVER expose internal details to clients

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Log full error internally
        tracing::error!("AppError: {self:?}");

        // Return sanitized response to client
        let (status, message) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized"),
            AppError::InvalidCredentials(_) => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AppError::RateLimited => (StatusCode::TOO_MANY_REQUESTS, "Too many requests"),
            AppError::Config(_) => (StatusCode::INTERNAL_SERVER_ERROR, "Configuration error"),
            // All other errors: generic 500
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal server error"),
        };

        (status, Json(ErrorResponse { error: message.into() })).into_response()
    }
}
```

## F.5 Additional Security Measures

- **HTTPS:** Cloudflare handles TLS at the edge. Origin can run HTTP (Flexible SSL) or HTTPS with Cloudflare Origin Certificate (Full/Full Strict). Native TLS via `--gen-self-signed` or `--acme` for local/dev.
- **CSRF:** Admin forms include a CSRF token (via `axum-csrf` or a custom double-submit cookie pattern).
- **Content Security Policy:** Set strict CSP headers via Tower middleware — no inline scripts, no external resources.
- **X-Frame-Options:** DENY (prevent clickjacking).
- **Dependency auditing:** Run `cargo audit` in CI to catch known vulnerabilities.

---

# G. DEPLOYMENT CONSIDERATIONS

## G.1 Binary Compilation and Optimization

```bash
# Build optimized release binary
cargo build --release

# Result: target/release/portfolio-server (~10-15MB)
# With profile.release settings from Cargo.toml:
#   opt-level="z", lto=true, codegen-units=1, strip=true, panic="abort"

# For even smaller binary (optional):
# Install: cargo install cargo-bloat
# Analyze: cargo bloat --release --crates
# Consider: upx --best target/release/portfolio-server (further 50-70% size reduction)
```

**Cross-compilation for Linux target (if developing on macOS/Windows):**
```bash
# Using cross (Docker-based cross-compilation)
cargo install cross
cross build --release --target x86_64-unknown-linux-gnu
```

## G.2 Environment Variable Management

```bash
# .env.example — ship with the binary
PORTFOLIO_PORT=8080
PORTFOLIO_DATA_DIR=./data
PORTFOLIO_LOG_LEVEL=info
PORTFOLIO_MASTER_KEY=  # REQUIRED: generate with `openssl rand -hex 32`
PORTFOLIO_BIND_ADDR=0.0.0.0

# Production: use systemd EnvironmentFile or secrets manager
# NEVER commit .env with real MASTER_KEY to version control
```

## G.3 Systemd Service File

```ini
[Unit]
Description=Portfolio Web Server
After=network.target

[Service]
Type=simple
User=portfolio
Group=portfolio
WorkingDirectory=/opt/portfolio
ExecStart=/opt/portfolio/portfolio-server
EnvironmentFile=/opt/portfolio/.env
Restart=always
RestartSec=5

# Security hardening
NoNewPrivileges=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/opt/portfolio/data
PrivateTmp=true

[Install]
WantedBy=multi-user.target
```

## G.4 Logging and Monitoring

```rust
// Structured logging via tracing
// Output format: JSON in production, pretty in development

// Key log events:
// INFO:  server start, DNS update success, admin login
// WARN:  failed login attempt, DNS check failure (transient)
// ERROR: encryption failure, database error, Cloudflare API error

// Log rotation: handled by systemd journal or logrotate
// Monitoring: expose /health endpoint (unauthenticated) returning:
//   { "status": "ok", "uptime": "...", "last_dns_check": "...", "dns_monitor": "running" }
```

## G.5 Graceful Shutdown

```rust
// f22: handles SIGTERM and SIGINT
async fn graceful_shutdown() {
    let ctrl_c = async {
        tokio::signal::ctrl_c().await.expect("Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("SIGTERM handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => tracing::info!("Ctrl+C received"),
        _ = terminate => tracing::info!("SIGTERM received"),
    }

    tracing::info!("Shutting down gracefully...");
    // Axum's serve().with_graceful_shutdown() handles:
    // - Stop accepting new connections
    // - Wait for in-flight requests to complete
    // - DNS monitor loop exits naturally when runtime drops
}
```

---

# H. ANSWERS TO SPECIFIC QUESTIONS

## H.1 "What's the best way to structure the background DNS update task alongside the web server?"

Use `tokio::spawn` (Ω5) to launch the DNS monitor (f12) as a separate task on the same Tokio runtime. This is the idiomatic Rust approach and works perfectly with Axum because Axum is built on Tokio.

```
main() {
    let state = Arc::new(AppState { ... });

    // Spawn DNS monitor
    let dns_state = Arc::clone(&state);
    tokio::spawn(async move { dns_monitor_loop(dns_state).await });

    // Start web server (blocks until shutdown)
    axum::serve(listener, app).with_graceful_shutdown(shutdown).await;
}
```

**Why this works well:**
- Both tasks share `Arc<AppState>`, so the DNS monitor reads the same config and writes to the same `last_ip` that the admin dashboard displays.
- No inter-process communication needed — it's all in-process with `Arc<Mutex<T>>` for shared mutable state.
- Graceful shutdown: when the Tokio runtime drops, the spawned task is cancelled automatically.
- If you need to restart the DNS monitor (e.g., after settings change), use a `tokio::sync::watch` channel to signal the loop to reload config.

**Alternative considered:** Running the DNS monitor as a separate OS thread with `std::thread::spawn`. This would work but loses the benefits of async I/O (the monitor is mostly waiting on network calls) and complicates shared state.

## H.2 "Should I use a templating engine (like Tera, Askama) or serve a static SPA?"

**Askama (compile-time templates).** Definitive recommendation for your use case.

| Factor | Askama | Tera | SPA (Vue/React) |
|--------|--------|------|-----------------|
| Single binary | ✅ Compiled in | ⚠️ Runtime load | ❌ Needs Node build |
| Type safety | ✅ Compile-time | ❌ Runtime errors | N/A |
| Performance | ✅ Zero-cost | ⚠️ Runtime parsing | ⚠️ Client-side render |
| 100% Rust | ✅ Yes | ✅ Yes | ❌ JavaScript |
| SEO | ✅ Server-rendered | ✅ Server-rendered | ⚠️ Needs SSR |
| Complexity | ✅ Low | ✅ Low | ❌ High |
| Hot reload (dev) | ❌ Recompile | ✅ Yes | ✅ Yes |

The only downside of Askama is no hot-reload during development — you must recompile to see template changes. Mitigation: use `cargo watch -x run` for automatic recompilation on file changes (~2-3 second cycle for template-only changes).

## H.3 "What's the most secure yet practical way to handle admin authentication for a single-user application?"

**Server-side sessions with HttpOnly cookies.** This is the simplest secure approach for single-user.

The full flow:
1. User submits username + password to POST `/login`
2. Server verifies password against Argon2id hash in SQLite
3. Server creates a session row in SQLite with UUID v4 and 24h expiry
4. Server sets cookie: `session_id={uuid}; HttpOnly; Secure; SameSite=Strict`
5. All admin routes use the `AuthenticatedAdmin` extractor (f24) which validates the session
6. Logout deletes the session row and clears the cookie

**Why not JWT?**
- JWTs are designed for distributed systems where multiple services need to verify identity without a shared database. You have one server and one database.
- JWT revocation requires a blocklist — which is just a session store with extra steps.
- JWT secret rotation is complex. Session deletion is trivial.

**Why not HTTP Basic Auth?**
- Credentials sent with every request (even over HTTPS, this increases exposure surface).
- No session management — can't "log out" without closing the browser.
- Poor UX — browser's built-in auth dialog is ugly and inflexible.

## H.4 "How should I handle the initial setup flow (first-time admin password creation, API token input)?"

**Two-phase setup with a setup guard middleware:**

```
Phase 1: First Launch (no admin in DB)
  - ALL routes except /setup and /assets/* redirect to /setup
  - /setup page: create admin username + password
  - On submit: f28 creates admin, redirects to /login
  - Setup endpoint becomes permanently disabled (guard: admin count > 0)

Phase 2: Normal Operation (admin exists)
  - /setup returns 404 or redirects to /login
  - Admin logs in, navigates to /admin/settings
  - Enters Cloudflare API token, zone ID, record IDs
  - Token is encrypted (Ω7) and stored
  - DNS monitor starts on next cycle (or immediately via watch channel)
```

Implementation of the setup guard:

```rust
// Middleware: redirect to /setup if no admin exists
async fn setup_guard(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    // Cache this check — only query DB once, then store in AtomicBool
    if !state.is_setup_complete.load(Ordering::Relaxed) {
        let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM admin")
            .fetch_one(&state.db_pool).await.unwrap_or(0);

        if count == 0 && req.uri().path() != "/setup" && !req.uri().path().starts_with("/assets") {
            return Redirect::to("/setup").into_response();
        }

        if count > 0 {
            state.is_setup_complete.store(true, Ordering::Relaxed);
        }
    }
    next.run(req).await
}
```

---

# APPENDIX: QUICK-START IMPLEMENTATION ORDER

For iterative, testable development, implement in this order:

```
Week 1: Skeleton
  1. cargo init, Cargo.toml with all deps
  2. main.rs: Axum hello-world on port 8080
  3. include_packed: serve a single static HTML file
  4. Askama: base template + one page (resume)
  ✓ Testable: visit localhost:8080/resume, see rendered page

Week 2: All Pages + Database
  5. Add remaining 3 page templates (whitepaper, innovation, admin)
  6. SQLite setup with sqlx migrations
  7. First-time setup flow (f28)
  8. Admin login/logout (f7, f9, f18, f19)
  9. Auth middleware (f24)
  ✓ Testable: full page navigation, create admin, login, see admin page

Week 3: Security + Settings
  10. AES-GCM token encryption (f14, f15)
  11. Admin settings page — save/load Cloudflare token
  12. CSRF protection on forms
  13. Rate limiting on login
  ✓ Testable: save token, verify it's encrypted in DB, load it back

Week 4: DNS Monitor
  14. IP detection (f13)
  15. Cloudflare API integration (f11)
  16. Background monitor loop (f12)
  17. DNS log table and admin dashboard display
  ✓ Testable: change IP (use VPN), verify DNS record updates

Week 5: Polish
  18. CSS design system implementation
  19. Resume content formatting with animations
  20. Whitepaper content with syntax highlighting
  21. Innovation page with technical diagrams
  22. Graceful shutdown, logging, health endpoint
  23. Release build optimization, systemd service
  ✓ Testable: full production deployment
```

---

# APPENDIX: COMPRESSION MAP REFERENCE

All compact identifiers used in this document are defined in `kova/docs/compression_map.md`.
To expand any compact notation to full human-readable Rust code:

1. Replace `fN` with the function name from the Functions table
2. Replace `tN` with the type name from the Types table
3. Replace `pN`, `sN`, `eN`, `vN` with their respective names
4. Expand `@N` templates by substituting the function-specific parameters
5. Expand `ΩN` atomic operations inline
6. Expand `ΣN` sequences as the ordered chain of their component operations
7. Expand `ΦN` function templates as the full function body

The mapping is deterministic and lossless. `expand(compact(code)) ≡ code` for all structural and semantic content.