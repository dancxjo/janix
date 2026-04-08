import glob
import os
import re

directory = "/home/dancxjo/src/thing-os/abi/src/types"

for filename in glob.glob(os.path.join(directory, "*.rs")):
    with open(filename, "r") as f:
        content = f.read()
    
    if "use crate::" not in content and "pub struct" in content:
        content = "use crate::{SymbolId, BlobId, ThingId};\n" + content
        with open(filename, "w") as f:
            f.write(content)
print("Added imports safely")
