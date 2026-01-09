# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-08 22:21:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 316ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 3841ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 1270ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11257137210] [INFO] System booted
[11267764893] [INFO] boot: phys ranges=35 modules=0
[11268618900] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[11269952364] [INFO] Initializing Real Frame Allocator...
[11271483564] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[11870082015] [INFO] frame_alloc: total=516638 free=511732 used=4906
[11870877645] [INFO] Running frame_alloc sanity check...
[11873774781] [INFO] frame_alloc: sanity: single ok
[11876235921] [INFO] frame_alloc: sanity: contig(8) ok
[11876743461] [INFO] System halted

```
</details>
