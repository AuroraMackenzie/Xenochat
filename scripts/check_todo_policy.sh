#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

# TODO(XENO-123): keep this policy check aligned with repository conventions.
# TODO(#123): alternate issue binding format remains accepted.
INVALID=$(rg -n "TODO" "$ROOT_DIR/crates" "$ROOT_DIR/frontend/src" \
  | rg -v "TODO\((XENO-[0-9]+|#[0-9]+)\):" || true)

if [[ -n "$INVALID" ]]; then
  echo "Found TODO entries without required issue binding format:"
  echo "$INVALID"
  exit 1
fi

echo "TODO policy check passed."
