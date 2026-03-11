<!-- Copyright (c) 2026 The Cochran Block. All rights reserved. -->
# Intent Engine — Research Foundation

**Target:** DARPA-contract-worthy intent engine for Rust coding  
**Method:** Bleeding-edge research synthesis  
**Date:** 2026-02-28

---

## 1. DARPA Intent-Based Computing

### IDAS: Intent-Defined Adaptive Software
*Source: [DARPA IDAS](https://www.darpa.mil/research/programs/intent-defined-adaptive-software)*

**Core insight:** Separate *intent* (what the engineer wants) from *implementation* (concrete code). Concretization—making design decisions early—creates technical debt when requirements change. IDAS captures intent and constraints separately, enabling automated adaptation.

**Key objectives:**
- Capture, learn, or annotate software intent and constraints *separate* from concrete implementation
- Reduce human effort for adapting software to new requirements, platforms, resources
- Verify adapted software meets requirements
- Integrate into Agile workflows

**Application to Intent Engine:** Our intent layer captures user goals ("make this faster", "add tests", "fix warnings") as structured intent. The compute layer maps intent to parallelized actions without requiring the user to specify *how*.

### CODORD: Human-AI Communication for Deontic Reasoning
*Source: [DARPA CODORD](https://www.darpa.mil/research/programs/codord)*

**Core insight:** Convert natural language into logical languages for AI reasoning. Enables reasoning about obligations, permissions, prohibitions with high assurance.

**Application:** Intent engine uses deontic-style constraints—e.g., "must not break tests", "may use GPU if available", "must complete within N seconds".

---

## 2. Rust Compute Offload — Bleeding Edge

### CubeCL
*Source: [CubeCL crates.io](https://crates.io/crates/cubecl), [tracel-ai/cubecl](https://github.com/tracel-ai/cubecl)*

- Multi-platform GPU compute: CUDA, HIP (AMD), WGPU
- Zero-cost abstractions in Rust
- SIMD via `Line<T>` type—automatic vectorization
- Thread block / grid abstractions for parallel kernels

**Application:** Optional GPU backend for heavy compute (e.g., batch intent resolution, semantic analysis). CPU-first with GPU offload when available.

### Burn Framework
*Source: [burn-ndarray](https://docs.rs/burn-ndarray), [burn-cubecl](https://crates.io/crates/burn-candle)*

- Swappable backends: ndarray (CPU), CubeCL (GPU), Candle, LibTorch
- burn-ndarray: CPU with optional BLAS (OpenBLAS, NetLib)
- burn-cubecl: CUDA, ROCm, Metal, Vulkan, WebGPU

**Application:** If we add local LLM inference for intent parsing, Burn provides Rust-native inference across CPU/GPU.

### Rust GPU / rust-gpu
*Source: [rust-gpu.github.io](https://rust-gpu.github.io/blog)*

- Write GPU kernels in Rust
- NVVM IR for NVIDIA; SPIR-V for Vulkan; Metal, DX12, WebGPU
- Single codebase, multiple targets

---

## 3. Local Inference — Low Latency

### vLLM
*Source: [vLLM docs](https://docs.vllm.ai)*

- PagedAttention for efficient KV cache
- Continuous batching (23x throughput, lower p50 latency)
- CUDA graph execution
- Quantization: GPTQ, AWQ, INT4, INT8, FP8

**Application:** If intent engine uses LLM for natural-language→intent, vLLM or similar patterns for low-latency local inference.

### NanoLLM / LocalAI
*Source: [dusty-nv/NanoLLM](https://github.com/dusty-nv/nanollm), LocalAI*

- Lightweight local inference
- HuggingFace-like APIs
- CPU-only options (no GPU required)

---

## 4. High-Speed Transits (Laptop)

| Transit | Use Case | Rust Crate |
|---------|----------|------------|
| CPU cores | Parallel compilation, test runs | rayon, std::thread |
| SIMD | Vectorized ops | std::simd, packed_simd |
| GPU | Heavy compute, inference | burn-cubecl, candle |
| Memory | Zero-copy, mmap | memmap2, bytemuck |
| Disk I/O | Async, parallel reads | tokio, async-std |
| NIC | (Future) distributed | — |

---

## 5. Synthesis: Intent Engine Architecture

1. **Intent Layer** — Parse natural language / structured input → Intent AST
2. **Constraint Layer** — Deontic constraints (must/may/must-not)
3. **Plan Layer** — Map intent to action DAG; identify parallelizable units
4. **Compute Layer** — Offload to rayon (CPU), optional GPU, SIMD
5. **Rust Integration** — cargo, rust-analyzer, sccache, mold

**Principle:** Maximize utilization of all available compute (cores, SIMD, GPU) while keeping intent human-readable and constraints verifiable.
