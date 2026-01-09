# ✅ Scenario: Lilac Boot Screen

> Last run: 2026-01-08 22:40:51

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 432ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 4341ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Lilac" | ✅ | 2966ms | - [📜](./03/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[14301359292] [INFO] System booted
[14315205795] [INFO] boot: phys ranges=35 modules=0
[14320381779] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[14322590370] [INFO] Initializing Real Frame Allocator...
[14324221824] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[15065701497] [INFO] frame_alloc: total=516638 free=511720 used=4918
[15067223325] [INFO] Running frame_alloc sanity check...
[15070896489] [INFO] frame_alloc: sanity: single ok
[15077154774] [INFO] frame_alloc: sanity: contig(8) ok
[15081154506] [INFO] Testing paging subsystem...
[15089978772] [INFO] Switched to new address space
[15092647713] [INFO] Paging subsystem test passed
[15093455751] [INFO] System halted

```
</details>
