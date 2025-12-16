#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT="$ROOT/target/third_party/plataro"
ZIP="$OUT/plataro.zip"
SRC="$OUT/src"

mkdir -p "$OUT"

if [[ ! -f "$ZIP" ]]; then
  echo "Downloading Plataro icons..."
  curl -L -o "$ZIP" "https://github.com/tsujan/Plataro/archive/master.zip"
fi

rm -rf "$SRC"
mkdir -p "$SRC"
unzip -q "$ZIP" -d "$SRC"

TOP="$(find "$SRC" -maxdepth 1 -type d -name 'Plataro-*' | head -n 1)"
rm -rf "$SRC/plataro"
mv "$TOP" "$SRC/plataro"

# Copy the license text from upstream (name may differ; prefer COPYING)
if [[ -f "$SRC/plataro/COPYING" ]]; then
  cp -f "$SRC/plataro/COPYING" "$OUT/LICENSE_CC_BY_NC_SA_4.0.txt"
fi
