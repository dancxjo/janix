# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-08 22:21:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 312ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 3856ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 15ms | - - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 20ms | - - [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 18ms | - - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 21ms | - - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 210ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11277189825] [INFO] System booted
[11288347884] [INFO] boot: phys ranges=35 modules=0
[11289499485] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[11291305740] [INFO] Initializing Real Frame Allocator...
[11293057908] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[11976170613] [INFO] frame_alloc: total=516638 free=511732 used=4906
[11976971226] [INFO] Running frame_alloc sanity check...
[11979860310] [INFO] frame_alloc: sanity: single ok
[11982348675] [INFO] frame_alloc: sanity: contig(8) ok
[11982878655] [INFO] System halted

```
</details>
