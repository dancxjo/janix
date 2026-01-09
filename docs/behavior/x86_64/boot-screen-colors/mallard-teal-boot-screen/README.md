# ✅ Scenario: Mallard Teal Boot Screen

> Last run: 2026-01-08 21:08:14

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 297ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | When I wait for the system to boot | ✅ | 3732ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | Then the screen should be filled with "Mallard Teal" | ✅ | 1141ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[10841607447] [INFO] System booted
[10852073430] [INFO] boot: phys ranges=35 modules=0
[10853285487] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[10854968124] [INFO] Allocating Box...
[10860865686] [INFO] Boxed value: 42
[10861514070] [INFO] Allocating Vec...
[10868303094] [INFO] Vec length: 100
[10869739815] [INFO] System halted

```
</details>
