# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-08 21:17:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 281ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 2943ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 8ms | - - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 8ms | - - [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 9ms | - - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity ok" | ✅ | 8ms | - - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 206ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[8681905254] [INFO] System booted
[8689027578] [INFO] boot: phys ranges=35 modules=0
[8689800801] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[8690962665] [INFO] Initializing Real Frame Allocator...
[8692071828] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[9738693723] [INFO] frame_alloc: total=516638 free=511735 used=4903
[9739485228] [INFO] Running frame_alloc sanity check...
[10173323127] [INFO] frame_alloc: sanity ok
[10173705432] [INFO] System halted

```
</details>
