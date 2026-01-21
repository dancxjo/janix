from PIL import Image
import os
import glob

files = sorted(glob.glob("docs/behavior/x86_64/system-boot-and-ui-bring-up/boot-produces-logs-graphics-and-a-live-desktop/07/clock_verify_*.png"))

for f in files:
    img = Image.open(f).convert('RGB')
    width, height = img.size
    reds = 0
    for y in range(height):
        for x in range(width):
            r, g, b = img.getpixel((x, y))
            # exclude top-right landmark (e.g. x > width-50 and y < 50)
            if x > width - 50 and y < 50:
                continue
            if r > 100 and g < 100 and b < 100:
                reds += 1
    print(f"{f}: {reds} red pixels (excluding top-right)")
