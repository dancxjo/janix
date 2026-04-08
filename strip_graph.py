import os
import re

def remove_ring_calls(filepath):
    if not os.path.exists(filepath): return
    with open(filepath, 'r') as f:
        content = f.read()

    # Remove the generic crate::sched::ring calls, which might span multiple lines!
    content = re.sub(r'[ \t]*//[^\n]*(?:graph|ring)[^\n]*\n', '', content, flags=re.IGNORECASE)
    content = re.sub(r'[ \t]*crate::sched::ring::[a-zA-Z0-9_:<>]+\([^;]*\);\n?', '', content)

    # Clean up some specific mod.rs initialization
    content = re.sub(r'[ \t]*// Initialize per-CPU event rings\n[ \t]*let cpu_total =.+?crate::kinfo!\("  Initialized {} event ring\(s\)", cpu_total\);\n', '', content, flags=re.DOTALL)
    
    with open(filepath, 'w') as f:
        f.write(content)

for fn in ["kernel/src/sched/mod.rs", "kernel/src/sched/spawn.rs", "kernel/src/sched/sleep.rs", "kernel/src/sched/blocking.rs"]:
    remove_ring_calls(fn)

def remove_mod_decls(filepath):
    if not os.path.exists(filepath): return
    with open(filepath, 'r') as f:
        content = f.read()
    content = re.sub(r'pub mod graph;\n', '', content)
    content = re.sub(r'pub mod graph_queue;\n', '', content)
    content = re.sub(r'pub mod graphify;\n', '', content)
    content = re.sub(r'pub mod flusher;\n', '', content)
    content = re.sub(r'pub mod events;\n', '', content)
    content = re.sub(r'pub\(crate\) mod ring;\n', '', content)
    content = re.sub(r'crate::task::graph_queue::init\(\);\n', '', content)
    
    # remove init_graph_workers entirely recursively block
    content = re.sub(r'pub fn init_graph_workers[^{]*\{.*?(?=\n\n|\n[a-z])', '', content, flags=re.DOTALL)

    with open(filepath, 'w') as f:
        f.write(content)

remove_mod_decls("kernel/src/task/mod.rs")
remove_mod_decls("kernel/src/sched/mod.rs")

# Remove the files
files_to_remove = [
    "kernel/src/task/graph.rs",
    "kernel/src/task/graph_queue.rs",
    "kernel/src/task/graphify.rs",
    "kernel/src/task/flusher.rs",
    "kernel/src/sched/events.rs",
    "kernel/src/sched/ring.rs"
]
for f in files_to_remove:
    if os.path.exists(f):
        os.remove(f)

print("Done stripping graph remnants from kernel tasks!")
