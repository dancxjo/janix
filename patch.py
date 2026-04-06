import sys

with open("userspace/display_virtio_gpu/src/main.rs", "r") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "if let Ok(n) = port_recv(drv_req_read, &mut buf) {" in line:
        lines.insert(i+1, "            if n > 0 {\n                info!(\"display_virtio_gpu: received {} bytes on drv_req_read\", n);\n            }\n")
        break

with open("userspace/display_virtio_gpu/src/main.rs", "w") as f:
    f.writelines(lines)
