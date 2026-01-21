from PIL import Image
import sys
from collections import Counter

img_path = sys.argv[1]
img = Image.open(img_path).convert('RGB')
width, height = img.size

# The clock digits in the screenshot I saw were at bottom right.
# Let's Scan 1000, 600 area.
colors = []
for y in range(600, 720):
    for x in range(800, 1280):
        colors.append(img.getpixel((x, y)))

counter = Counter(colors)
print("Top 100 colors in bottom-right region:")
for color, count in counter.most_common(100):
    if color[0] > 50: # some red component
        print(f"  {color}: {count}")

