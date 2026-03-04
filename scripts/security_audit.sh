#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

require_cmd() {
  local cmd="$1"
  local hint="$2"
  if ! command -v "$cmd" >/dev/null 2>&1; then
    echo "Missing required tool: $cmd"
    echo "Install hint: $hint"
    exit 1
  fi
}

require_cmd cargo "https://doc.rust-lang.org/cargo/getting-started/installation.html"
require_cmd pnpm "https://pnpm.io/installation"
require_cmd cargo-audit "cargo install --locked cargo-audit"
require_cmd cargo-deny "cargo install --locked cargo-deny"

cd "$ROOT_DIR"

echo "[security] running cargo-audit"
cargo audit

echo "[security] running cargo-deny check advisories"
cargo deny check advisories

echo "[security] running pnpm production audit"
pnpm --dir frontend audit --prod --audit-level moderate

echo "[security] all security audits passed"
