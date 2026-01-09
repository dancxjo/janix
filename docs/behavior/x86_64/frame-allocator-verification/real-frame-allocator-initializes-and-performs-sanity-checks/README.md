# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-08 22:26:09

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 571ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 3695ms | - [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 120ms | - - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 151ms | - [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 20ms | - - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 896ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - - |
| 7 | And the serial output should contain "System halted" | ✅ | 0ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[13167387981] [INFO] System booted
[13182035460] [INFO] boot: phys ranges=35 modules=0
[13183521780] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[13185291537] [INFO] Initializing Real Frame Allocator...
[13191193455] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[14187970401] [INFO] frame_alloc: total=516638 free=511732 used=4906
[14188682112] [INFO] Running frame_alloc sanity check...
[14191723755] [INFO] frame_alloc: sanity: single ok
[14196169284] [INFO] frame_alloc: sanity: contig(8) ok
[14196660753] [INFO] System halted

```
</details>
