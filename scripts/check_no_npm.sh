#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"

if find "$ROOT_DIR" -name package-lock.json -o -name yarn.lock | grep -q .; then
  echo "Detected forbidden npm/yarn lock files. Use pnpm only."
  exit 1
fi

if rg -n "(^|[[:space:]])npm([[:space:]]|$)" "$ROOT_DIR" \
  --glob '!Xenochat项目蓝图.txt' \
  --glob '!前身项目总结.txt' \
  --glob '!TODOWRITE_STATUS.md' \
  --glob '!pnpm-lock.yaml' \
  --glob '!scripts/check_no_npm.sh' >/dev/null; then
  echo "Detected npm command usage in project files. Replace with pnpm."
  exit 1
fi

echo "pnpm-only check passed."
