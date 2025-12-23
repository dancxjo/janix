#!/usr/bin/env bash
set -e

# Setup paths
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
OUT_DIR="$ROOT/assets/icons/plataro64"
IMG0="/home/dancxjo/.gemini/antigravity/brain/3bc233c3-0068-4964-aa62-a56936846098/uploaded_image_0_1765867872395.png"
IMG1="/home/dancxjo/.gemini/antigravity/brain/3bc233c3-0068-4964-aa62-a56936846098/uploaded_image_1_1765867872395.png"
IMG2="/home/dancxjo/.gemini/antigravity/brain/3bc233c3-0068-4964-aa62-a56936846098/uploaded_image_0_1765868759845.png"

echo "Processing custom icons..."
python3 "$ROOT/tools/icons/process_custom_icons.py" "$OUT_DIR" "$IMG0" "$IMG1" "$IMG2"

echo "Done."
