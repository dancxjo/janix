# ✅ Scenario: Kernel successfully initializes allocators and performs early allocations

> Last run: 2026-01-08 21:09:42

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 337ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "boot: phys ranges=" | ✅ | 3987ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "Boxed value: 42" | ✅ | 10ms | - - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "Vec length: 100" | ✅ | 515ms | - - - |
| 5 | And the serial output should contain "System halted" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[11698469310] [INFO] System booted
[11709208170] [INFO] boot: phys ranges=35 modules=0
[11710415838] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[11712097815] [INFO] Allocating Box...
[11718270696] [INFO] Boxed value: 42
[11718932082] [INFO] Allocating Vec...
[11725961148] [INFO] Vec length: 100
[11727422685] [INFO] System halted

```
</details>
