# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-08 22:24:57

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 1326ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 5547ms | - [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 1263ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 35ms | - - [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 778ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 249ms | - - - |
| 7 | And the serial output should contain "System halted" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[20801577522] [INFO] System booted
[20822742501] [INFO] boot: phys ranges=35 modules=0
[20823897303] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[20825664321] [INFO] Initializing Real Frame Allocator...
[20837359686] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[22143088209] [INFO] frame_alloc: total=516638 free=511732 used=4906
[22149897858] [INFO] Running frame_alloc sanity check...
[22159925733] [INFO] frame_alloc: sanity: single ok
[22162688262] [INFO] frame_alloc: sanity: contig(8) ok
[22163293944] [INFO] System halted

```
</details>
