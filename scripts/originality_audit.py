#!/usr/bin/env python3
"""Lightweight originality and hygiene audit for Xenochat."""

from __future__ import annotations

import hashlib
import os
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
REFERENCE_ROOT = Path("/Users/ycy/Desktop/open-resources-programs/GitHub/AI-Programs/MaiMai")

IGNORES = {
    ".git",
    "target",
    ".Xenochat",
    "node_modules",
}


def iter_files(root: Path):
    for base, dirs, files in os.walk(root):
        dirs[:] = [d for d in dirs if d not in IGNORES]
        for name in files:
            if name == ".DS_Store":
                continue
            path = Path(base) / name
            yield path


def file_hash(path: Path) -> str:
    hasher = hashlib.sha256()
    with path.open("rb") as fh:
        while True:
            chunk = fh.read(65536)
            if not chunk:
                break
            hasher.update(chunk)
    return hasher.hexdigest()


def main() -> int:
    if not REFERENCE_ROOT.exists():
        print("Reference root not found; skipping cross-project hash check.")
        return 0

    ref_hashes = {}
    for path in iter_files(REFERENCE_ROOT):
        ref_hashes[file_hash(path)] = str(path)

    clashes = []
    for path in iter_files(ROOT):
        digest = file_hash(path)
        if digest in ref_hashes:
            clashes.append((str(path), ref_hashes[digest]))

    if clashes:
        print("Detected hash collisions with reference project files:")
        for ours, theirs in clashes:
            print(f"- {ours} == {theirs}")
        return 1

    print("Originality hash audit passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
