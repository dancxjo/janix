# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-08 23:58:35

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 362ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 5625ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 1283ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 771ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 869ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 793ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 2050ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[17657590566] [INFO] System booted
[17675624934] [INFO] boot: phys ranges=35 modules=0
[17677585200] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[17688847044] [INFO] Initializing Real Frame Allocator...
[17692348542] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[18611822460] [INFO] frame_alloc: total=516638 free=511720 used=4918
[18613502391] [INFO] Running frame_alloc sanity check...
[18618115758] [INFO] frame_alloc: sanity: single ok
[18623615406] [INFO] frame_alloc: sanity: contig(8) ok
[18626544321] [INFO] Testing paging subsystem...
[18639539259] [INFO] Switched to new address space
[18640701717] [INFO] Paging subsystem test passed
[18641263212] [INFO] System halted

```
</details>
