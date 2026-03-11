# Rogue Runner — Game Build & Delivery Plan

**Scope:** Build rogue-runner, deliver to logged-in users via downloads.  
**Date:** 2026-03-01

---

## 1. Current State

| Component | Status |
|-----------|--------|
| **WASM (web)** | `scripts/build-web.sh` builds → copies to `rogue-repo/assets/apps/rogue-runner-wasm/` → served at `/apps/rogue-runner-wasm` (no auth) |
| **Auth** | f97–f101: register, login, verify, logout. Session cookie `rr_session`, 7-day expiry |
| **Assets** | rust-embed `assets/` folder. PWA serves HTML, manifest, sw.js, app icons |
| **Native binary** | Not built or delivered yet |

---

## 2. Delivery Modes

| Mode | Audience | Auth | Storage |
|------|----------|------|---------|
| **Web/WASM** | Browser play | Optional (can require login) | Embedded in API binary |
| **Native download** | Desktop (Linux, Windows, macOS) | Required | `assets/downloads/` or object storage |

---

## 3. Build Pipeline

### 3.1 WASM (existing + CI)

- **Script:** `rogue-runner/scripts/build-web.sh`
- **Output:** `rogue-repo/assets/apps/rogue-runner-wasm/` (rogue-runner.wasm, index.html)
- **CI:** On release/tag, run script → commit assets or artifact → deploy

### 3.2 Native (implemented)

- **Windows:** `scripts/build-release.sh` or `cargo build -p rogue-runner --release --target x86_64-pc-windows-gnu`
- **Android:** `scripts/build-release.sh` — notfl3/cargo-apk Docker image may fail on Cargo.lock v4; fallback: run from rogue-runner/ with standalone build

**Targets:**

| Platform | Target | Binary name |
|----------|--------|-------------|
| Linux x64 | `x86_64-unknown-linux-gnu` | `rogue-runner-linux-x64` |
| Windows x64 | `x86_64-pc-windows-msvc` | `rogue-runner-windows-x64.exe` |
| macOS x64 | `x86_64-apple-darwin` | `rogue-runner-macos-x64` |
| macOS ARM | `aarch64-apple-darwin` | `rogue-runner-macos-aarch64` |

**Build commands (per platform):**

```bash
cargo build -p rogue-runner --release --target x86_64-unknown-linux-gnu
cargo build -p rogue-runner --release --target x86_64-pc-windows-msvc
cargo build -p rogue-runner --release --target x86_64-apple-darwin
cargo build -p rogue-runner --release --target aarch64-apple-darwin
```

**CI:** GitHub Actions matrix job (or similar) → output to `rogue-repo/assets/downloads/` or upload to artifact storage.

---

## 4. Storage

### Option A: Embedded (recommended for v1)

- **Path:** `rogue-repo/assets/downloads/`
- **Layout:**
  ```
  rogue-repo/assets/downloads/
  ├── rogue-runner-linux-x64
  ├── rogue-runner-windows-x64.exe
  ├── rogue-runner-macos-x64
  └── rogue-runner-macos-aarch64
  ```
- **Pros:** Single binary, no external deps, offline-capable
- **Cons:** API binary grows with each platform (~10–50 MB each)
- **Build:** CI copies `target/<triple>/release/rogue-runner` → `assets/downloads/<name>`

### Option B: Object storage (S3/MinIO)

- **Path:** Bucket `roguerepo-downloads/` or similar
- **Layout:** `rogue-runner/v0.1.0/rogue-runner-linux-x64`
- **Pros:** Smaller API binary, CDN-friendly, versioned
- **Cons:** Requires storage config, signed URLs or proxy
- **Build:** CI uploads binaries to bucket after build

---

## 5. Protected Download Route

### 5.1 New route

- **Path:** `GET /downloads/rogue-runner?platform=linux`
- **Auth:** Require valid session cookie (`rr_session`)
- **Platform values:** `linux`, `windows`, `macos`, `macos-arm`
- **Response:** Stream binary with `Content-Disposition: attachment; filename="rogue-runner-..."`

### 5.2 Implementation sketch

1. **Session extractor** (reuse auth logic): parse `rr_session` cookie → `Option<Uuid>` (user_id)
2. **Handler:** `serve_download(State, Query<Platform>, cookies)` → if no session, 401/redirect to `/login`
3. **Platform → file mapping:** `linux` → `downloads/rogue-runner-linux-x64`, etc.
4. **Serve:** `Assets::get("downloads/...")` (if embedded) or read from disk/S3

### 5.3 Optional: Entitlement check

- Before serving, query `entitlements` for `user_id` + `game_id = "rogue-runner"`
- If no entitlement: 403 "Purchase required" or redirect to buy (42 bucks via f15)
- If entitlement exists: serve binary

---

## 6. User Flow

```
1. User visits roguerepo.io
2. User logs in (f98) → session set
3. User navigates to "Downloads" or app page
4. User selects platform (Linux / Windows / macOS)
5. User clicks "Download" → GET /downloads/rogue-runner?platform=linux
6. API checks session → if valid, streams binary
7. Browser downloads rogue-runner-linux-x64
8. User runs binary locally
```

**Web play (no download):**

```
1. User logs in
2. User goes to /apps/rogue-runner-wasm
3. (Optional) Require login for WASM route too
4. Game loads in browser
```

---

## 7. Implementation Phases

### Phase 1: Native build + embedded storage

- [ ] Add `rogue-repo/assets/downloads/` (gitignore binaries, or commit for releases)
- [ ] Add build script or CI job for native targets
- [ ] Add `GET /downloads/rogue-runner?platform=<platform>` route
- [ ] Require session; 401 if not logged in
- [ ] Add "Downloads" section to PWA (links to download by platform)

### Phase 2: Downloads UI

- [ ] Add `/downloads` page (or section on home) with platform buttons
- [ ] Each button: `/downloads/rogue-runner?platform=linux` (etc.)
- [ ] Show "Login required" if not authenticated

### Phase 3: Entitlement gate (optional)

- [ ] Before serving, check `entitlements` for user + `rogue-runner`
- [ ] If missing: redirect to buy flow (f15 provision_entitlement)
- [ ] If present: serve binary

### Phase 4: Object storage (optional)

- [ ] Add S3/MinIO config
- [ ] CI uploads to bucket
- [ ] Route serves from bucket (redirect or proxy) instead of embedded

---

## 8. File Changes Summary

| File | Change |
|------|--------|
| `rogue-repo/src/main.rs` | Add route `/downloads/rogue-runner` |
| `rogue-repo/src/routes.rs` or new `downloads.rs` | Handler `serve_rogue_runner_download` |
| `rogue-repo/src/auth.rs` | Extract `require_session` or `session_user_id` for reuse |
| `rogue-repo/assets/` | Add `downloads/` subfolder (or second RustEmbed for downloads) |
| `rogue-repo/pwa.css` + HTML | Add Downloads section with platform links |
| `.github/workflows/build.yml` (or similar) | Matrix build native + WASM, copy to assets |

---

## 9. Notes

- **WASM auth:** Currently `/apps/rogue-runner-wasm` is public. To gate: add session check before serving HTML, or serve HTML that checks session via API before loading WASM.
- **Versioning:** Consider `?version=0.1.0` or path `/downloads/rogue-runner/0.1.0?platform=linux` for multiple versions.
- **Android:** rogue-runner has `package.metadata.android`; mobile builds are a separate delivery path (store or APK download).
