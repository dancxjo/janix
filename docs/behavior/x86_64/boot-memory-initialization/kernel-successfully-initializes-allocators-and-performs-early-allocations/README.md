# ✅ Scenario: Kernel successfully initializes allocators and performs early allocations

> Last run: 2026-01-08 23:58:35

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 443ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "boot: phys ranges=" | ✅ | 4564ms | - [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "System halted" | ✅ | 2047ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[16970139714] [INFO] System booted
[16986562659] [INFO] boot: phys ranges=35 modules=0
[16992230244] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[16994241495] [INFO] Initializing Real Frame Allocator...
[16996175196] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[19122519735] [INFO] frame_alloc: total=516638 free=511720 used=4918
[19123614081] [INFO] Running frame_alloc sanity check...
[19128014796] [INFO] frame_alloc: sanity: single ok
[19132381422] [INFO] frame_alloc: sanity: contig(8) ok
[19132964532] [INFO] Testing paging subsystem...
[19143519516] [INFO] Switched to new address space
[19144065534] [INFO] Paging subsystem test passed
[19144603137] [INFO] System halted

```
</details>
