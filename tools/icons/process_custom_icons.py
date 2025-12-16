#!/usr/bin/env python3
import sys
import os
import struct

try:
    from PIL import Image
except ImportError:
    print("Error: PIL (Pillow) not found. Please install python3-pil or pip install Pillow")
    sys.exit(1)

# Configuration
BG_COLOR = (0, 0, 0)
TOLERANCE = 30
OUTPUT_SIZE = (64, 64)

# Global list of names for ALL input images combined
# Order: Image 0, then Image 1, then Image 2
KINDS_ORDER = [
    # Image 0 (GUI)
    "Window", "MenuBar", "MenuItem", "Button", "Dropdown",
    "TextView", "TimeSource", "Ps2Keyboard", "Ps2Mouse", "IoPortRegion",
    "BlockRing", "CharRing", "Spawn", "Storage", "Channel",
    
    # Image 1 (Hardware/Kernel)
    "Kind", "FramePool", "PciDevice", "Framebuffer",
    "KernelInfo", "BootStats", "FrameHeap", "PciBus",
    "Ramdisk", "IRQDesc", "TimerWheel", "Owned",

    # Image 2 (Graph/Nodes)
    "Link", "Node", "Thing", "GraphRoot"
]

def is_bg(pixel, bg, tol):
    return (abs(pixel[0] - bg[0]) <= tol and
            abs(pixel[1] - bg[1]) <= tol and
            abs(pixel[2] - bg[2]) <= tol)

def rect_union(r1, r2):
    return (
        min(r1[0], r2[0]),
        min(r1[1], r2[1]),
        max(r1[2], r2[2]),
        max(r1[3], r2[3])
    )

def rect_intersects_or_close(r1, r2, dist=10):
    # Check if r1 expanded by dist intersects r2
    return not (r1[2] + dist < r2[0] - dist or 
                r1[0] - dist > r2[2] + dist or 
                r1[3] + dist < r2[1] - dist or 
                r1[1] - dist > r2[3] + dist)

def merge_rects(rects):
    if not rects: return []
    # Simple iterative merge
    merged = True
    while merged:
        merged = False
        new_rects = []
        skip = set()
        for i in range(len(rects)):
            if i in skip: continue
            r_curr = rects[i]
            for j in range(i+1, len(rects)):
                if j in skip: continue
                if rect_intersects_or_close(r_curr, rects[j]):
                    r_curr = rect_union(r_curr, rects[j])
                    skip.add(j)
                    merged = True
            new_rects.append(r_curr)
        rects = new_rects
    return rects

def find_blobs(img):
    width, height = img.size
    pixels = img.load()
    
    bg = pixels[0, 0]
    visited = set()
    raw_rects = []
    
    for y in range(height):
        for x in range(width):
            if (x, y) in visited: continue
            
            p = pixels[x, y]
            is_tr = (len(p) == 4 and p[3] < 128)
            is_bg_px = is_bg(p[:3], bg[:3], TOLERANCE)
            
            if is_tr or is_bg_px:
                visited.add((x, y))
                continue
            
            min_x, max_x = x, x
            min_y, max_y = y, y
            stack = [(x, y)]
            visited.add((x, y))
            
            while stack:
                cx, cy = stack.pop()
                min_x = min(min_x, cx)
                max_x = max(max_x, cx)
                min_y = min(min_y, cy)
                max_y = max(max_y, cy)
                
                for nx, ny in [(cx+1, cy), (cx-1, cy), (cx, cy+1), (cx, cy-1)]:
                    if 0 <= nx < width and 0 <= ny < height and (nx, ny) not in visited:
                        np = pixels[nx, ny]
                        nis_tr = (len(np) == 4 and np[3] < 128)
                        nis_bg = is_bg(np[:3], bg[:3], TOLERANCE)
                        
                        if not (nis_tr or nis_bg):
                            visited.add((nx, ny))
                            stack.append((nx, ny))
                            
            # Initial Filter
            w, h = max_x - min_x + 1, max_y - min_y + 1
            # Very small blobs are noise, but parts of icons can be small.
            # We keep reasonable ones, then merge.
            if w > 5 and h > 5:
                raw_rects.append( (min_x, min_y, max_x, max_y) )

    # Merge rects
    final_rects = merge_rects(raw_rects)
    
    # Final Size Filter
    valid_rects = []
    for r in final_rects:
         w, h = r[2] - r[0] + 1, r[3] - r[1] + 1
         if w > 20 and h > 20: # Slightly relaxed
             valid_rects.append(r)
             
    # Sort
    valid_rects.sort(key=lambda r: r[1])
    rows = []
    if valid_rects:
        current_row = [valid_rects[0]]
        current_y = valid_rects[0][1]
        row_h_max = valid_rects[0][3] - valid_rects[0][1]
        
        for r in valid_rects[1:]:
            if r[1] < current_y + row_h_max * 0.5:
                current_row.append(r)
                row_h_max = max(row_h_max, r[3] - r[1])
            else:
                current_row.sort(key=lambda r: r[0])
                rows.extend(current_row)
                current_row = [r]
                current_y = r[1]
                row_h_max = r[3] - r[1]
        current_row.sort(key=lambda r: r[0])
        rows.extend(current_row)
        
    final_blobs = []
    for (x1, y1, x2, y2) in rows:
        final_blobs.append(img.crop((x1, y1, x2+1, y2+1)))
        
    return final_blobs

def save_as_bmp32(img, path):
    w, h = img.size
    ratio = min(OUTPUT_SIZE[0]/w, OUTPUT_SIZE[1]/h)
    new_w = int(w * ratio)
    new_h = int(h * ratio)
    img = img.resize((new_w, new_h), Image.Resampling.LANCZOS)
    
    final = Image.new('RGBA', OUTPUT_SIZE, (0, 0, 0, 0))
    offset_x = (OUTPUT_SIZE[0] - new_w) // 2
    offset_y = (OUTPUT_SIZE[1] - new_h) // 2
    final.paste(img, (offset_x, offset_y))
    final.save(path, format='BMP')

def main():
    if len(sys.argv) < 3:
        print("Usage: process.py <out_dir> <img1> [img2 ...]")
        sys.exit(1)
        
    out_dir = sys.argv[1]
    inputs = sys.argv[2:]
    
    os.makedirs(out_dir, exist_ok=True)
    all_icons = []
    
    for ipath in inputs:
        print(f"Processing {ipath}...")
        img = Image.open(ipath).convert('RGBA')
        blobs = find_blobs(img)
        print(f"  Found {len(blobs)} icons.")
        all_icons.extend(blobs)
        
    print(f"Total icons extracted: {len(all_icons)}")
    print(f"Total names expected: {len(KINDS_ORDER)}")
    
    for i, icon in enumerate(all_icons):
        if i < len(KINDS_ORDER):
            name = KINDS_ORDER[i]
            path = os.path.join(out_dir, f"{name}.bmp")
            save_as_bmp32(icon, path)
            print(f"Saved {path}")
        else:
            print(f"Warning: Icon {i} has no name (Generic Unknown)")

if __name__ == "__main__":
    main()
