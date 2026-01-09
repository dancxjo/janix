# ✅ Scenario: Kernel successfully initializes allocators and performs early allocations

> Last run: 2026-01-08 22:39:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 318ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "boot: phys ranges=" | ✅ | 4855ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "System halted" | ✅ | 2050ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[15268432179] [INFO] System booted
[15282618021] [INFO] boot: phys ranges=35 modules=0
[15287900034] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[15290096085] [INFO] Initializing Real Frame Allocator...
[15291931083] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[16305275646] [INFO] frame_alloc: total=516638 free=511720 used=4918
[16311222279] [INFO] Running frame_alloc sanity check...
[16318590519] [INFO] frame_alloc: sanity: single ok
[16326196359] [INFO] frame_alloc: sanity: contig(8) ok
[16330642944] [INFO] Testing paging subsystem...
[16340001777] [INFO] Switched to new address space
[16341934422] [INFO] Paging subsystem test passed
[16342394838] [INFO] System halted

```
</details>
