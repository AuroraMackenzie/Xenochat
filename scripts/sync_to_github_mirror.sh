#!/usr/bin/env bash
set -euo pipefail

SOURCE_DIR="/Users/ycy/Desktop/open-resources-programs/My-program/Xenochat"
MIRROR_DIR="/Users/ycy/Desktop/open-resources-programs/GitHub/Myself/Xenochat"

mkdir -p "$MIRROR_DIR"
rsync -a --delete \
  --exclude '.git' \
  --exclude 'target' \
  --exclude '.Xenochat' \
  --exclude 'node_modules' \
  --exclude 'frontend/dist' \
  --exclude '.DS_Store' \
  "$SOURCE_DIR/" "$MIRROR_DIR/"

echo "Synced Xenochat to mirror: $MIRROR_DIR"
