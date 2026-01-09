# ✅ Scenario: System boots successfully

> Last run: 2026-01-09 10:51:25

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | When I turn on the machine | ✅ | 384ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then I should see a message in the serial output that says "System booted" | ✅ | 8304ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[25280036265] [INFO] System booted
[25316170044] [INFO] boot: phys ranges=35 modules=0
[25319040021] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[25322053449] [INFO] Initializing Real Frame Allocator...
[25324517988] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[25325406084] [INFO] Allocating bitmap of 8073 words...
[25340319840] [INFO] Bitmap allocated at 0xffffff8040000000
[25359624840] [INFO] Zeroing bitmap...
[25370653803] [INFO] Bitmap zeroed.

```
</details>
