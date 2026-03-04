# Scripts

This folder contains local helper scripts used by Xenochat development and quality checks.

## Available scripts
- `check_no_npm.sh`: enforces pnpm-only policy across the repository.
- `check_todo_policy.sh`: blocks TODO entries without issue binding (`TODO(XENO-123):` or `TODO(#123):`).
- `security_audit.sh`: runs `cargo-audit`, `cargo-deny`, and `pnpm audit` security scans.
- `originality_audit.py`: computes project hash signatures for originality tracking.
- `sync_to_github_mirror.sh`: syncs source tree to mirror repository with privacy excludes.
