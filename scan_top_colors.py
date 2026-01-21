from PIL import Image
import sys
from collections import Counter

img_path = sys.argv[1]
img = Image.open(img_path).convert('RGB')
counter = Counter(img.getdata())
print("Top 20 colors in whole image:")
for color, count in counter.most_common(20):
    print(f"  {color}: {count}")
