#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo "Usage: $0 <path-to-rust-checkout>" >&2
  exit 1
fi

RUST_DIR="$1"
SCRIPT_DIR="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" && pwd)"
PATCH_DIR="$(cd -- "$SCRIPT_DIR/../patches" && pwd)"

if [[ ! -d "$RUST_DIR/.git" ]]; then
  echo "error: '$RUST_DIR' is not a git checkout" >&2
  exit 1
fi

for patch in "$PATCH_DIR"/*.patch; do
  echo "Applying $(basename "$patch")"
  git -C "$RUST_DIR" am "$patch"
done

echo "Done. Next: cd '$RUST_DIR' && ./x.py build library/std --target /path/to/thing-os/targets/x86_64-unknown-thingos.json"
