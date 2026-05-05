#!/usr/bin/env bash
set -euo pipefail

VAULT_ROOT="${1:-/home/will/Documents/Obsidian Vault}"
QUARTZ_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CONTENT_DIR="$QUARTZ_ROOT/content"

mkdir -p "$CONTENT_DIR"

find "$CONTENT_DIR" -mindepth 1 -maxdepth 1 \
  ! -name 'index.md' \
  ! -name '.gitkeep' \
  ! -name 'album' \
  ! -name 'artist' \
  ! -name 'genre' \
  -exec rm -rf {} +

for folder in album artist genre; do
  mkdir -p "$CONTENT_DIR/$folder"
  if [ -d "$VAULT_ROOT/$folder" ]; then
    rsync -a --delete "$VAULT_ROOT/$folder"/ "$CONTENT_DIR/$folder"/
  else
    rm -rf "$CONTENT_DIR/$folder"
  fi
done

echo "Synced vault from '$VAULT_ROOT' into '$CONTENT_DIR'."
