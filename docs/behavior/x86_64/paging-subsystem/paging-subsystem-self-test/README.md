# ✅ Scenario: Paging Subsystem Self-Test

> Last run: 2026-01-08 23:58:35

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 367ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Testing paging subsystem..." | ✅ | 5417ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "Switched to new address space" | ✅ | 2047ms | - - - |
| 4 | And the serial output should contain "Paging subsystem test passed" | ✅ | 1ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[15840122595] [INFO] System booted
[15866783922] [INFO] boot: phys ranges=35 modules=0
[15872127282] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[15878253765] [INFO] Initializing Real Frame Allocator...
[15880698933] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[16901112228] [INFO] frame_alloc: total=516638 free=511720 used=4918
[16904137602] [INFO] Running frame_alloc sanity check...
[16908402522] [INFO] frame_alloc: sanity: single ok
[16917190257] [INFO] frame_alloc: sanity: contig(8) ok
[16917784983] [INFO] Testing paging subsystem...
[16928466720] [INFO] Switched to new address space
[16933832157] [INFO] Paging subsystem test passed
[16935012732] [INFO] System halted

```
</details>
