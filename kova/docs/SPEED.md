# Blazing Speed — Optimization Guide

**Purpose:** Document each speed optimization, config flags, and baseline metrics for Kova.

**Date:** 2026-03-10

---

## Step 1: No Encryption (Localhost Baseline)

**Localhost = no TLS = fastest path.**

- When binding to `127.0.0.1`, Kova uses plain HTTP/WebSocket. No TLS overhead.
- Config: `KOVA_FAST_LOCALHOST=true` (default). Set to `false` only when adding TLS for remote access.
- Baseline: `curl -w '%{time_total}\n' -o /dev/null -s http://127.0.0.1:3002/api/status` — latency in seconds.

---

## Step 2: Kernel Bypass (io_uring)

**Status:** Deferred. Feature stub `io_uring` exists; no-op.

**Design doc (future migration):**
- Hyper has no native io_uring support (open issue since 2020).
- Options: (A) tokio-uring with `tokio-uring::net::TcpListener` + Axum; (B) monoio-transports; (C) glommio.
- Linux-only. Requires `tokio` with `io-uring` feature or runtime switch.
- When implementing: replace `tokio::net::TcpListener::bind(addr)` in serve.rs with io_uring variant when `io_uring` feature enabled.

---

## Step 3: Zero-Copy

**Status:** See implementation. Use `Arc<str>`/`Bytes` in inference and pipeline hot paths.

---

## Step 4: QUIC / HTTP/3

**Status:** Optional. `KOVA_QUIC=1` enables QUIC listener on separate port.

---

## Step 5: Hardware AES (AES-NI)

**TLS uses AES-NI when available.** rustls defaults prefer AES-GCM (AES-NI accelerated). ChaCha20 fallback for non-AES-NI.

---

## Step 6: Persistent Connections

**HTTP/2** and **keep-alive** enabled. WebSocket is persistent per client.

---

## Step 7: Compression

**Config:** `KOVA_COMPRESS_RESPONSES=true` (default off for localhost, on for remote). Reduces bytes on wire for large payloads.

---

## Step 8: Batching

**Config:** `KOVA_BATCH_TOKENS` — batch size for f76 token streaming (e.g. 4–8). Fewer packets, less overhead.

---

## Step 9: Local Inference

**Kova inference is local-only. No network hop.** Models loaded from `FileSource::Local(path)`.

---

## Step 10: Memory-Mapped I/O (mmap)

**Config:** `KOVA_MMAP_MODELS=true` (default). Kalosm/Candle GGUF loaders use mmap when available.

---

## Configuration Summary

| Flag | Purpose |
|------|---------|
| `KOVA_FAST_LOCALHOST` | Skip TLS on loopback (default: true) |
| `KOVA_QUIC` | Enable QUIC listener |
| `KOVA_COMPRESS_RESPONSES` | Enable response compression |
| `KOVA_BATCH_TOKENS` | Batch size for f76 |
| `KOVA_MMAP_MODELS` | Document mmap for models |
| `KOVA_MODEL_CACHE_SIZE` | Max models in memory (default: 2) |
| `KOVA_ROUTER_STRUCTURED` | Grammar-constrained Router output |
| `KOVA_CODE_GEN_STRUCTURED` | Experimental: structured code output |

---

## Baseline Metrics

Run `scripts/bench.sh` or `kova bench` for latency/throughput baseline. Used for regression.
