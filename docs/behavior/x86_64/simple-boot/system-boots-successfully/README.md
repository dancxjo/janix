# ✅ Scenario: System boots successfully

> Last run: 2026-01-08 22:21:43

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 389ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then I should see a message in the serial output that says "System booted" | ✅ | 3952ms | - [📜](./02/serial.log) - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12356129214] [INFO] System booted
[12368064555] [INFO] boot: phys ranges=35 modules=0
[12371342346] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12373127646] [INFO] Initializing Real Frame Allocator...
[12374779065] [INFO] frame_alloc: base=0x0 frames=516638 words=8073

```
</details>
