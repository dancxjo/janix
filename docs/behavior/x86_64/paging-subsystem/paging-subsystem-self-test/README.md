# ✅ Scenario: Paging Subsystem Self-Test

> Last run: 2026-01-08 22:41:21

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 323ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Testing paging subsystem..." | ✅ | 5499ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "Switched to new address space" | ✅ | 2046ms | - - - |
| 4 | And the serial output should contain "Paging subsystem test passed" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[16673765460] [INFO] System booted
[16687709379] [INFO] boot: phys ranges=35 modules=0
[16689509661] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[16691502465] [INFO] Initializing Real Frame Allocator...
[16696595124] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[17401026126] [INFO] frame_alloc: total=516638 free=511720 used=4918
[17403861618] [INFO] Running frame_alloc sanity check...
[17410756836] [INFO] frame_alloc: sanity: single ok
[17415410859] [INFO] frame_alloc: sanity: contig(8) ok
[17417425641] [INFO] Testing paging subsystem...
[17426789160] [INFO] Switched to new address space
[17430112689] [INFO] Paging subsystem test passed
[17430589968] [INFO] System halted

```
</details>
