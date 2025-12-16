import sys
import os
# import toml
# Fallback to manual parsing if toml module is missing not needed as we can use simple parsing
# wait, 'toml' module might be missing. I should use simple manual parsing as environment is minimal.

from PIL import Image, ImageDraw

def parse_map(toml_path):
    mapping = {}
    import re
    with open(toml_path, 'r') as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith('#'): continue
            # Match: Key = { svg = "path/val.svg" }
            if '=' in line and 'svg' in line:
                 key_part = line.split('=', 1)[0].strip()
                 # find svg = "..." using regex
                 m = re.search(r'svg\s*=\s*"([^"]+)"', line)
                 if m:
                     val = m.group(1)
                     mapping[key_part] = val
    return mapping

def gen_icon(kind, out_path):
    # Hash name to color
    h = hash(kind)
    r = (h & 0xFF)
    g = ((h >> 8) & 0xFF)
    b = ((h >> 16) & 0xFF)
    
    # Create 64x64 RGBA
    img = Image.new('RGBA', (64, 64), (0, 0, 0, 0))
    d = ImageDraw.Draw(img)
    
    # Draw rounded rect
    d.rounded_rectangle([4, 4, 60, 60], radius=16, fill=(r, g, b, 200), outline=(255,255,255,255), width=2)
    
    # Draw first letter if possible
    # We don't load font to avoid dependency, just simple element.
    # d.text((20, 20), kind[0], fill=(255,255,255,255)) 
    # PIL default font requires valid font logic, sometimes not available.
    
    # Save as 32-bit BMP
    img.save(out_path, "BMP")

def main():
    if len(sys.argv) < 3:
        print("Usage: gen_dummy_icons.py <config> <out_dir>")
        sys.exit(1)
        
    config = sys.argv[1]
    out_dir = sys.argv[2]
    
    print(f"Generating dummy icons from {config} to {out_dir}")
    
    mapping = parse_map(config)
    
    if not os.path.exists(out_dir):
        os.makedirs(out_dir)

    for kind, svg_path in mapping.items():
        fname = f"{kind}.bmp"
        out_path = os.path.join(out_dir, fname)
        # print(f"Generating {out_path} for {kind}")
        gen_icon(kind, out_path)

if __name__ == "__main__":
    main()
