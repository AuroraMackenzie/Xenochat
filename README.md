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
2. `cargo run -p xenochat-bin -- preview`
3. `cargo run -p xenochat-bin -- serve`
4. `cargo run -p xenochat-cli -- gpu-info`
5. `cargo run -p xenochat-cli -- check-config configs/xenochat.toml`
6. `export XENOCHAT_MASTER_KEY='replace-with-strong-secret'`
7. `cargo run -p xenochat-cli -- seal-key 'my-api-key'`
8. `pnpm --dir frontend install`
9. `pnpm --dir frontend dev`
10. `scripts/security_audit.sh`

## Security baseline
- API host default: `127.0.0.1`
- CORS default: deny all origins
- Public bind requires non-empty encrypted `api_keys`
- Queue model: bounded queue with optional drop policy
- Logs: secret redaction helpers in `crates/common/src/security.rs`
- Query-token auth is rejected (`token`/`access_token` in URL).
- Protected routes require `Authorization: Bearer <token>`.
- Encrypted API keys use `enc:v1:<nonce>:<ciphertext>` and are decrypted by `XENOCHAT_MASTER_KEY`.
- Startup prints an explicit warning when API bind host is non-local.

## Security docs
- Threat model: `docs/security/threat-model.md`
- Release gate checklist: `docs/security/security-baseline.md`

## License
AGPL-3.0-only.
