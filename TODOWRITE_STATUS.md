# Xenochat TodoWrite Status

## Execution Board
| Block | Scope | Status | Last Updated |
|---|---|---|---|
| B1 | Backend core expansion (Rust) | COMPLETED | 2026-03-04 |
| B2 | Configs, docs, compliance, empty folder completion | COMPLETED | 2026-03-04 |
| B3 | Frontend (pnpm, original UI, Crescent icon) | COMPLETED | 2026-03-04 |
| B4 | Quality gates (CI, lint rules, anti-npm checks) | COMPLETED | 2026-03-04 |
| B5 | Sync to GitHub working copy and commit staging | COMPLETED | 2026-03-04 |
| B6 | Full reference read pass + secure API runtime uplift | COMPLETED | 2026-03-04 |
| B7 | Security governance closure (threat model, audits, startup risk warning) | COMPLETED | 2026-03-04 |
| B8 | macOS Keychain master-key fallback integration | COMPLETED | 2026-03-04 |
| B9 | RBAC baseline (admin route gate + dashboard access tier update) | COMPLETED | 2026-03-04 |
| B10 | Zero-warning stabilization (pnpm build-script policy + CI parity validation) | COMPLETED | 2026-03-04 |

## Notes
- The project was initialized from the blueprint and predecessor summary documents.
- Crate directories in `crates/` were renamed to remove the `xenochat-` path prefix as requested.
- Added core modules for persona, plugin lifecycle, safety guard, tool registry, keyword triggers, and adapter import contracts.
- Fixed a real log-sanitization infinite-loop bug and verified `xenochat-common` tests pass.
- Filled previously empty folders with architecture, compliance, reference docs, scripts, tests, benchmark guides, and baseline config templates.
- Created a pnpm-based frontend, integrated `Crescent.png`, and validated `pnpm build` and `pnpm lint`.
- Added CI pipeline with Rust + pnpm checks, TODO policy, and originality hash audit scripts.
- Verified `cargo fmt --check`, `cargo clippy -D warnings`, `cargo test --workspace`, `pnpm lint`, and `pnpm build` all pass.
- Synced project to `/Users/ycy/Desktop/open-resources-programs/GitHub/Myself/Xenochat`, created commits, and pushed `main` to `origin`.
- Completed a full read pass for reference repositories under `AI-Programs/MaiMai` by file-level content scanning (1613 files) and critical source extraction.
- Removed blueprint/summary files from the mirror repository and updated sync rules to keep those local-only.
- Upgraded `xenochat-api` from contract-only state to a runnable Axum runtime with strict CORS allowlist enforcement, bearer-only protected routes, query-token rejection, and in-memory rate limiting.
- Added encrypted secret primitives with `enc:v1` format enforcement, startup rejection of plaintext API keys, and CLI helpers for sealing/opening keys with `XENOCHAT_MASTER_KEY`.
- Added security governance deliverables: `docs/security/threat-model.md`, `docs/security/security-baseline.md`, `scripts/security_audit.sh`, `deny.toml`, and CI integration for `cargo-audit` / `cargo-deny` / `pnpm audit`.
- Added explicit startup warning when API bind host is not local-only so public exposure risk is visible at launch time.
- Added optional macOS Keychain master-key fallback (`security find-generic-password`) with environment-priority resolution in `xenochat-common::secrets`.
- Wired Keychain-aware master key resolution into API bootstrap and CLI (`check-config`, `seal-key`, `open-key`, and `master-key-check`).
- Added split operator/admin encrypted key configuration (`api.api_keys`, `api.admin_api_keys`) with strict plaintext rejection and decryption checks in config validation and CLI.
- Added admin-only API route family (`/api/v1/admin/*`) with dedicated bearer enforcement and regression tests for non-admin/admin access behavior.
- Updated dashboard UI with explicit Access Tiers panel and refreshed docs to reflect RBAC route-level policy.
- Eliminated pnpm install warnings by moving `onlyBuiltDependencies` policy to workspace root (`pnpm-workspace.yaml`) and verifying `esbuild` postinstall is explicitly approved.
- Re-ran full quality and security gates after cleanup to confirm zero errors and zero warnings across Rust, frontend, policy scripts, and vulnerability scans.
