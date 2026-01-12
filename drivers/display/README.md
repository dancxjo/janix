# Display Drivers (v0)

Display drivers are leaf processes that own presentation. They claim devices, map scanout memory, and map compositor bytespaces read-only.

## Required behavior

- Use Driver Protocol v0.
- Send REGISTER to Blossom on startup.
- Receive BIND with the compositor bytespace id and metadata.
- Map the bytespace via SYS_ROOT_BYTESPACE_MAP.
- On PRESENT, copy or scanout the provided regions.

## Device access

- Only drivers claim devices or perform MMIO/IOPORT/DMA.
- Blossom and Bloom must not touch devices.

## Minimal skeleton checklist

- Parse driver port handles from the spawn arg.
- REGISTER -> wait for BIND.
- Map bytespace on BIND.
- ACK on PRESENT even if the implementation is a stub.

## Existing drivers

- display_bootfb: claims dev.display.Framebuffer and blits compositor -> scanout.
- display_virtio_gpu: claims dev.display.Gpu and acknowledges binds/presents.
- display_ramfb: software target for testing (copies into a RAM bytespace).
