import sys
with open("userspace/display_virtio_gpu/src/main.rs", "r") as f:
    lines = f.readlines()

for i, line in enumerate(lines):
    if "let _ = port_send(port, &out_buf[..total]);" in line:
        lines[i] = "            let status = port_send(port, &out_buf[..total]);\n            info!(\"display_virtio_gpu: sent msg_type={} size={} to {} status={:?}\", msg_type, total, port, status);\n"
        break

with open("userspace/display_virtio_gpu/src/main.rs", "w") as f:
    f.writelines(lines)
