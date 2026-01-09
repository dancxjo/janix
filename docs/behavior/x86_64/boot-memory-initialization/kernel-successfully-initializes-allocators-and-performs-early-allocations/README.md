# ✅ Scenario: Kernel successfully initializes allocators and performs early allocations

> Last run: 2026-01-08 22:21:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 371ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "boot: phys ranges=" | ✅ | 3982ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "System halted" | ✅ | 208ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11839718220] [INFO] System booted
[11852557167] [INFO] boot: phys ranges=35 modules=0
[11853654615] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[11855663094] [INFO] Initializing Real Frame Allocator...
[11857330023] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[12732955026] [INFO] frame_alloc: total=516638 free=511732 used=4906
[12734239914] [INFO] Running frame_alloc sanity check...
[12737477247] [INFO] frame_alloc: sanity: single ok
[12746548056] [INFO] frame_alloc: sanity: contig(8) ok
[12748408101] [INFO] System halted

```
</details>
