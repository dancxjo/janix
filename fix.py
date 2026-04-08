import glob
import os
import re

directory = "/home/dancxjo/src/thing-os/abi/src/types"

for filename in glob.glob(os.path.join(directory, "*.rs")):
    with open(filename, "r") as f:
        content = f.read()
    
    # Remove all top-level crate:: wires since we injected it badly.
    content = re.sub(r'^use crate[^\n]+\n', '', content, flags=re.MULTILINE)

    # Insert a blanket import at the top
    content = "use crate::{SymbolId, KindId, PredicateId, BlobId, ThingId};\n" + content

    with open(filename, "w") as f:
        f.write(content)
print("Fixed imports")
