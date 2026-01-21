from PIL import Image
import sys
from collections import Counter

img_path = sys.argv[1]
img = Image.open(img_path).convert('RGB')
width, height = img.size
print(f"Dimensions: {width}x{height}")

# Center of the expected clock window region
# Window is at bottom right. For 1280x720:
# width.saturating_sub(220) = 1060
# height.saturating_sub(105) = 615
cx, cy = 1060, 615
print(f"Scanning region around ({cx}, {cy})")

colors = []
for y in range(cy-50, cy+50):
    for x in range(cx-100, cx+100):
        if 0 <= x < width and 0 <= y < height:
            colors.append(img.getpixel((x, y)))

counter = Counter(colors)
print("Top 10 colors in region:")
for color, count in counter.most_common(10):
    print(f"  {color}: {count}")

