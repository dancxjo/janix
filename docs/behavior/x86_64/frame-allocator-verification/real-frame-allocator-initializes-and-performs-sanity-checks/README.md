# ✅ Scenario: Real Frame Allocator initializes and performs sanity checks

> Last run: 2026-01-09 09:13:04

## Steps

| # | Step | Result | Duration | Artifacts |
|---|------|--------|----------|-----------|
| 1 | Given the machine is started | ✅ | 317ms | <a href="./01/after.png"><img src="./01/after.png" width="150" /></a> - [💾](./01/registers.txt) |
| 2 | Then the serial output should contain "Initializing Real Frame Allocator..." | ✅ | 4088ms | <a href="./02/after.png"><img src="./02/after.png" width="150" /></a> [📜](./02/serial.log) [💾](./02/registers.txt) |
| 3 | And the serial output should contain "frame_alloc: base=" | ✅ | 638ms | <a href="./03/after.png"><img src="./03/after.png" width="150" /></a> - [💾](./03/registers.txt) |
| 4 | And the serial output should contain "frame_alloc: total=" | ✅ | 640ms | <a href="./04/after.png"><img src="./04/after.png" width="150" /></a> [📜](./04/serial.log) [💾](./04/registers.txt) |
| 5 | And the serial output should contain "Running frame_alloc sanity check..." | ✅ | 652ms | <a href="./05/after.png"><img src="./05/after.png" width="150" /></a> - [💾](./05/registers.txt) |
| 6 | And the serial output should contain "frame_alloc: sanity: single ok" | ✅ | 625ms | <a href="./06/after.png"><img src="./06/after.png" width="150" /></a> - [💾](./06/registers.txt) |
| 7 | And the serial output should contain "System halted" | ✅ | 2047ms | - - - |

<details>
<summary>📜 Full Serial Log</summary>

```
[2J[01;01H[=3h[2J[01;01H[2J[01;01H[8;042;160t0[2J[01;01H[2J[01;01H[8;056;240t0[2J[01;01HBdsDxe: loading Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
BdsDxe: starting Boot0002 "UEFI QEMU DVD-ROM QM00005 " from PciRoot(0x0)/Pci(0x1F,0x2)/Sata(0x2,0xFFFF,0x0)
[2J[01;01H[01;01H[2J[01;01H[01;01H[12639379401] [INFO] System booted
[12652864950] [INFO] boot: phys ranges=35 modules=0
[12657835542] [INFO] BootHeap initialized. Range: 0xffffff8040000000 - 0xffffff8041000000
[12660199563] [INFO] Initializing Real Frame Allocator...
[12661944372] [INFO] frame_alloc: base=0x0 frames=516638 words=8073
[13494165927] [INFO] frame_alloc: total=516638 free=511667 used=4971
[13507268511] [INFO] Running frame_alloc sanity check...
[13510919136] [INFO] frame_alloc: sanity: single ok
[13517415120] [INFO] frame_alloc: sanity: contig(8) ok
[13520183259] [INFO] Testing paging subsystem...
[13529173152] [INFO] Switched to new address space
[13532027223] [INFO] Paging subsystem test passed
[13532500146] [INFO] Paging subsystem test passed
[13532935713] [INFO] Initializing Kernel Heap...
[13540216800] [INFO] kheap: grew by 64 pages (phys=0x3f000, virt=0xffffa00000000000)
[13545804030] [INFO] global_alloc: switched to kernel heap
[13546337871] [INFO] Running heap sanity check...
[13566139686] [INFO] kheap: sanity ok
[13569595512] [INFO] kheap: forcing growth...
[13573457106] [INFO] kheap: grew by 16 pages (phys=0x82000, virt=0xffffa00000040000)
[13599354483] [INFO] kheap: big allocation ok (len=307200)
[13613219334] [INFO] kheap: reserved=268435456 committed=327680
[13614565503] [INFO] System halted

```
</details>
