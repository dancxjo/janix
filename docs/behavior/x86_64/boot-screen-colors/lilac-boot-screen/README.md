# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-08 22:25:03

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 381ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 5387ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 2881ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[16359452538] [INFO] System booted
[16373783415] [INFO] boot: phys ranges=35 modules=0
[16378017216] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[16383081198] [INFO] Initializing Real Frame Allocator...
[16385425914] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[17356288719] [INFO] frame_alloc: total=516638 free=511732 used=4906
[17357806620] [INFO] Running frame_alloc sanity check...
[17361575583] [INFO] frame_alloc: sanity: single ok
[17366960028] [INFO] frame_alloc: sanity: contig(8) ok
[17367900627] [INFO] System halted

```
</details>
