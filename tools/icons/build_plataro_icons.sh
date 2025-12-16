#!/usr/bin/env bash
set -e

# Setup paths
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
MAP="$ROOT/assets/icons/plataro_map.toml"
PLATARO_SRC="$ROOT/target/third_party/plataro/src/plataro"
OUT_DIR="$ROOT/rootfs/share/icons/plataro64"
FETCH_SCRIPT="$ROOT/tools/icons/fetch_plataro.sh"

# Ensure tools
if ! command -v rsvg-convert &> /dev/null; then
    echo "rsvg-convert not found. Checking for Python PIL..."
    if python3 -c "import PIL" 2>/dev/null; then
        echo "Falling back to Python/PIL generation."
        python3 tools/icons/gen_dummy_icons.py "$MAP" "$OUT_DIR"
        exit $?
    else
        echo "Error: rsvg-convert not found and Python PIL not available."
        echo "Please install librsvg2-bin or python3-pil."
        exit 1
    fi
fi
if ! command -v magick &> /dev/null; then
    echo "Error: magick (ImageMagick) not found. Please install imagemagick."
    exit 1
fi

# Fetch icons
"$FETCH_SCRIPT"

mkdir -p "$OUT_DIR"

# Generate license/attribution
cp "$ROOT/target/third_party/plataro/LICENSE_CC_BY_NC_SA_4.0.txt" "$OUT_DIR/"
echo "Icons derived from Plataro (tsujan), licensed CC BY-NC-SA 4.0." > "$OUT_DIR/ATTRIBUTION.txt"
echo "Source: https://github.com/tsujan/Plataro" >> "$OUT_DIR/ATTRIBUTION.txt"
echo "Converted to 64x64 BMP; colors/sizing may be modified." >> "$OUT_DIR/ATTRIBUTION.txt"

# Parse TOML and convert
# We look for lines like: Key = { svg = "value" }
grep -E '^[A-Za-z0-9_]+ = \{ svg = "[^"]+" \}' "$MAP" | while read -r line; do
    KIND=$(echo "$line" | cut -d' ' -f1)
    SVG_REL=$(echo "$line" | sed -E 's/.*svg = "([^"]+)".*/\1/')
    SVG_PATH="$PLATARO_SRC/$SVG_REL"
    BMP_OUT="$OUT_DIR/$KIND.bmp"

    if [[ ! -f "$SVG_PATH" ]]; then
        # Fallback search
        FOUND=$(find "$PLATARO_SRC" -name "$(basename "$SVG_REL")" | head -n 1)
        if [[ -f "$FOUND" ]]; then
            SVG_PATH="$FOUND"
        else
            echo "Warning: Icon for $KIND not found at $SVG_PATH"
            continue
        fi
    fi

    echo "Converting $KIND..."
    # SVG -> PNG (preserve alpha)
    rsvg-convert -w 64 -h 64 -a "$SVG_PATH" -o "tmp_$KIND.png"
    
    # PNG -> BMP (32-bit ARGB/BGRA)
    magick "tmp_$KIND.png" -background none -alpha on -define bmp:format=bmp3 "BMP3:$BMP_OUT"
    
    rm "tmp_$KIND.png"
done

echo "Icon build complete."
