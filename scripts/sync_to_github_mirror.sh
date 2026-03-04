#!/usr/bin/env bash
set -euo pipefail

SOURCE_DIR="/Users/ycy/Desktop/open-resources-programs/My-program/Xenochat"
MIRROR_DIR="/Users/ycy/Desktop/open-resources-programs/GitHub/Myself/Xenochat"

mkdir -p "$MIRROR_DIR"
rsync -a --delete \
  --filter='P .git/' \
  --exclude '.git/' \
  --exclude 'target' \
  --exclude '.Xenochat' \
  --exclude 'node_modules' \
  --exclude 'frontend/dist' \
  --exclude 'Xenochat项目蓝图.txt' \
  --exclude '前身项目总结.txt' \
  --exclude '.DS_Store' \
  "$SOURCE_DIR/" "$MIRROR_DIR/"

rm -f "$MIRROR_DIR/Xenochat项目蓝图.txt" "$MIRROR_DIR/前身项目总结.txt"

echo "Synced Xenochat to mirror: $MIRROR_DIR"
