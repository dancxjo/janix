# ✅ Scenario: Kernel successfully initializes allocators and performs early allocations

> Last run: 2026-01-08 22:20:16

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 430ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "boot: phys ranges=" | ✅ | 4075ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "System halted" | ✅ | 585ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12430445379] [INFO] System booted
[12440971389] [INFO] boot: phys ranges=35 modules=0
[12443906739] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12445516776] [INFO] Initializing Real Frame Allocator...
[12447058107] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[13140147273] [INFO] frame_alloc: total=516638 free=511732 used=4906
[13142044872] [INFO] Running frame_alloc sanity check...
[13144705068] [INFO] frame_alloc: sanity: single ok
[13146927090] [INFO] frame_alloc: sanity: contig(8) ok
[13147375989] [INFO] System halted

```
</details>
