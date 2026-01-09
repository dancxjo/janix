# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-08 22:39:27

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 319ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 4071ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 665ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 684ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 626ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 753ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 2046ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13754099073] [INFO] System booted
[13765460643] [INFO] boot: phys ranges=35 modules=0
[13770898977] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[13773299859] [INFO] Initializing Real Frame Allocator...
[13775188284] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[14515722342] [INFO] frame_alloc: total=516638 free=511720 used=4918
[14517650532] [INFO] Running frame_alloc sanity check...
[14521213212] [INFO] frame_alloc: sanity: single ok
[14525055732] [INFO] frame_alloc: sanity: contig(8) ok
[14525588781] [INFO] Testing paging subsystem...
[14535422583] [INFO] Switched to new address space
[14539643679] [INFO] Paging subsystem test passed
[14543681658] [INFO] System halted

```
</details>
