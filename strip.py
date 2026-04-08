import glob
import os
import re

directory = "/home/dancxjo/src/thing-os/abi/src/types"

for filename in glob.glob(os.path.join(directory, "*.rs")):
    with open(filename, "r") as f:
        content = f.read()
    
    # Remove `use crate::{Graphable, ThingId};` or similar
    content = re.sub(r'use crate::graphable::Graphable;\n', '', content)
    content = re.sub(r'use crate::{Graphable,?\s*[^}]*};\n', 'use crate::wire::ThingId;\n', content)
    content = re.sub(r'Graphable,\s*', '', content)
    
    with open(filename, "w") as f:
        f.write(content)
print("Stripped Graphable from types")
