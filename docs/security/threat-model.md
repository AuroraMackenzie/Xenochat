# Threat Model (STRIDE)

This document defines the production threat model baseline for Xenochat and maps each risk category to concrete controls in this repository.

## Scope
- API runtime (`xenochat-api`) and launcher (`xenochat-bin`)
- Config and secret handling (`xenochat-common`)
- Frontend dashboard transport assumptions
- Local deployment on macOS Apple Silicon, with optional public API exposure

## Security Objectives
- Keep API keys encrypted at rest and inaccessible in logs.
- Block common remote abuse paths by default.
- Preserve service availability under burst traffic.
- Maintain auditable security checks in CI and local workflows.

## Assets
- `L1` internal: non-sensitive runtime metadata, health counters.
- `L2` protected: user prompts and responses, adapter payload envelopes.
- `L3` secret: API keys, bearer tokens, master key material.

## STRIDE Analysis

### Spoofing
- Threat: unauthenticated client calls protected API routes.
- Controls:
  - Bearer token enforcement on non-health routes.
  - Separate admin bearer key gate for `/api/v1/admin/*` routes.
  - Query-token rejection to prevent URL token leakage.
  - Public bind without auth keys is rejected at config validation.

### Tampering
- Threat: config values modified to weaken CORS or auth protections.
- Controls:
  - Wildcard CORS is rejected by validation.
  - Plaintext API keys are rejected by validation.
  - Encrypted secret format (`enc:v1`) is required for stored keys.

### Repudiation
- Threat: actions cannot be reliably traced during incident response.
- Controls:
  - Structured audit event model in `xenochat-common::audit`.
  - CI-enforced quality and policy checks (`TODO`, pnpm-only, originality scan).
  - Security audit script output retained in CI logs.

### Information Disclosure
- Threat: secrets appear in config snapshots, logs, or process output.
- Controls:
  - AES-256-GCM encryption for persisted API keys.
  - Runtime decryption only with `XENOCHAT_MASTER_KEY`.
  - Log redaction helpers sanitize token-like fields.
  - Query-string token usage blocked.

### Denial of Service
- Threat: request flooding exhausts runtime resources.
- Controls:
  - In-memory request rate limiting in API middleware.
  - Bounded queue defaults in config.
  - Zero-capacity queue configuration is rejected.

### Elevation of Privilege
- Threat: attacker bypasses policy by exploiting default-open settings.
- Controls:
  - Default API bind is localhost (`127.0.0.1`).
  - Startup warning on non-local bind to highlight risk posture.
  - CORS allowlist-only mode (deny by default).

## Residual Risks and Next Actions
- No hardware-backed secret storage yet for all platforms.
- Current RBAC is route-level (standard vs admin keys) and does not yet include user/group policy graph.
- No distributed rate-limit backend for multi-instance deployment.
- Next milestones:
  - Add role-based route scopes with user identities and session management.
  - Add per-route adaptive throttling with persistent counters.
