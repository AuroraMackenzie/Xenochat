# Security Baseline Checklist

This checklist is the release gate for Xenochat runtime security.

## Runtime Defaults
- API bind default is localhost (`127.0.0.1`).
- Wildcard CORS (`*`) is forbidden.
- Query-token authentication is forbidden.
- Bearer token is required when API keys are configured.
- Admin routes require dedicated admin bearer keys.
- Queue capacity must be non-zero.

## Secret Management
- `api.api_keys` and `api.admin_api_keys` must use encrypted format: `enc:v1:<nonce>:<ciphertext>`.
- Plaintext API keys are rejected at startup.
- Master key must be resolvable from `XENOCHAT_MASTER_KEY` or macOS Keychain fallback.
- Logs must pass redaction tests for token-like data.

## Build and Test Gate
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- `pnpm --dir frontend lint`
- `pnpm --dir frontend build`

## Security Scan Gate
- Rust advisory scan (`cargo-audit`)
- Rust dependency policy scan (`cargo-deny`)
- Frontend dependency audit (`pnpm audit --prod --audit-level moderate`)

Run all three scanners with:

```bash
scripts/security_audit.sh
```

## Release Policy
- A release candidate must not ship with unresolved scanner failures.
- Any temporary scanner allowlist must be documented with expiry date and owner.
- Threat model must be reviewed when auth, secrets, transport, or plugin runtime changes.
