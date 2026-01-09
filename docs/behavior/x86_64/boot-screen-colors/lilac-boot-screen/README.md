# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-08 22:31:12

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 328ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 3774ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 2702ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11752128927] [INFO] System booted
[11766342390] [INFO] boot: phys ranges=35 modules=0
[11772070860] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[11776881666] [INFO] Initializing Real Frame Allocator...
[11778442203] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[12537104811] [INFO] frame_alloc: total=516638 free=511732 used=4906
[12541600170] [INFO] Running frame_alloc sanity check...
[12544555518] [INFO] frame_alloc: sanity: single ok
[12547462059] [INFO] frame_alloc: sanity: contig(8) ok
[12553002858] [INFO] System halted

```
</details>
