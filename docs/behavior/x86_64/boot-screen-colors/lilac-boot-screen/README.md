# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-08 22:39:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 464ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 4334ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 2776ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12342384483] [INFO] System booted
[12355293159] [INFO] boot: phys ranges=35 modules=0
[12358369815] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12360124260] [INFO] Initializing Real Frame Allocator...
[12364998393] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[13071670557] [INFO] frame_alloc: total=516638 free=511720 used=4918
[13072763385] [INFO] Running frame_alloc sanity check...
[13077038535] [INFO] frame_alloc: sanity: single ok
[13085570421] [INFO] frame_alloc: sanity: contig(8) ok
[13086422943] [INFO] Testing paging subsystem...
[13096028055] [INFO] Switched to new address space
[13099497246] [INFO] Paging subsystem test passed
[13099956045] [INFO] System halted

```
</details>
