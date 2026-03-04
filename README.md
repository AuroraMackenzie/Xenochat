# Xenochat

Xenochat is a Rust-first multi-platform AI bot framework designed for high safety defaults, modular adapters, and Apple Silicon Metal acceleration.

## Core principles
- Original implementation with independent architecture and assets.
- Rust backend as the single source of runtime logic.
- pnpm-only frontend workflow.
- Secure defaults: localhost bind, strict CORS, no query-token auth.
- Adapter contracts shared across all supported platforms.

## Repository layout
- `crates/`: Rust workspace crates (core, api, adapter, protocol, gpu, cli, bin, platform adapters).
- `frontend/`: React + TypeScript + Vite dashboard project managed by pnpm.
- `configs/`: baseline config templates.
- `docs/`: architecture, compliance, and reference documents.
- `scripts/`: helper automation and quality scripts.
- `tests/`: integration test plans and test notes.
- `benchmarks/`: benchmark plans and baseline records.

## Quick start
1. `cargo check --workspace`
2. `cargo run -p xenochat-bin`
3. `cargo run -p xenochat-cli -- gpu-info`
4. `pnpm --dir frontend install`
5. `pnpm --dir frontend dev`

## Security baseline
- API host default: `127.0.0.1`
- CORS default: deny all origins
- Public bind requires non-empty `api_keys`
- Queue model: bounded queue with optional drop policy
- Logs: secret redaction helpers in `crates/common/src/security.rs`
