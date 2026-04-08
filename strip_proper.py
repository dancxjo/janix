import glob
import os
import re

directory = "/home/dancxjo/src/thing-os/abi/src/types"

for filename in glob.glob(os.path.join(directory, "*.rs")):
    with open(filename, "r") as f:
        content = f.read()
    
    # Strip Graphable trait references
    content = re.sub(r'#\[derive\(Graphable,\s*', '#[derive(', content)
    content = re.sub(r'use crate::\{[^}]*Graphable[^}]*\};\n', '', content)
    
    with open(filename, "w") as f:
        f.write(content)
print("Stripped Graphable cleanly")
