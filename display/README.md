# Display Stack v0 (Bloom/Blossom/Drivers)

Bloom draws pixels into a shared Bytespace. Blossom brokers display requests without touching pixel memory. Display drivers map the Bytespace and present to hardware.

## Pipeline overview

```
+-------+    display protocol v0    +---------+    driver protocol v0    +------------------+    scanout
| Bloom | <-----------------------> | Blossom | <-----------------------> | display_* driver | ---------> device
+-------+    ports (req/resp)       +---------+    ports (req/resp)       +------------------+
   |                                                                                  |
   | maps compositor bytespace (write)                                                | maps compositor bytespace (read)
   +----------------------------------------------------------------------------------+
```

## Ownership rules

- Bloom never touches hardware and never claims devices.
- Blossom never maps bytespaces and never touches pixel memory.
- Display drivers are the only processes that claim devices or map scanout memory.

## Bytespace data model

- One compositor surface bytespace.
- Size = height * stride.
- Properties on bytespace node: width, height, stride, format.
- Backed by mem.Range (created by bytespace_create).
- Mapped by Bloom (write) and by the active driver (read).

## Protocols

- Display Protocol v0 (Bloom <-> Blossom)
  - HELLO, INFO_REQ/INFO_RESP, BUFFER_REQ/BUFFER_RESP, PRESENT, ACK/ERR
- Display Driver Protocol v0 (Blossom <-> Driver)
  - REGISTER, BIND, PRESENT, ACK/ERR

See `abi/src/display_protocol.rs` and `abi/src/display_driver_protocol.rs` for wire formats.

## Sprout wiring (v0)

- Creates compositor bytespace sized to the scanout.
- Creates two ports for Bloom <-> Blossom and two ports for Blossom <-> driver.
- Spawns:
  - `/blossom` (broker)
  - `/bloom` (demo compositor)
  - One active driver (bootfb or virtio)
- Sets graph nodes/edges for svc/app/driver and the bytespace chain.
