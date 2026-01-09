# ✅ Scenario: Basic Paging Mechanism

> Last run: 2026-01-09 00:00:57

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 396ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Paging subsystem test passed" | ✅ | 7205ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[18044524410] [INFO] System booted
[18069236229] [INFO] boot: phys ranges=35 modules=0
[18071441685] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[18073765842] [INFO] Initializing Real Frame Allocator...
[18076231701] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[18945619341] [INFO] frame_alloc: total=516638 free=511720 used=4918
[18954034704] [INFO] Running frame_alloc sanity check...
[18960102117] [INFO] frame_alloc: sanity: single ok
[18965787423] [INFO] frame_alloc: sanity: contig(8) ok
[18971532855] [INFO] Testing paging subsystem...
[18983243400] [INFO] Switched to new address space
[18985318341] [INFO] Paging subsystem test passed
[18985916862] [INFO] System halted

```
</details>
