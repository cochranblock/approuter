# KOVA: COMPOSER 1.5 - MASTER ARCHITECTURE & IMPLEMENTATION PLAN

## CORE OBJECTIVE
Evolve Kova from a localized desktop tool into a ubiquitous, mobile-accessible, multi-model AI orchestration engine (Thin Client / Thick Server). Host runs on the primary gaming rig; access is securely routed to any mobile device via WebAssembly (WASM) and Cloudflare tunnels.

---

## PHASE 1: THE WASM / PWA THIN CLIENT (MOBILE `egui`)
Goal: Decouple the GUI from local host execution and compile it for mobile browsers.

1. App State Refactoring:
   - Strip `sled` (t12) and `std::process::Command` out of the `kova gui` binary.
   - Replace all direct disk I/O with asynchronous HTTP/WebSocket calls to `kova serve`.
2. Responsive UI (Mobile-First egui):
   - Implement `egui::Context::screen_rect()` checks.
   - If width < 600px: Collapse the `f25` Backlog into a hamburger menu. Maximize the chat interface and streaming code-block views.
3. WASM Compilation Target:
   - Command: `rustup target add wasm32-unknown-unknown`
   - Use `trunk` to build and bundle the egui WASM application: `trunk build --release`.
4. PWA Manifest:
   - Add a `manifest.json` and service worker to allow the Kova web app to be "installed" on iOS/Android home screens for native-like full-screen execution.

---

## PHASE 2: THE MULTI-MODEL ORCHESTRATION LAYER (KALOSM MoE)
Goal: Replace the monolithic LLM approach with a fast, task-specific routing pipeline to preserve VRAM and drastically reduce latency.

1. The Micro-Router (In-Memory Resident):
   - Model: `Qwen2.5-0.5B-Instruct-GGUF` (or similar sub-1B).
   - Role: Replaces regex/keyword `f62` parsing only when `IntentKind::Custom` is triggered. Classifies intent into: `code_gen`, `refactor`, `explain`, `fix`.
2. The Heavy Lifter (Dynamic Load):
   - Model: `Qwen2.5-Coder-7B-GGUF`.
   - Role: Generates code. Loaded into VRAM *only* when the router dispatches a `code_gen` or `refactor` task.
3. The Mechanic (Dynamic Load):
   - Model: `DeepSeek-Coder-1.3B-Base-GGUF` (or similar lightweight logic model).
   - Role: Reads `cargo check` `stderr` outputs and generates syntax fixes during the `f81` loop.
4. Memory Manager:
   - Implement an LRU cache system in the Kova engine that drops the Coder from Kalosm/VRAM to load the Mechanic when the `f81` fix loop initiates.

---

## PHASE 3: THE THICK SERVER & WEB-SOCKET BRIDGE (`kova serve`)
Goal: Bridge the gap between the mobile WASM client and the local multi-model pipeline.

1. Axum Framework Integration:
   - Upgrade `kova serve` to use `axum` and `tokio` for robust, asynchronous routing.
2. Endpoints:
   - `POST /api/intent`: Receives the `t0` Intent payload from the mobile client.
   - `GET /ws/stream`: Upgrades the connection to a WebSocket.
3. The `f81` Pipeline Bridge:
   - Modify `pipeline::f81()` to return a `tokio::sync::broadcast::Receiver` instead of a standard `mpsc`.
   - Pipe the stdout/stderr of the inference loop, `cargo check`, and `cargo clippy` directly into the WebSocket stream so the mobile UI updates character-by-character without HTTP polling.

---

## PHASE 4: SECURE MOBILE TUNNELING (CLOUDFLARE)
Goal: Safely expose the `kova serve` API and WASM client to the public internet without opening router ports.

1. Local Network Bind:
   - Run `kova serve` on `127.0.0.1:8080` (Serving the Axum API).
   - Serve the compiled WASM static files (`trunk` output) from `/` on the same Axum server.
2. Cloudflare `cloudflared` Daemon:
   - Register a dedicated subdomain (e.g., `kova.yourdomain.com`).
   - Run: `cloudflared tunnel route dns kova-tunnel kova.yourdomain.com`
   - Run: `cloudflared tunnel run kova-tunnel` pointing to `localhost:8080`.
3. Authentication (Crucial for Mobile):
   - Wrap the subdomain in Cloudflare Zero Trust (Access).
   - Require a One-Time Pin (OTP) sent to your email or GitHub OAuth to access the Kova UI on your phone. This keeps the engine strictly locked to you.

---

## PHASE 5: EXECUTION ORDER
1. ✅ Refactor `t0` and `f62` structures into a shared `kova-core` library crate so both the WASM client and the Server can use the exact same types.
2. ✅ Build the Axum server (`kova serve`) with mock WebSocket outputs to test the connection.
3. 🔄 Strip `kova gui`, add `trunk`, and successfully render the UI in a local browser. (kova-web crate created; trunk build pending)
4. Integrate Kalosm multi-model loading and wire `pipeline::f81` to the WebSocket.
5. Deploy the Cloudflare tunnel and authenticate from your mobile device.
