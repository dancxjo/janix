# ✅ Scenario: System halts

> Last run: 2026-01-08 22:39:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 394ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> [📜](./01/serial.log) [💾](./01/registers.txt) |
| 2 | Then I should see that the machine has halted | ✅ | 5691ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12994162500] [INFO] System booted
[13007419161] [INFO] boot: phys ranges=35 modules=0
[13009076850] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[13013479677] [INFO] Initializing Real Frame Allocator...
[13015191189] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[13652352021] [INFO] frame_alloc: total=516638 free=511720 used=4918
[13654745841] [INFO] Running frame_alloc sanity check...
[13658443821] [INFO] frame_alloc: sanity: single ok
[13664457411] [INFO] frame_alloc: sanity: contig(8) ok
[13666690455] [INFO] Testing paging subsystem...
[13676350182] [INFO] Switched to new address space
[13679676450] [INFO] Paging subsystem test passed
[13680152376] [INFO] System halted

```
</details>
