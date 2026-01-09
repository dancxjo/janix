# ✅ Scenario: Kernel successfully initializes allocators and performs early allocations

> Last run: 2026-01-08 22:40:51

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 472ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "boot: phys ranges=" | ✅ | 4201ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "System halted" | ✅ | 2060ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13734421569] [INFO] System booted
[13759651719] [INFO] boot: phys ranges=35 modules=0
[13764475593] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[13779614508] [INFO] Initializing Real Frame Allocator...
[13784095809] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[14680947171] [INFO] frame_alloc: total=516638 free=511720 used=4918
[14685555027] [INFO] Running frame_alloc sanity check...
[14691879906] [INFO] frame_alloc: sanity: single ok
[14704966881] [INFO] frame_alloc: sanity: contig(8) ok
[14708184315] [INFO] Testing paging subsystem...
[14725113150] [INFO] Switched to new address space
[14731785585] [INFO] Paging subsystem test passed
[14732998335] [INFO] System halted

```
</details>
